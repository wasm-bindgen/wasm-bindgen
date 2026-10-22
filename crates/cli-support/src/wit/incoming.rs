//! Definition of how to convert Rust types (`Description`) into Wasm types
//! through adapter functions.
//!
//! Note that many Rust types use "nonstandard" instructions which only work in
//! the JS output, not for the "pure Wasm interface types" output.
//!
//! Note that the mirror operation, going from WebAssembly to JS, is found in
//! the `outgoing.rs` module.

use crate::descriptor::{Descriptor, SeqValue, SequenceCell};
use crate::wit::InstructionData;
use crate::wit::{AdapterType, Instruction, InstructionBuilder, OptionKind, StackChange};
use anyhow::{bail, format_err, Error};
use walrus::ValType;

impl InstructionBuilder<'_, '_> {
    /// Process a `Descriptor` as if it's being passed from JS to Rust. This
    /// will skip `Unit` and otherwise internally add instructions necessary to
    /// convert the foreign type into the Rust bits.
    pub fn incoming(&mut self, arg: &Descriptor) -> Result<(), Error> {
        if let Descriptor::Unit = arg {
            return Ok(());
        }
        // This is a wrapper around `_incoming` to have a number of sanity checks
        // that we don't forget things. We should always produce at least one
        // Wasm arg and exactly one webidl arg. Additionally the number of
        // bindings should always match the number of webidl types for now.
        let input_before = self.input.len();
        let output_before = self.output.len();
        self._incoming(arg)?;
        assert_eq!(
            input_before + 1,
            self.input.len(),
            "didn't push an input {arg:?}"
        );
        assert!(
            output_before < self.output.len(),
            "didn't push more outputs {arg:?}"
        );
        Ok(())
    }

