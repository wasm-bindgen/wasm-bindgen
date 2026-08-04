use crate::descriptor::{Descriptor, Function};
use crate::wasm_conventions::get_function_table_entry;
use crate::wit::{AdapterType, ClosureDtor, Instruction, InstructionBuilder};
use crate::wit::{InstructionData, StackChange};
use anyhow::{bail, format_err, Error};
use walrus::{ExportId, ValType};
use wasm_bindgen_shared::identifier::to_valid_ident;

fn closure_word_descriptor(memory64: bool) -> Descriptor {
    if memory64 {
        Descriptor::I64AsF64
    } else {
        Descriptor::I32
    }
}

impl InstructionBuilder<'_, '_> {
    /// Processes one more `Descriptor` as an argument to a JS function that
    /// Wasm is calling.
    ///
    /// This will internally skip `Unit` and otherwise build up the `bindings`
    /// map and ensure that it's correctly mapped from Wasm to JS.
    pub fn outgoing(&mut self, arg: &Descriptor) -> Result<(), Error> {
        if let Descriptor::Unit = arg {
            return Ok(());
        }
        // Similar rationale to `incoming.rs` around these sanity checks.
        let input_before = self.input.len();
        let output_before = self.output.len();
        self._outgoing(arg)?;

        assert!(input_before < self.input.len());
        if let Descriptor::Result(arg) = arg {
            if let Descriptor::Unit = &**arg {
                assert_eq!(output_before, self.output.len());
                return Ok(());
            }
        }
        assert_eq!(output_before + 1, self.output.len());
        Ok(())
    }

