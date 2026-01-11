//! A tiny and incomplete Wasm interpreter
//!
//! This module contains a tiny and incomplete Wasm interpreter built on top of
//! `walrus`'s module structure. Each `Interpreter` contains some state
//! about the execution of a Wasm instance. The "incomplete" part here is
//! related to the fact that this is *only* used to execute the various
//! descriptor functions for wasm-bindgen.
//!
//! As a recap, the wasm-bindgen macro generate "descriptor functions" which
//! basically as a mapping of rustc's trait resolution in executable code. This
//! allows us to detect, after the macro is invoke, what trait selection did and
//! what types of functions look like. By executing descriptor functions they'll
//! each invoke a known import (with only one argument) some number of times,
//! which gives us a list of `u32` values to then decode.
//!
//! The interpreter here is only geared towards this one exact use case, so it's
//! quite small and likely not extra-efficient.

#![deny(missing_docs)]

use anyhow::{bail, ensure};
use std::collections::{BTreeMap, HashSet};
use walrus::ir::InstrSeqId;
use walrus::{ExportId, FunctionId, LocalFunction, LocalId, Module};

/// A ready-to-go interpreter of a Wasm module.
///
/// An interpreter currently represents effectively cached state. It is reused
/// between calls to `interpret` and is precomputed from a `Module`. It houses
/// state like the Wasm stack, Wasm memory, etc.
#[derive(Default)]
pub struct Interpreter {
    // Function index of the `__wbindgen_describe` and
    // `__wbindgen_describe_cast` imported functions. We special case this
    // to know when the environment's imported function is called.
    describe_id: Option<FunctionId>,
    describe_cast_id: Option<FunctionId>,

    // The current stack pointer (global 0) and Wasm memory (the stack). Only
    // used in a limited capacity.
    sp: i32,
    mem: Vec<i32>,
    scratch: Vec<i32>,

    // The descriptor which we're assembling, a list of `u32` entries. This is
    // very specific to wasm-bindgen and is the purpose for the existence of
    // this module.
    descriptor: Vec<u32>,

    /// The `__wbindgen_skip_interpret_calls`'s id.
    skip_interpret: Option<ExportId>,

    /// Some functions that need to skip interpret, such as `__wasm_call_ctors`
    /// and `__wasm_call_dtors`.
    skip_calls: HashSet<FunctionId>,
    stopped: bool,
}

fn skip_calls(module: &Module, id: FunctionId) -> HashSet<FunctionId> {
    use walrus::ir::*;

    let func = module.funcs.get(id);

    let local = match &func.kind {
        walrus::FunctionKind::Local(l) => l,
        _ => panic!("can only call locally defined functions"),
    };

    let entry = local.entry_block();
    let block = local.block(entry);

    block
        .instrs
        .iter()
        .filter_map(|(instr, _)| match instr {
            // There are only up to three calls for now:
            //   1. __wasm_call_ctors (`#[link_section = ".init_array"]`)
            //   2. __wbindgen_skip_interpret_calls (The original symbol, we don't care about it)
            //   3. __wasm_call_dtors (This symbol may not be present in Rust program, but may be present if C program is linked)
            Instr::Call(Call { func }) | Instr::ReturnCall(ReturnCall { func }) => Some(*func),
            // Typically, there are no other instructions, or only a return instruction.
            //
            // When coverage is turned on, there may be llvm coverage instrumentation
            // instructions.
            _ => None,
        })
        .collect()
}

impl Interpreter {
    /// Creates a new interpreter from a provided `Module`, precomputing all
    /// information necessary to interpret further.
    ///
    /// Note that the `module` passed in to this function must be the same as
    /// the `module` passed to `interpret` below.
    pub fn new(module: &Module) -> Result<Interpreter, anyhow::Error> {
        let mut ret = Interpreter::default();

        // Give ourselves some memory and set the stack pointer
        // (the LLVM call stack, now the Wasm stack, global 0) to the top.
        ret.mem = vec![0; 0x8000];
        ret.sp = ret.mem.len() as i32;

        // Figure out where the `__wbindgen_describe` imported function is, if
        // it exists. We'll special case calls to this function as our
        // interpretation should only invoke this function as an imported
        // function.
        for import in module.imports.iter() {
            let id = match import.kind {
                walrus::ImportKind::Function(id) => id,
                _ => continue,
            };
            if import.module != "__wbindgen_placeholder__" {
                continue;
            }
            if import.name == "__wbindgen_describe" {
                ret.describe_id = Some(id);
            } else if import.name == "__wbindgen_describe_cast" {
                ret.describe_cast_id = Some(id);
            }
        }

        // Setup skip_interpret id and skip_calls
        if let Some(export) = module
            .exports
            .iter()
            .find(|export| export.name == "__wbindgen_skip_interpret_calls")
        {
            let id = match export.item {
                walrus::ExportItem::Function(id) => id,
                _ => panic!("__wbindgen_skip_interpret_calls must be an export function"),
            };
            ret.skip_interpret = Some(export.id());
            ret.skip_calls = skip_calls(module, id);
        }

        Ok(ret)
    }