    fn _incoming(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Boolean => {
                self.instruction(
                    &[AdapterType::Bool],
                    Instruction::I32FromBool,
                    &[AdapterType::I32],
                );
            }
            Descriptor::Char => {
                self.instruction(
                    &[AdapterType::String],
                    Instruction::I32FromStringFirstChar,
                    &[AdapterType::I32],
                );
            }
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::Externref],
                    Instruction::I32FromExternrefOwned,
                    &[AdapterType::I32],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::NamedExternref(name.clone())],
                    Instruction::I32FromExternrefOwned,
                    &[AdapterType::I32]
                )
            }
            Descriptor::RustStruct { name: class, .. } => {
                let ptr_ty = if self.cx.memory64() {
                    AdapterType::F64
                } else {
                    self.ptr_ty()
                };
                self.instruction(
                    &[AdapterType::Struct(class.clone())],
                    Instruction::I32FromExternrefRustOwned {
                        class: class.clone(),
                    },
                    &[ptr_ty],
                );
            }
            Descriptor::I8 => self.number_i32(AdapterType::S8),
            Descriptor::U8 => self.number_i32(AdapterType::U8),
            Descriptor::I16 => self.number_i32(AdapterType::S16),
            Descriptor::U16 => self.number_i32(AdapterType::U16),
            Descriptor::I32 => self.number_i32(AdapterType::S32),
            Descriptor::U32 => self.number_i32(AdapterType::U32),
            Descriptor::I64 => self.number_i64(AdapterType::S64),
            Descriptor::U64 => self.number_i64(AdapterType::U64),
            Descriptor::I64AsF64 | Descriptor::U64AsF64 => {
                self.number_f64(AdapterType::F64);
            }
            Descriptor::RawPointer => {
                if self.cx.memory64() {
                    self.number_f64(AdapterType::F64);
                } else {
                    self.number_i32(AdapterType::U32);
                }
            }
            Descriptor::I128 => {
                self.instruction(
                    &[AdapterType::S128],
                    Instruction::Int128ToWasm,
                    &[AdapterType::I64, AdapterType::I64],
                );
            }
            Descriptor::U128 => {
                self.instruction(
                    &[AdapterType::U128],
                    Instruction::Int128ToWasm,
                    &[AdapterType::I64, AdapterType::I64],
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
            Descriptor::Enum { name, .. } => {
                self.instruction(
                    &[AdapterType::Enum(name.clone())],
                    Instruction::F64FromEnum,
                    &[AdapterType::F64],
                );
            },
            Descriptor::StringEnum { name, invalid, .. } => {
                self.instruction(
                    &[AdapterType::StringEnum(name.clone())],
                    Instruction::StringEnumToWasm {
                        name: name.clone(),
                        invalid: *invalid,
                    },
                    &[AdapterType::I32],
                );
            },
            Descriptor::DynamicUnion { name, .. } => {
                // Dynamic unions use the JsValue ABI (externref).
                self.instruction(
                    &[AdapterType::DynamicUnion(name.clone())],
                    Instruction::I32FromExternrefOwned,
                    &[AdapterType::I32],
                );
            },
            Descriptor::Ref(d) => self.incoming_ref(false, d)?,
            Descriptor::RefMut(d) => self.incoming_ref(true, d)?,
            Descriptor::Option { inner, strict } => {
                self.incoming_option(inner, OptionKind::option(*strict))?
            }
            Descriptor::Nullable(inner) => self.incoming_option(inner, OptionKind::Nullable)?,

            Descriptor::String | Descriptor::CachedString => {
                let ptr_ty = self.incoming_internal_word_ty();
                self.instruction(
                    &[AdapterType::String],
                    Instruction::StringToMemory {
                        malloc: self.cx.malloc()?,
                        free: self.cx.free()?,
                        realloc: self.cx.realloc(),
                        mem: self.cx.memory()?,
                    },
                    &[ptr_ty.clone(), ptr_ty],
                );
            }

            Descriptor::Vector(cell) => {
                let kind = match cell.sequence_cell() {
                    SequenceCell::InPlace(kind) => kind,
                    SequenceCell::Encoded(elem) => {
                        return self.incoming_encoded_vector(elem, false, OptionKind::Lenient)
                    }
                };
                let ptr_ty = self.incoming_internal_word_ty();
                self.instruction(
                    &[AdapterType::Vector(kind.clone())],
                    Instruction::VectorToMemory {
                        kind,
                        malloc: self.cx.malloc()?,
                        free: self.cx.free()?,
                        mem: self.cx.memory()?,
                    },
                    &[ptr_ty.clone(), ptr_ty],
                );
            }

            // Can't be passed from JS to Rust yet
            Descriptor::Function(_) |
            Descriptor::Closure(_) |

            Descriptor::Result(_) |
            // Only on imported function arguments
            Descriptor::SeqMode(..) |
            // Always behind a `Ref`
            Descriptor::Slice(_) => bail!(
                "unsupported argument type for calling Rust function from JS: {arg:?}"
            ),

            // nothing to do
            Descriptor::Unit => {}

            // Largely synthetic and can't show up
            Descriptor::ClampedU8 => unreachable!(),

            Descriptor::NonNull => {
                let ptr_ty = if self.cx.memory64() {
                    AdapterType::F64
                } else {
                    self.ptr_ty()
                };
                self.instruction(
                    &[AdapterType::NonNull],
                    Instruction::I32FromNonNull,
                    &[ptr_ty],
                )
            }
        }
        Ok(())
    }

    fn incoming_ref(&mut self, mutable: bool, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::RustStruct { name: class, .. } => {
                let ptr_ty = if self.cx.memory64() {
                    AdapterType::F64
                } else {
                    self.ptr_ty()
                };
                self.instruction(
                    &[AdapterType::Struct(class.clone())],
                    Instruction::I32FromExternrefRustBorrow {
                        class: class.clone(),
                    },
                    &[ptr_ty],
                );
            }
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::Externref],
                    Instruction::I32FromExternrefBorrow,
                    &[AdapterType::I32],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::NamedExternref(name.clone())],
                    Instruction::I32FromExternrefBorrow,
                    &[AdapterType::I32],
                );
            }
            Descriptor::String | Descriptor::CachedString => {
                // This allocation is cleaned up once it's received in Rust.
                let ptr_ty = self.ptr_ty();
                self.instruction(
                    &[AdapterType::String],
                    Instruction::StringToMemory {
                        malloc: self.cx.malloc()?,
                        free: self.cx.free()?,
                        realloc: self.cx.realloc(),
                        mem: self.cx.memory()?,
                    },
                    &[ptr_ty.clone(), ptr_ty],
                );
            }
            Descriptor::Slice(_) => {
                // like strings, this allocation is cleaned up after being
                // received in Rust.
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported argument type for calling Rust function from JS {arg:?}"
                    )
                })?;
                let ptr_ty = self.ptr_ty();
                if mutable {
                    self.instruction(
                        &[AdapterType::MutableSlice(kind.clone())],
                        Instruction::MutableSliceToMemory {
                            kind,
                            malloc: self.cx.malloc()?,
                            free: self.cx.free()?,
                            mem: self.cx.memory()?,
                        },
                        &[ptr_ty.clone(), ptr_ty, AdapterType::Externref],
                    );
                    self.late_instruction(
                        &[AdapterType::Externref],
                        Instruction::I32FromExternrefOwned,
                        &[AdapterType::I32],
                    );
                } else {
                    self.instruction(
                        &[AdapterType::Vector(kind.clone())],
                        Instruction::VectorToMemory {
                            kind,
                            malloc: self.cx.malloc()?,
                            free: self.cx.free()?,
                            mem: self.cx.memory()?,
                        },
                        &[ptr_ty.clone(), ptr_ty],
                    );
                }
            }
            _ => bail!(
                "unsupported reference argument type for calling Rust function from JS: {arg:?}"
            ),
        }
        Ok(())
    }

    fn incoming_option(&mut self, arg: &Descriptor, option_kind: OptionKind) -> Result<(), Error> {
        match arg {
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::Externref.option_kind(option_kind)],
                    Instruction::I32FromOptionExternref {
                        table_and_alloc: None,
                        kind: option_kind,
                    },
                    &[AdapterType::I32],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::NamedExternref(name.clone()).option_kind(option_kind)],
                    Instruction::I32FromOptionExternref {
                        table_and_alloc: None,
                        kind: option_kind,
                    },
                    &[AdapterType::I32],
                );
            }
            Descriptor::DynamicUnion { name, .. } => {
                // Dynamic unions share the externref ABI; reuse the externref
                // option lowering.
                self.instruction(
                    &[AdapterType::DynamicUnion(name.clone()).option_kind(option_kind)],
                    Instruction::I32FromOptionExternref {
                        table_and_alloc: None,
                        kind: option_kind,
                    },
                    &[AdapterType::I32],
                );
            }
            Descriptor::I8 => self.in_option_sentinel32(AdapterType::S8, option_kind),
            Descriptor::U8 => self.in_option_sentinel32(AdapterType::U8, option_kind),
            Descriptor::I16 => self.in_option_sentinel32(AdapterType::S16, option_kind),
            Descriptor::U16 => self.in_option_sentinel32(AdapterType::U16, option_kind),
            Descriptor::I32 => self.in_option_sentinel64_int(AdapterType::I32, true, option_kind),
            Descriptor::U32 => self.in_option_sentinel64_int(AdapterType::U32, false, option_kind),
            Descriptor::F32 => self.in_option_sentinel64_f32(AdapterType::F32, option_kind),
            Descriptor::F64 => self.in_option_native(ValType::F64, option_kind),
            Descriptor::I64AsF64 | Descriptor::U64AsF64 => {
                self.in_option_sentinel64_number(option_kind)
            }
            Descriptor::I64 | Descriptor::U64 => self.in_option_native(ValType::I64, option_kind),
            Descriptor::I128 => {
                self.instruction(
                    &[AdapterType::S128.option_kind(option_kind)],
                    Instruction::OptionInt128ToWasm { kind: option_kind },
                    &[AdapterType::I32, AdapterType::I64, AdapterType::I64],
                );
            }
            Descriptor::U128 => {
                self.instruction(
                    &[AdapterType::U128.option_kind(option_kind)],
                    Instruction::OptionInt128ToWasm { kind: option_kind },
                    &[AdapterType::I32, AdapterType::I64, AdapterType::I64],
                );
            }
            Descriptor::Boolean => {
                self.instruction(
                    &[AdapterType::Bool.option_kind(option_kind)],
                    Instruction::I32FromOptionBool { kind: option_kind },
                    &[AdapterType::I32],
                );
            }
            Descriptor::Char => {
                self.instruction(
                    &[AdapterType::String.option_kind(option_kind)],
                    Instruction::I32FromOptionChar { kind: option_kind },
                    &[AdapterType::I32],
                );
            }
            Descriptor::Enum { name, hole, .. } => {
                self.instruction(
                    &[AdapterType::Enum(name.clone()).option_kind(option_kind)],
                    Instruction::F64FromOptionEnum {
                        hole: *hole,
                        kind: option_kind,
                    },
                    &[AdapterType::F64],
                );
            }
            Descriptor::StringEnum {
                name,
                invalid,
                hole,
                ..
            } => {
                self.instruction(
                    &[AdapterType::StringEnum(name.clone()).option_kind(option_kind)],
                    Instruction::OptionStringEnumToWasm {
                        name: name.clone(),
                        invalid: *invalid,
                        hole: *hole,
                        kind: option_kind,
                    },
                    &[AdapterType::I32],
                );
            }
            Descriptor::RustStruct { name, .. } => {
                let ptr_ty = if self.cx.memory64() {
                    AdapterType::F64
                } else {
                    self.ptr_ty()
                };
                self.instruction(
                    &[AdapterType::Struct(name.clone()).option_kind(option_kind)],
                    Instruction::I32FromOptionRust {
                        class: name.to_string(),
                        kind: option_kind,
                    },
                    &[ptr_ty],
                );
            }

            Descriptor::String | Descriptor::CachedString => {
                let malloc = self.cx.malloc()?;
                let free = self.cx.free()?;
                let mem = self.cx.memory()?;
                let realloc = self.cx.realloc();
                let ptr_ty = self.incoming_internal_word_ty();
                self.instruction(
                    &[AdapterType::String.option_kind(option_kind)],
                    Instruction::OptionString {
                        malloc,
                        free,
                        mem,
                        realloc,
                        kind: option_kind,
                    },
                    &[ptr_ty.clone(), ptr_ty],
                );
            }

            Descriptor::Vector(cell) => {
                let kind = match cell.sequence_cell() {
                    SequenceCell::InPlace(kind) => kind,
                    SequenceCell::Encoded(elem) => {
                        return self.incoming_encoded_vector(elem, true, option_kind)
                    }
                };
                let malloc = self.cx.malloc()?;
                let free = self.cx.free()?;
                let mem = self.cx.memory()?;
                let ptr_ty = self.incoming_internal_word_ty();
                self.instruction(
                    &[AdapterType::Vector(kind.clone()).option_kind(option_kind)],
                    Instruction::OptionVector {
                        kind,
                        malloc,
                        free,
                        mem,
                        option_kind,
                    },
                    &[ptr_ty.clone(), ptr_ty],
                );
            }

            // Every non-null address is a `NonNull`, so a nullable one crosses
            // as a pointer with the `f64` sentinels instead of the null word.
            Descriptor::NonNull if option_kind.nullable() => {
                self.in_option_sentinel64_number(option_kind)
            }
            Descriptor::NonNull => {
                let ptr_ty = if self.cx.memory64() {
                    AdapterType::F64
                } else {
                    self.ptr_ty()
                };
                self.instruction(
                    &[AdapterType::NonNull.option_kind(option_kind)],
                    Instruction::I32FromOptionNonNull { kind: option_kind },
                    &[ptr_ty],
                )
            }
            Descriptor::RawPointer => {
                self.in_option_sentinel64_number(option_kind);
            }

            _ => bail!(
                "unsupported optional argument type for calling Rust function from JS: {arg:?}"
            ),
        }
        Ok(())
    }

    pub fn get(&mut self, ty: AdapterType) {
        self.input.push(ty);

        // If we're generating instructions in the return position then the
        // arguments are already on the stack to consume, otherwise we need to
        // fetch them from the parameters.
        if !self.return_position {
            let idx = self.input.len() as u32 - 1;
            self.instructions.push(InstructionData {
                instr: Instruction::ArgGet(idx),
                stack_change: StackChange::Modified {
                    pushed: 1,
                    popped: 0,
                },
            });
        }
    }

    pub fn instruction(
        &mut self,
        inputs: &[AdapterType],
        instr: Instruction,
        outputs: &[AdapterType],
    ) {
        // If we're generating instructions in the return position then the
        // arguments are already on the stack to consume, otherwise we need to
        // fetch them from the parameters.
        if !self.return_position {
            for input in inputs {
                self.get(input.clone());
            }
        } else {
            self.input.extend_from_slice(inputs);
        }

        self.instructions.push(InstructionData {
            instr,
            stack_change: StackChange::Modified {
                popped: inputs.len(),
                pushed: outputs.len(),
            },
        });
        self.output.extend_from_slice(outputs);
    }

    /// Add an instruction whose inputs are the results of previous instructions
    /// instead of the parameters from JS / results from Rust.
    pub fn late_instruction(
        &mut self,
        inputs: &[AdapterType],
        instr: Instruction,
        outputs: &[AdapterType],
    ) {
        for input in inputs {
            assert_eq!(self.output.pop().unwrap(), *input);
        }
        self.instructions.push(InstructionData {
            instr,
            stack_change: StackChange::Modified {
                popped: inputs.len(),
                pushed: outputs.len(),
            },
        });
        self.output.extend_from_slice(outputs);
    }

    fn number_i32(&mut self, input: AdapterType) {
        self.instruction(&[input], Instruction::Int32ToWasm, &[AdapterType::I32]);
    }
    fn number_i64(&mut self, input: AdapterType) {
        self.instruction(&[input], Instruction::Int64ToWasm, &[AdapterType::I64]);
    }
    fn number_f64(&mut self, input: AdapterType) {
        self.get(input);
        self.output.push(AdapterType::F64);
    }

    fn in_option_native(&mut self, wasm: ValType, option_kind: OptionKind) {
        let ty = AdapterType::from_wasm(wasm).unwrap();
        self.instruction(
            &[ty.clone().option_kind(option_kind)],
            Instruction::FromOptionNative {
                ty: wasm,
                kind: option_kind,
            },
            &[AdapterType::I32, ty],
        );
    }

    fn in_option_sentinel32(&mut self, ty: AdapterType, option_kind: OptionKind) {
        self.instruction(
            &[ty.option_kind(option_kind)],
            Instruction::I32FromOptionU32Sentinel { kind: option_kind },
            &[AdapterType::I32],
        );
    }
    fn in_option_sentinel64_int(&mut self, ty: AdapterType, signed: bool, option_kind: OptionKind) {
        self.instruction(
            &[ty.option_kind(option_kind)],
            Instruction::F64FromOptionSentinelInt {
                signed,
                kind: option_kind,
            },
            &[AdapterType::F64],
        );
    }
    fn in_option_sentinel64_f32(&mut self, ty: AdapterType, option_kind: OptionKind) {
        self.instruction(
            &[ty.option_kind(option_kind)],
            Instruction::F64FromOptionSentinelF32 { kind: option_kind },
            &[AdapterType::F64],
        );
    }

    fn in_option_sentinel64_number(&mut self, option_kind: OptionKind) {
        self.instruction(
            &[AdapterType::F64.option_kind(option_kind)],
            Instruction::F64FromOptionSentinelNumber { kind: option_kind },
            &[AdapterType::F64],
        );
    }

    /// An owned vector with encoded cells is a buffer of its elements'
    /// scalar ABIs, the mirror of `outgoing_encoded_slice`. JS encodes each
    /// element with the element's own instructions, as for an argument of the
    /// element type, and writes its ABI words into the buffer, which Rust
    /// takes ownership of.
    fn incoming_encoded_vector(
        &mut self,
        elem: &Descriptor,
        optional: bool,
        option_kind: OptionKind,
    ) -> Result<(), Error> {
        let mut encode = InstructionBuilder {
            input: Vec::new(),
            output: Vec::new(),
            instructions: Vec::new(),
            cx: &mut *self.cx,
            // The element is pushed onto the stack by the loop, not fetched
            // as an argument.
            return_position: true,
            seq_value: SeqValue::View,
        };
        // Every element that isn't in place reaches here, so this is what
        // rejects one the glue can't encode. Element encoders must not call
        // into wasm: the glue writes the whole buffer through one view after
        // allocating it. That rules out elements that own memory, such as a
        // string.
        match elem {
            Descriptor::Boolean
            | Descriptor::Char
            | Descriptor::Enum { .. }
            | Descriptor::StringEnum { .. } => encode._incoming(elem)?,
            _ => {
                bail!("unsupported vector element type for calling Rust function from JS: {elem:?}")
            }
        }
        let InstructionBuilder {
            mut input,
            output: words,
            instructions: elem,
            ..
        } = encode;
        let array = AdapterType::Array(Box::new(input.remove(0)));
        let malloc = self.cx.malloc()?;
        let free = self.cx.free()?;
        let mem = self.cx.memory()?;
        let ptr_ty = self.incoming_internal_word_ty();
        self.instruction(
            &[if optional {
                array.option_kind(option_kind)
            } else {
                array
            }],
            Instruction::EncodedSliceStore {
                words,
                elem,
                malloc,
                free,
                mem,
                optional,
                option_kind,
            },
            &[ptr_ty.clone(), ptr_ty],
        );
        Ok(())
    }

    fn incoming_internal_word_ty(&self) -> AdapterType {
        if self.return_position && self.cx.memory64() {
            AdapterType::F64
        } else {
            self.ptr_ty()
        }
    }
}