    fn _outgoing(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Boolean => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::BoolFromI32,
                    &[AdapterType::Bool],
                );
            }
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::ExternrefLoadOwned {
                        table_and_drop: None,
                    },
                    &[AdapterType::Externref],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::ExternrefLoadOwned {
                        table_and_drop: None,
                    },
                    &[AdapterType::NamedExternref(name.clone())],
                );
            }
            Descriptor::I8 => self.outgoing_i32(AdapterType::S8),
            Descriptor::U8 => self.outgoing_i32(AdapterType::U8),
            Descriptor::I16 => self.outgoing_i32(AdapterType::S16),
            Descriptor::U16 => self.outgoing_i32(AdapterType::U16),
            Descriptor::I32 => self.outgoing_i32(AdapterType::S32),
            Descriptor::U32 => self.outgoing_i32(AdapterType::U32),
            Descriptor::I64 => self.outgoing_i64(AdapterType::I64),
            Descriptor::U64 => self.outgoing_i64(AdapterType::U64),
            Descriptor::I64AsF64 | Descriptor::U64AsF64 => {
                self.outgoing_f64();
            }
            Descriptor::RawPointer => {
                if self.cx.memory64() {
                    self.outgoing_f64();
                } else {
                    self.outgoing_i32(AdapterType::U32);
                }
            }
            Descriptor::I128 => {
                self.instruction(
                    &[AdapterType::I64, AdapterType::I64],
                    Instruction::WasmToInt128 { signed: true },
                    &[AdapterType::S128],
                );
            }
            Descriptor::U128 => {
                self.instruction(
                    &[AdapterType::I64, AdapterType::I64],
                    Instruction::WasmToInt128 { signed: false },
                    &[AdapterType::U128],
                );
            }
            Descriptor::F32 => {
                self.get(AdapterType::F32);
                self.output.push(AdapterType::F32);
            }
            Descriptor::F64 => {
                self.get(AdapterType::F64);
                self.output.push(AdapterType::F64);
            }
            Descriptor::Enum { name, .. } => self.outgoing_i32(AdapterType::Enum(name.clone())),
            Descriptor::StringEnum { name, .. } => self.outgoing_string_enum(name),
            Descriptor::DynamicUnion { name, .. } => self.outgoing_dynamic_union(name)?,

            Descriptor::Char => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::StringFromChar,
                    &[AdapterType::String],
                );
            }

            Descriptor::RustStruct(class) => {
                let ptr_ty = if self.cx.memory64() {
                    AdapterType::F64
                } else {
                    self.ptr_ty()
                };
                self.instruction(
                    &[ptr_ty],
                    Instruction::RustFromI32 {
                        class: class.to_string(),
                    },
                    &[AdapterType::Struct(class.clone())],
                );
            }
            Descriptor::Ref(d) => self.outgoing_ref(false, d)?,
            Descriptor::RefMut(d) => self.outgoing_ref(true, d)?,

            Descriptor::CachedString => self.cached_string(true)?,

            Descriptor::String => {
                // fetch the ptr/length ...
                let ptr_ty = self.outgoing_internal_word_ty();
                self.get(ptr_ty.clone());
                self.get(ptr_ty);

                // ... then defer a call to `free` to happen later
                let free = self.cx.free()?;
                self.instructions.push(InstructionData {
                    instr: Instruction::DeferFree { free, align: 1 },
                    stack_change: StackChange::Modified {
                        popped: 2,
                        pushed: 2,
                    },
                });

                // ... and then convert it to a string type
                self.instructions.push(InstructionData {
                    instr: Instruction::MemoryToString(self.cx.memory()?),
                    stack_change: StackChange::Modified {
                        popped: 2,
                        pushed: 1,
                    },
                });
                self.output.push(AdapterType::String);
            }

            Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported argument type for calling JS function from Rust {arg:?}"
                    )
                })?;
                let mem = self.cx.memory()?;
                let free = self.cx.free()?;
                let ptr_ty = self.outgoing_internal_word_ty();
                self.instruction(
                    &[ptr_ty.clone(), ptr_ty],
                    Instruction::VectorLoad {
                        kind: kind.clone(),
                        mem,
                        free,
                    },
                    &[AdapterType::Vector(kind)],
                );
            }

            Descriptor::Option(d) => self.outgoing_option(d)?,
            Descriptor::Result(d) => self.outgoing_result(d)?,

            Descriptor::Function(descriptor) => {
                // By-value &dyn Fn(...) (immutable)
                self.outgoing_function(false, descriptor, None)?;
            }

            Descriptor::Slice(_) => {
                bail!("unsupported argument type for calling JS function from Rust: {arg:?}")
            }

            // nothing to do
            Descriptor::Unit => {}

            // Largely synthetic and can't show up
            Descriptor::ClampedU8 => unreachable!(),

            Descriptor::NonNull => {
                if self.cx.memory64() {
                    self.get(AdapterType::F64);
                    self.output.push(AdapterType::NonNull);
                } else {
                    self.outgoing_i32(AdapterType::NonNull);
                }
            }

            Descriptor::Closure(d) => {
                self.outgoing_function(d.mutable, &d.function, Some(d.owned))?
            }
        }
        Ok(())
    }

    fn outgoing_ref(&mut self, mutable: bool, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::TableGet,
                    &[AdapterType::Externref],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::TableGet,
                    &[AdapterType::NamedExternref(name.clone())],
                );
            }
            Descriptor::CachedString => self.cached_string(false)?,

            Descriptor::String => {
                let ptr_ty = self.outgoing_internal_word_ty();
                self.instruction(
                    &[ptr_ty.clone(), ptr_ty],
                    Instruction::MemoryToString(self.cx.memory()?),
                    &[AdapterType::String],
                );
            }
            Descriptor::Slice(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported argument type for calling JS function from Rust {arg:?}"
                    )
                })?;
                let mem = self.cx.memory()?;
                let ptr_ty = self.outgoing_internal_word_ty();
                self.instruction(
                    &[ptr_ty.clone(), ptr_ty],
                    Instruction::View {
                        kind: kind.clone(),
                        mem,
                    },
                    &[AdapterType::Vector(kind)],
                );
            }

            // `slice_to_array` argument: the macro rewrites a user-facing
            // `&[T]` into a `&Vec<T>` ABI call, which describes as
            // `Ref(Vector(T))`. The Rust side cloned the slice contents
            // into a freshly-allocated buffer JS owns and must free. The
            // wire format matches `Vec<T>` (transferred ownership), but
            // the JS-visible type is a plain `Array` rather than a typed
            // array — emit `VectorLoadAsArray` for primitive kinds.
            //
            // This arm is currently produced exclusively by the
            // `slice_to_array` codegen (`&Box<[T]>` has no `IntoWasmAbi`
            // impl, and `&Vec<T>` is not exposed as a user-writable
            // argument type).
            Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported argument type for calling JS function from Rust {arg:?}"
                    )
                })?;
                let mem = self.cx.memory()?;
                let free = self.cx.free()?;
                let ptr_ty = self.outgoing_internal_word_ty();
                self.instruction(
                    &[ptr_ty.clone(), ptr_ty],
                    Instruction::VectorLoadAsArray {
                        kind: kind.clone(),
                        mem,
                        free,
                    },
                    &[AdapterType::Vector(kind)],
                );
            }

            Descriptor::Function(descriptor) => {
                self.outgoing_function(mutable, descriptor, None)?;
            }

            // &mut dyn FnMut(...) emits RefMut(Function(...)) to signal that
            // a reentrancy guard is needed in the JS wrapper.
            Descriptor::RefMut(inner) => match inner.as_ref() {
                Descriptor::Function(descriptor) => {
                    self.outgoing_function(true, descriptor, None)?;
                }
                _ => bail!(
                    "unsupported reference argument type for calling JS function from Rust: {arg:?}"
                ),
            },

            // `&T` where `T` is a scalar. The Rust side passes the value by copy
            // (`impl<T: ScalarIntoWasmAbi> IntoWasmAbi for &T`), so the wire is
            // identical to passing `T` by value — JS just receives the primitive.
            // Only the shared-ref form participates; `&mut primitive` has no such
            // ABI.
            //
            // The accepted set lives in `is_scalar_by_shared_ref` below and must
            // match the `scalar_into_wasm_abi!` list in `src/convert/impls.rs`.
            _ if !mutable && is_scalar_by_shared_ref(arg) => {
                self.outgoing(arg)?;
            }

            // Reaching here means `&T: IntoWasmAbi` held on the Rust side for a
            // `T` that has no by-reference wire representation. `ScalarIntoWasmAbi`
            // is meant to keep that set in lockstep with the arms above, so this
            // is either a type that opted into `ScalarIntoWasmAbi` without really
            // being scalar, or the two lists have drifted. Say what *is*
            // supported rather than only dumping the internal descriptor.
            _ => bail!(
                "unsupported type behind a reference when passing a value to JS: {arg:?}. \
                 Only scalars, `JsValue`, imported JS types, strings, slices and \
                 `&dyn Fn`/`&mut dyn FnMut` closures can cross the boundary by \
                 reference — pass anything else by value"
            ),
        }
        Ok(())
    }

    // The function table never changes right now, so we can statically
    // look up the desired function.
    fn export_table_element(&mut self, idx: u32) -> ExportId {
        let module = &mut *self.cx.module;
        let func_id = get_function_table_entry(module, idx).unwrap();
        if let Some(export) = module
            .exports
            .iter()
            .find(|e| matches!(e.item, walrus::ExportItem::Function(id) if id == func_id))
        {
            return export.id();
        }
        let name = match &module.funcs.get(func_id).name {
            Some(name) => to_valid_ident(name),
            None => format!("__wasm_bindgen_func_elem_{}", func_id.index()),
        };
        module.exports.add(&name, func_id)
    }

    fn outgoing_function(
        &mut self,
        mutable: bool,
        descriptor: &Function,
        owned_closure: Option<bool>,
    ) -> Result<(), Error> {
        let mut descriptor = descriptor.clone();
        // Synthesize the a/b arguments that aren't present in the
        // signature from wasm-bindgen but are present in the Wasm file.
        // On wasm64 these use the same number ABI as the rest of the
        // pointer-sized wasm-bindgen surface.
        let nargs = descriptor.arguments.len();
        let ptr_descriptor = closure_word_descriptor(self.cx.memory64());
        descriptor.arguments.insert(0, ptr_descriptor.clone());
        descriptor.arguments.insert(0, ptr_descriptor);
        let shim = self.export_table_element(descriptor.shim_idx);
        let dtor = match owned_closure {
            None => ClosureDtor::Immediate,
            Some(false) => ClosureDtor::Borrowed,
            Some(true) => ClosureDtor::OwnClosure,
        };
        let adapter = self.cx.export_adapter(shim, descriptor)?;
        let ptr_ty = self.ptr_ty();
        self.instruction(
            &[ptr_ty.clone(), ptr_ty],
            Instruction::Closure {
                adapter,
                nargs,
                mutable,
                dtor,
            },
            &[AdapterType::Function],
        );
        Ok(())
    }

    fn outgoing_option(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Externref => {
                // This is set to `undefined` in the `None` case and otherwise
                // is the valid owned index.
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::ExternrefLoadOwned {
                        table_and_drop: None,
                    },
                    &[AdapterType::Externref.option()],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::ExternrefLoadOwned {
                        table_and_drop: None,
                    },
                    &[AdapterType::NamedExternref(name.clone()).option()],
                );
            }
            Descriptor::DynamicUnion {
                name,
                variant_types: _,
            } => {
                // Dynamic unions share the externref ABI; reuse the externref
                // option lifting.
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::ExternrefLoadOwned {
                        table_and_drop: None,
                    },
                    &[AdapterType::DynamicUnion(name.clone()).option()],
                );
            }
            Descriptor::I8 => self.out_option_sentinel32(AdapterType::S8),
            Descriptor::U8 => self.out_option_sentinel32(AdapterType::U8),
            Descriptor::I16 => self.out_option_sentinel32(AdapterType::S16),
            Descriptor::U16 => self.out_option_sentinel32(AdapterType::U16),
            Descriptor::I32 => self.out_option_sentinel64(AdapterType::S32),
            Descriptor::U32 => self.out_option_sentinel64(AdapterType::U32),
            Descriptor::I64AsF64 => self.out_option_sentinel64(AdapterType::I64),
            Descriptor::U64AsF64 => self.out_option_sentinel64(AdapterType::U64),
            Descriptor::I64 => self.option_native(true, ValType::I64),
            Descriptor::U64 => self.option_native(false, ValType::I64),
            Descriptor::F32 => self.out_option_sentinel64(AdapterType::F32),
            Descriptor::F64 => self.option_native(true, ValType::F64),
            Descriptor::I128 => {
                self.instruction(
                    &[AdapterType::I32, AdapterType::I64, AdapterType::I64],
                    Instruction::OptionWasmToInt128 { signed: true },
                    &[AdapterType::S128.option()],
                );
            }
            Descriptor::U128 => {
                self.instruction(
                    &[AdapterType::I32, AdapterType::I64, AdapterType::I64],
                    Instruction::OptionWasmToInt128 { signed: false },
                    &[AdapterType::U128.option()],
                );
            }
            Descriptor::Boolean => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::OptionBoolFromI32,
                    &[AdapterType::Bool.option()],
                );
            }
            Descriptor::Char => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::OptionCharFromI32,
                    &[AdapterType::String.option()],
                );
            }
            Descriptor::Enum { name, hole } => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::OptionEnumFromI32 { hole: *hole },
                    &[AdapterType::Enum(name.clone()).option()],
                );
            }
            Descriptor::StringEnum { name, .. } => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::OptionWasmToStringEnum { name: name.clone() },
                    &[AdapterType::StringEnum(name.clone()).option()],
                );
            }
            Descriptor::RustStruct(name) => {
                let ptr_ty = if self.cx.memory64() {
                    AdapterType::F64
                } else {
                    self.ptr_ty()
                };
                self.instruction(
                    &[ptr_ty],
                    Instruction::OptionRustFromI32 {
                        class: name.to_string(),
                    },
                    &[AdapterType::Struct(name.clone()).option()],
                );
            }
            Descriptor::Ref(d) => self.outgoing_option_ref(false, d)?,
            Descriptor::RefMut(d) => self.outgoing_option_ref(true, d)?,

            Descriptor::CachedString => self.cached_string(true)?,

            Descriptor::String | Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional slice type for calling JS function from Rust {arg:?}"
                    )
                })?;
                let mem = self.cx.memory()?;
                let free = self.cx.free()?;
                let ptr_ty = self.outgoing_internal_word_ty();
                self.instruction(
                    &[ptr_ty.clone(), ptr_ty],
                    Instruction::OptionVectorLoad {
                        kind: kind.clone(),
                        mem,
                        free,
                    },
                    &[AdapterType::Vector(kind).option()],
                );
            }

            Descriptor::NonNull => {
                let ptr_ty = if self.cx.memory64() {
                    AdapterType::F64
                } else {
                    self.ptr_ty()
                };
                self.instruction(
                    &[ptr_ty],
                    Instruction::OptionNonNullFromI32,
                    &[AdapterType::NonNull.option()],
                );
            }
            Descriptor::RawPointer => {
                self.out_option_sentinel64(AdapterType::U32);
            }

            _ => bail!(
                "unsupported optional argument type for calling JS function from Rust: {arg:?}"
            ),
        }
        Ok(())
    }

    fn outgoing_result(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Externref
            | Descriptor::NamedExternref(_)
            | Descriptor::I8
            | Descriptor::U8
            | Descriptor::I16
            | Descriptor::U16
            | Descriptor::I32
            | Descriptor::U32
            | Descriptor::F32
            | Descriptor::F64
            | Descriptor::I64
            | Descriptor::U64
            | Descriptor::I64AsF64
            | Descriptor::U64AsF64
            | Descriptor::I128
            | Descriptor::U128
            | Descriptor::Boolean
            | Descriptor::Char
            | Descriptor::Enum { .. }
            | Descriptor::StringEnum { .. }
            | Descriptor::DynamicUnion { .. }
            | Descriptor::RustStruct(_)
            | Descriptor::Ref(_)
            | Descriptor::RefMut(_)
            | Descriptor::CachedString
            | Descriptor::Option(_)
            | Descriptor::Vector(_)
            | Descriptor::Unit
            | Descriptor::NonNull
            | Descriptor::RawPointer => {
                // We must throw before reading the Ok type, if there is an error. However, the
                // structure of ResultAbi is that the Err value + discriminant come last (for
                // alignment reasons). So the UnwrapResult instruction must come first, but the
                // inputs must be read last.
                //
                // So first, push an UnwrapResult instruction without modifying the inputs list.
                //
                //     []
                //     -------------------------<
                //     UnwrapResult { popped: 2 }
                //
                self.instructions.push(InstructionData {
                    instr: Instruction::UnwrapResult {
                        table_and_drop: None,
                    },
                    stack_change: StackChange::Modified {
                        popped: 2,
                        pushed: 0,
                    },
                });

                // Then push whatever else you were going to do, modifying the inputs and
                // instructions.
                //
                //     [f64, u32, u32]
                //     -------------------------<
                //     UnwrapResult { popped: 2 }
                //     SomeOtherInstruction { popped: 3 }
                //
                // The popped numbers don't add up yet (3 != 5), but they will.
                let len = self.instructions.len();
                self._outgoing(arg)?;

                // check we did not add any deferred calls, because we have undermined the idea of
                // running them unconditionally in a finally {} block. String does this, but we
                // special case it.
                assert!(!self.instructions[len..]
                    .iter()
                    .any(|idata| matches!(idata.instr, Instruction::DeferFree { .. })));

                // Finally, we add the two inputs to UnwrapResult, and everything checks out
                //
                //     [f64, u32, u32, u32, u32]
                //     -------------------------<
                //     UnwrapResult { popped: 2 }
                //     SomeOtherInstruction { popped: 3 }
                //
                self.get(AdapterType::I32);
                self.get(AdapterType::I32);
            }
            Descriptor::String => {
                // fetch the ptr/length ...
                let ptr_ty = self.outgoing_internal_word_ty();
                self.get(ptr_ty.clone());
                self.get(ptr_ty);
                // fetch the err/is_err
                self.get(AdapterType::I32);
                self.get(AdapterType::I32);

                self.instructions.push(InstructionData {
                    instr: Instruction::UnwrapResultString {
                        table_and_drop: None,
                    },
                    stack_change: StackChange::Modified {
                        // 2 from UnwrapResult, 2 from ptr/len
                        popped: 4,
                        // pushes the ptr/len back on
                        pushed: 2,
                    },
                });

                // ... then defer a call to `free` to happen later
                // this will run string's DeferCallCore with the length parameter, but if is_err,
                // then we have never written anything into that, so it is poison. So we'll have to
                // make sure we call it with length 0, which according to __wbindgen_free's
                // implementation is always safe. We do this in UnwrapResultString's
                // implementation.
                let free = self.cx.free()?;
                self.instructions.push(InstructionData {
                    instr: Instruction::DeferFree { free, align: 1 },
                    stack_change: StackChange::Modified {
                        popped: 2,
                        pushed: 2,
                    },
                });

                // ... and then convert it to a string type
                self.instructions.push(InstructionData {
                    instr: Instruction::MemoryToString(self.cx.memory()?),
                    stack_change: StackChange::Modified {
                        popped: 2,
                        pushed: 1,
                    },
                });
                self.output.push(AdapterType::String);
            }

            Descriptor::ClampedU8
            | Descriptor::Function(_)
            | Descriptor::Closure(_)
            | Descriptor::Slice(_)
            | Descriptor::Result(_) => {
                bail!("unsupported Result type for returning from exported Rust function: {arg:?}")
            }
        }
        Ok(())
    }

    fn outgoing_option_ref(&mut self, _mutable: bool, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Externref => {
                // If this is `Some` then it's the index, otherwise if it's
                // `None` then it's the index pointing to undefined.
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::TableGet,
                    &[AdapterType::Externref.option()],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::TableGet,
                    &[AdapterType::NamedExternref(name.clone()).option()],
                );
            }
            Descriptor::CachedString => self.cached_string(false)?,
            Descriptor::String | Descriptor::Slice(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional slice type for calling JS function from Rust {arg:?}"
                    )
                })?;
                let mem = self.cx.memory()?;
                let ptr_ty = self.outgoing_internal_word_ty();
                self.instruction(
                    &[ptr_ty.clone(), ptr_ty],
                    Instruction::OptionView {
                        kind: kind.clone(),
                        mem,
                    },
                    &[AdapterType::Vector(kind).option()],
                );
            }
            // `slice_to_array` for `Option<&[T]>`: same rewrite as the
            // non-optional case yields `Option<&Vec<T>>` whose descriptor
            // is `Option(Ref(Vector(T)))`. Same wire format as
            // `Option<Vec<T>>` but produces a plain JS `Array` rather
            // than a typed array for primitive element kinds. See the
            // `Descriptor::Vector` arm in `outgoing_ref`.
            Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional vector type for calling JS function from Rust {arg:?}"
                    )
                })?;
                let mem = self.cx.memory()?;
                let free = self.cx.free()?;
                let ptr_ty = self.outgoing_internal_word_ty();
                self.instruction(
                    &[ptr_ty.clone(), ptr_ty],
                    Instruction::OptionVectorLoadAsArray {
                        kind: kind.clone(),
                        mem,
                        free,
                    },
                    &[AdapterType::Vector(kind).option()],
                );
            }
            _ => bail!(
                "unsupported optional ref argument type for calling JS function from Rust: {arg:?}"
            ),
        }
        Ok(())
    }

    fn outgoing_string_enum(&mut self, name: &str) {
        self.instruction(
            &[AdapterType::I32],
            Instruction::WasmToStringEnum {
                name: name.to_string(),
            },
            &[AdapterType::StringEnum(name.to_string())],
        );
    }

    fn outgoing_dynamic_union(&mut self, name: &str) -> Result<(), Error> {
        // Dynamic unions use the JsValue ABI (externref).
        self.instruction(
            &[AdapterType::I32],
            Instruction::ExternrefLoadOwned {
                table_and_drop: None,
            },
            &[AdapterType::DynamicUnion(name.to_string())],
        );
        Ok(())
    }

    fn outgoing_i32(&mut self, output: AdapterType) {
        let instr = Instruction::WasmToInt32 {
            unsigned_32: output == AdapterType::U32 || output == AdapterType::NonNull,
        };
        self.instruction(&[AdapterType::I32], instr, &[output]);
    }
    fn outgoing_i64(&mut self, output: AdapterType) {
        let instr = Instruction::WasmToInt64 {
            unsigned: output == AdapterType::U64,
        };
        self.instruction(&[AdapterType::I64], instr, &[output]);
    }
    fn outgoing_f64(&mut self) {
        self.get(AdapterType::F64);
        self.output.push(AdapterType::F64);
    }

    fn cached_string(&mut self, owned: bool) -> Result<(), Error> {
        let mem = self.cx.memory()?;
        let free = self.cx.free()?;
        let ptr_ty = self.outgoing_internal_word_ty();
        self.instruction(
            &[ptr_ty.clone(), ptr_ty],
            Instruction::CachedStringLoad {
                owned,
                mem,
                free,
                table: None,
            },
            &[AdapterType::String],
        );
        Ok(())
    }

    fn option_native(&mut self, signed: bool, ty: ValType) {
        let adapter_ty = AdapterType::from_wasm(ty).unwrap();
        self.instruction(
            &[AdapterType::I32, adapter_ty.clone()],
            Instruction::ToOptionNative { signed, ty },
            &[adapter_ty.option()],
        );
    }

    fn out_option_sentinel32(&mut self, ty: AdapterType) {
        self.instruction(
            &[AdapterType::I32],
            Instruction::OptionU32Sentinel,
            &[ty.option()],
        );
    }

    fn out_option_sentinel64(&mut self, ty: AdapterType) {
        self.instruction(
            &[AdapterType::F64],
            Instruction::OptionF64Sentinel,
            &[ty.option()],
        );
    }

    fn outgoing_internal_word_ty(&self) -> AdapterType {
        if self.return_position && self.cx.memory64() {
            AdapterType::F64
        } else {
            self.ptr_ty()
        }
    }
}