    /// Interprets the execution of the descriptor function `func`.
    ///
    /// This function will execute `func` in the `module` provided. Note that
    /// the `module` provided here must be the same as the one passed to `new`
    /// when this `Interpreter` was constructed.
    ///
    /// The `func` must be a wasm-bindgen descriptor function meaning that it
    /// doesn't do anything like use floats or i64. Instead all it should do is
    /// call other functions, sometimes some stack pointer manipulation, and
    /// then call the one imported `__wbindgen_describe` function. Anything else
    /// will cause this interpreter to panic.
    ///
    /// When the descriptor has finished running the assembled descriptor list
    /// is returned. The descriptor returned can then be re-parsed into an
    /// actual `Descriptor` in the cli-support crate.
    ///
    /// # Return value
    ///
    /// Returns `Some` if `func` was found in the `module` and `None` if it was
    /// not found in the `module`.
    pub fn interpret_descriptor(&mut self, id: FunctionId, module: &Module) -> &[u32] {
        self.descriptor.truncate(0);
        self.stopped = false;

        // We should have a blank Wasm and LLVM stack at both the start and end
        // of the call.
        assert_eq!(self.sp, self.mem.len() as i32);

        let func = module.funcs.get(id);
        let ty = module.types.get(func.ty());

        self.call(id, module, &vec![0; ty.params().len()]);

        assert_eq!(self.sp, self.mem.len() as i32);
        &self.descriptor
    }

    /// Returns the function id of the `__wbindgen_describe_cast`
    /// imported function.
    pub fn describe_cast_id(&self) -> Option<FunctionId> {
        self.describe_cast_id
    }

    /// Returns the export id of the `__wbindgen_skip_interpret_calls`.
    pub fn skip_interpret(&self) -> Option<ExportId> {
        self.skip_interpret
    }

    fn call(&mut self, id: FunctionId, module: &Module, args: &[i32]) {
        let func = module.funcs.get(id);
        log::trace!("starting a call of {id:?} {:?}", func.name);
        log::trace!("arguments {args:?}");
        let local = match &func.kind {
            walrus::FunctionKind::Local(l) => l,
            _ => panic!("can only call locally defined functions"),
        };

        let mut frame = Frame {
            module,
            func: local,
            interp: self,
            locals: BTreeMap::new(),
        };

        assert_eq!(local.args.len(), args.len());
        for (arg, val) in local.args.iter().zip(args) {
            frame.locals.insert(*arg, *val);
        }

        frame.eval(local.entry_block()).unwrap_or_else(|err| {
            if let Some(name) = &module.funcs.get(id).name {
                panic!("{name}: {err}")
            } else {
                panic!("{err}")
            }
        })
    }
}

struct Frame<'a> {
    module: &'a Module,
    func: &'a LocalFunction,
    interp: &'a mut Interpreter,
    locals: BTreeMap<LocalId, i32>,
}