/// Whether `Ref(d)` can be handed to JS by simply copying the value, i.e.
/// whether `d` is a scalar whose wire form is identical by value and by
/// shared reference.
///
/// This is the CLI half of a two-sided invariant. The Rust half is the
/// `scalar_into_wasm_abi!` list in `src/convert/impls.rs`, which decides for
/// which `T` an `impl IntoWasmAbi for &T` exists at all. If this function
/// accepts less than that list, a program compiles and then fails in the CLI;
/// if it accepts more, this arm is dead. The `scalar-ref-args` reference test
/// binds every type in that list, so it fails if this function is narrower;
/// `scalar_by_shared_ref_set_is_exactly_the_scalars` below pins this side.
///
/// The match is deliberately exhaustive (no `_` arm) so that adding a
/// `Descriptor` variant is a compile error here and forces an explicit
/// decision about its by-reference wire form.
fn is_scalar_by_shared_ref(d: &Descriptor) -> bool {
    match d {
        Descriptor::I8
        | Descriptor::U8
        | Descriptor::I16
        | Descriptor::U16
        | Descriptor::I32
        | Descriptor::U32
        | Descriptor::I64
        | Descriptor::U64
        | Descriptor::I64AsF64
        | Descriptor::U64AsF64
        | Descriptor::I128
        | Descriptor::U128
        | Descriptor::F32
        | Descriptor::F64
        | Descriptor::Boolean
        | Descriptor::Char => true,

        // `ClampedU8` only ever arises under `#[wasm_bindgen(clamped)]`, which
        // applies to `Clamped<T>`; `Clamped<u8>` is not `ScalarIntoWasmAbi`, so
        // there is no `impl IntoWasmAbi for &Clamped<u8>` and `Ref(ClampedU8)`
        // is unreachable. Excluded on purpose.
        Descriptor::ClampedU8
        // Handled by earlier, more specific arms of `outgoing_ref`, or genuinely
        // unsupported behind a reference.
        | Descriptor::Function(_)
        | Descriptor::Closure(_)
        | Descriptor::Ref(_)
        | Descriptor::RefMut(_)
        | Descriptor::Slice(_)
        | Descriptor::Vector(_)
        | Descriptor::CachedString
        | Descriptor::String
        | Descriptor::Externref
        | Descriptor::NamedExternref(_)
        | Descriptor::Enum { .. }
        | Descriptor::StringEnum { .. }
        | Descriptor::DynamicUnion { .. }
        | Descriptor::RustStruct(_)
        | Descriptor::Option(_)
        | Descriptor::Result(_)
        | Descriptor::Unit
        | Descriptor::NonNull
        | Descriptor::RawPointer => false,
    }
}

#[test]
fn closure_word_descriptor_uses_number_abi_on_memory64() {
    assert_eq!(closure_word_descriptor(true), Descriptor::I64AsF64);
    assert_eq!(closure_word_descriptor(false), Descriptor::I32);
}

/// Every `Descriptor` variant, so the tests below can enumerate the accepted
/// set rather than restating it. Payloads are irrelevant to
/// `is_scalar_by_shared_ref`, which only inspects the discriminant.
#[cfg(test)]
fn all_descriptor_variants() -> Vec<Descriptor> {
    use crate::descriptor::Function;

    let f = || Function {
        arguments: Vec::new(),
        shim_idx: 0,
        ret: Descriptor::Unit,
        inner_ret: None,
    };
    vec![
        Descriptor::I8,
        Descriptor::U8,
        Descriptor::ClampedU8,
        Descriptor::I16,
        Descriptor::U16,
        Descriptor::I32,
        Descriptor::U32,
        Descriptor::I64,
        Descriptor::U64,
        Descriptor::I64AsF64,
        Descriptor::U64AsF64,
        Descriptor::I128,
        Descriptor::U128,
        Descriptor::F32,
        Descriptor::F64,
        Descriptor::Boolean,
        Descriptor::Function(Box::new(f())),
        Descriptor::Closure(Box::new(crate::descriptor::Closure {
            owned: false,
            function: f(),
            mutable: false,
        })),
        Descriptor::Ref(Box::new(Descriptor::Unit)),
        Descriptor::RefMut(Box::new(Descriptor::Unit)),
        Descriptor::Slice(Box::new(Descriptor::U8)),
        Descriptor::Vector(Box::new(Descriptor::U8)),
        Descriptor::CachedString,
        Descriptor::String,
        Descriptor::Externref,
        Descriptor::NamedExternref("X".into()),
        Descriptor::Enum {
            name: "E".into(),
            hole: 0,
        },
        Descriptor::StringEnum {
            name: "S".into(),
            invalid: 0,
            hole: 1,
        },
        Descriptor::DynamicUnion {
            name: "D".into(),
            variant_types: Vec::new(),
        },
        Descriptor::RustStruct("R".into()),
        Descriptor::Char,
        Descriptor::Option(Box::new(Descriptor::U8)),
        Descriptor::Result(Box::new(Descriptor::U8)),
        Descriptor::Unit,
        Descriptor::NonNull,
        Descriptor::RawPointer,
    ]
}

/// Guards the CLI half of the invariant on its own: the exact set of
/// descriptors accepted behind a `Ref(..)`.
///
/// Deliberately spelled out rather than derived, so that *changing*
/// `is_scalar_by_shared_ref` fails here and has to be done on purpose.
#[test]
fn scalar_by_shared_ref_set_is_exactly_the_scalars() {
    let accepted: Vec<_> = all_descriptor_variants()
        .into_iter()
        .filter(is_scalar_by_shared_ref)
        .collect();

    assert_eq!(
        accepted,
        vec![
            Descriptor::I8,
            Descriptor::U8,
            Descriptor::I16,
            Descriptor::U16,
            Descriptor::I32,
            Descriptor::U32,
            Descriptor::I64,
            Descriptor::U64,
            Descriptor::I64AsF64,
            Descriptor::U64AsF64,
            Descriptor::I128,
            Descriptor::U128,
            Descriptor::F32,
            Descriptor::F64,
            Descriptor::Boolean,
            Descriptor::Char,
        ],
    );
}