impl Frame<'_> {
    fn eval(&mut self, seq: InstrSeqId) -> anyhow::Result<()> {
        use walrus::ir::*;

        for (instr, _) in self.func.block(seq).iter() {
            let stack = &mut self.interp.scratch;

            match instr {
                Instr::Const(c) => match c.value {
                    Value::I32(n) => stack.push(n),
                    _ => bail!("non-i32 constant"),
                },
                Instr::LocalGet(e) => stack.push(self.locals.get(&e.local).cloned().unwrap_or(0)),
                Instr::LocalSet(e) => {
                    let val = stack.pop().unwrap();
                    self.locals.insert(e.local, val);
                }
                Instr::LocalTee(e) => {
                    let val = *stack.last().unwrap();
                    self.locals.insert(e.local, val);
                }

                // Blindly assume all globals are the stack pointer
                Instr::GlobalGet(_) => stack.push(self.interp.sp),
                Instr::GlobalSet(_) => {
                    let val = stack.pop().unwrap();
                    self.interp.sp = val;
                }

                // Support simple arithmetic, mainly for the stack pointer
                // manipulation
                Instr::Binop(e) => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(match e.op {
                        BinaryOp::I32Sub => lhs - rhs,
                        BinaryOp::I32Add => lhs + rhs,
                        op => bail!("invalid binary op {op:?}"),
                    });
                }

                // Support small loads/stores to the stack. These show up in debug
                // mode where there's some traffic on the linear stack even when in
                // theory there doesn't need to be.
                Instr::Load(e) => {
                    let address = stack.pop().unwrap();
                    let address = address as u32 + e.arg.offset;
                    ensure!(
                        address > 0,
                        "Read a negative or zero address value from the stack. Did we run out of memory?"
                    );
                    let width = e.kind.width();
                    ensure!(address % width == 0);
                    let val = self.interp.mem[address as usize / 4];
                    if width == 4 {
                        stack.push(val)
                    } else if width == 1 {
                        let result = val.to_le_bytes()[(address % 4) as usize];
                        let LoadKind::I32_8 { kind } = e.kind else {
                            panic!("Unhandled load kind {:?}", e.kind)
                        };
                        match kind {
                            ExtendedLoad::SignExtend => {
                                stack.push(result as i8 as i32);
                            }
                            ExtendedLoad::ZeroExtend | ExtendedLoad::ZeroExtendAtomic => {
                                stack.push(result as i32);
                            }
                        };
                    } else {
                        panic!("Unhandled load width {width}");
                    }
                }
                Instr::Store(e) => {
                    let value = stack.pop().unwrap();
                    let address = stack.pop().unwrap();
                    let address = address as u32 + e.arg.offset;
                    ensure!(
                        address > 0,
                        "Read a negative or zero address value from the stack. Did we run out of memory?"
                    );
                    let width = e.kind.width();
                    ensure!(address % width == 0);
                    let index = address as usize / 4;
                    if width == 8 {
                        // Oops our stack is of i32s so we can't really handle a
                        // store of width 8. Just treat the more signifcant 4
                        // bytes as 0.
                        self.interp.mem[index] = value;
                        self.interp.mem[index + 1] = 0;
                    } else if width == 4 {
                        self.interp.mem[index] = value;
                    } else if width == 1 {
                        let mut bytes = self.interp.mem[index].to_le_bytes();
                        bytes[(address % 4) as usize] = value as u8;
                        self.interp.mem[index] = i32::from_le_bytes(bytes);
                    } else {
                        panic!("Unhandled store width {width}");
                    }
                }

                Instr::Return(_) => {
                    log::trace!("return");
                    break;
                }

                Instr::Drop(_) => {
                    log::trace!("drop");
                    stack.pop().unwrap();
                }

                Instr::Call(Call { func }) | Instr::ReturnCall(ReturnCall { func }) => {
                    let func = *func;
                    // If this function is calling the `__wbindgen_describe`
                    // function, which we've precomputed the id for, then
                    // it's telling us about the next `u32` element in the
                    // descriptor to return. We "call" the imported function
                    // here by directly inlining it.
                    if Some(func) == self.interp.describe_id {
                        let val = stack.pop().unwrap();
                        log::trace!("__wbindgen_describe({val})");
                        self.interp.descriptor.push(val as u32);

                    // If this function is calling the `__wbindgen_describe_cast`
                    // function then it's just a marker for the parent function
                    // to be treated as a cast.
                    } else if Some(func) == self.interp.describe_cast_id {
                        log::trace!("__wbindgen_describe_cast()");
                        // `__wbindgen_describe_cast` marks the end of the cast
                        // descriptor. Stop here, ignoring anything on the stack.
                        self.interp.sp = self.interp.mem.len() as i32;
                        self.interp.stopped = true;
                        break;

                    // ... otherwise this is a normal call so we recurse.
                    } else {
                        // Skip the constructor function.
                        //
                        // Complex logic can be implemented in the ctor, our simple interpreter will fail
                        // to execute due to missing instructions.
                        //
                        // For example, executing `1 + 1` fails due to the lack of `I32.And` instruction.
                        //
                        // Because `wasm-ld` may insert a call to ctor from the beginning of every function that
                        // your module exports, the interpreter will enter the ctor logic when parsing the
                        // `wasm-bindgen` function, causing failure.
                        if self.interp.skip_calls.contains(&func) {
                            continue;
                        }

                        // Skip profiling related functions which we don't want to interpret.
                        if self
                            .module
                            .funcs
                            .get(func)
                            .name
                            .as_ref()
                            .is_some_and(|name| {
                                name.starts_with("__llvm_profile_init")
                                    || name.starts_with("__llvm_profile_register_function")
                                    || name.starts_with("__llvm_profile_register_function")
                            })
                        {
                            continue;
                        }

                        let ty = self.module.types.get(self.module.funcs.get(func).ty());
                        let args = (0..ty.params().len())
                            .map(|_| stack.pop().unwrap())
                            .collect::<Vec<_>>();

                        self.interp.call(func, self.module, &args);
                    }

                    if let Instr::ReturnCall(_) = instr {
                        log::trace!("return_call");
                        break;
                    }
                }

                Instr::Block(block) => {
                    self.eval(block.seq)?;
                    if self.interp.stopped {
                        break;
                    }
                }

                Instr::Try(block) => {
                    self.eval(block.seq)?;
                    if self.interp.stopped {
                        break;
                    }
                }

                Instr::TryTable(block) => {
                    self.eval(block.seq)?;
                    if self.interp.stopped {
                        break;
                    }
                }

                // All other instructions shouldn't be used by our various
                // descriptor functions. LLVM optimizations may mean that some
                // of the above instructions aren't actually needed either, but
                // the above instructions have empirically been required when
                // executing our own test suite in wasm-bindgen.
                //
                // Note that LLVM may change over time to generate new
                // instructions in debug mode, and we'll have to react to those
                // sorts of changes as they arise.
                s => bail!("unknown instruction {s:?}"),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod smoke_tests;
