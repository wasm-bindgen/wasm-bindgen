//! The `wasm-bindgen` multi-value transformation.
//!
//! This crate provides a transformation to turn exported functions that use a
//! return pointer into exported functions that use multi-value.
//!
//! Consider the following function:
//!
//! ```
//! #[no_mangle]
//! pub extern "C" fn pair(a: u32, b: u32) -> [u32; 2] {
//!     [a, b]
//! }
//! ```
//!
//! LLVM will by default compile this down into the following Wasm:
//!
//! ```wasm
//! (func $pair (param i32 i32 i32)
//!   local.get 0
//!   local.get 2
//!   i32.store offset=4
//!   local.get 0
//!   local.get 1
//!   i32.store)
//! ```
//!
//! What's happening here is that the function is not directly returning the
//! pair at all, but instead the first `i32` parameter is a pointer to some
//! scratch space, and the return value is written into the scratch space. LLVM
//! does this because it doesn't yet have support for multi-value Wasm, and so
//! it only knows how to return a single value at a time.
//!
//! Ideally, with multi-value, what we would like instead is this:
//!
//! ```wasm
//! (func $pair (param i32 i32) (result i32 i32)
//!   local.get 0
//!   local.get 1)
//! ```
//!
//! However, that's not what this transformation does at the moment. This
//! transformation is a little simpler than mutating existing functions to
//! produce a multi-value result, instead it introduces new functions that wrap
//! the original function and translate the return pointer to multi-value
//! results in this wrapper function.
//!
//! With our running example, we end up with this:
//!
//! ```wasm
//! ;; The original function.
//! (func $pair (param i32 i32 i32)
//!   local.get 0
//!   local.get 2
//!   i32.store offset=4
//!   local.get 0
//!   local.get 1
//!   i32.store)
//!
//! (func $pairWrapper (param i32 i32) (result i32 i32)
//!   ;; Our return pointer that points to the scratch space we are allocating
//!   ;; on the stack for calling `$pair`.
//!   (local i32)
//!
//!   ;; Allocate space on the stack for the result.
//!   global.get $stackPointer
//!   i32.const 8
//!   i32.sub
//!   local.tee 2
//!   global.set $stackPointer
//!
//!   ;; Call `$pair` with our allocated stack space for its results.
//!   local.get 2
//!   local.get 0
//!   local.get 1
//!   call $pair
//!
//!   ;; Copy the return values from the stack to the Wasm stack.
//!   local.get 2
//!   i32.load
//!   local.get 2 offset=4
//!   i32.load
//!
//!   ;; Finally, restore the stack pointer.
//!   local.get 2
//!   i32.const 8
//!   i32.add
//!   global.set $stackPointer)
//! ```
//!
//! This `$pairWrapper` function is what we actually end up exporting instead of
//! `$pair`.

#![deny(missing_docs, missing_debug_implementations)]

use anyhow::Context;

/// Run the transformation.
///
/// See the module-level docs for details on the transformation.
///
/// * `memory` is the module's memory that has the stack where return
///   pointers are allocated within.
///
/// * `__stack_pointer` is the global that is being used as the stack
///   pointer. With LLVM, this is typically the first global.
///
/// * `to_xform` is the set of exported functions we want to transform and
///   information required to transform them. The `usize` is the index of the
///   return pointer parameter that will be removed. The `Vec<walrus::ValType>`
///   is the new result type that will be returned directly instead of via the
///   return pointer.
///
/// Returns a list of wrappers which have multi value signatures and call the
/// corresponding element in the `to_xform` list.
pub fn run(
    module: &mut walrus::Module,
    memory: walrus::MemoryId,
    stack_pointer: walrus::GlobalId,
    to_xform: &[(walrus::FunctionId, usize, Vec<walrus::ValType>)],
) -> Result<Vec<walrus::FunctionId>, anyhow::Error> {
    // Insert multi-value to the target features section.
    crate::wasm_conventions::insert_target_feature(module, "multivalue")
        .context("failed to parse `target_features` custom section")?;

    let mut wrappers = Vec::new();
    for (func, return_pointer_index, results) in to_xform {
        wrappers.push(xform_one(
            module,
            memory,
            stack_pointer,
            *func,
            *return_pointer_index,
            results,
        )?);
    }
    Ok(wrappers)
}

// Ensure that `n` is aligned to `align`, rounding up as necessary.
fn round_up_to_alignment(n: u32, align: u32) -> u32 {
    debug_assert!(align.is_power_of_two());
    (n + align - 1) & !(align - 1)
}

fn xform_one(
    module: &mut walrus::Module,
    memory: walrus::MemoryId,
    stack_pointer: walrus::GlobalId,
    func: walrus::FunctionId,
    return_pointer_index: usize,
    results: &[walrus::ValType],
) -> Result<walrus::FunctionId, anyhow::Error> {
    if module.globals.get(stack_pointer).ty != walrus::ValType::I32 {
        anyhow::bail!("stack pointer global does not have type `i32`");
    }

    // Compute the total size of all results, potentially with padding to ensure
    // that each result is aligned.
    let mut results_size = 0;
    for ty in results {
        results_size = match ty {
            walrus::ValType::I32 | walrus::ValType::F32 => {
                debug_assert_eq!(results_size % 4, 0);
                results_size + 4
            }
            walrus::ValType::I64 | walrus::ValType::F64 => {
                round_up_to_alignment(results_size, 8) + 8
            }
            walrus::ValType::V128 => round_up_to_alignment(results_size, 16) + 16,
            walrus::ValType::Ref(_) => anyhow::bail!(
                "cannot multi-value transform functions that return \
                     reference types, since they can't go into linear memory"
            ),
        };
    }
    // Round up to 16-byte alignment, since that's what LLVM's emitted Wasm code
    // seems to expect.
    let results_size = round_up_to_alignment(results_size, 16);

    let ty = module.funcs.get(func).ty();
    let (ty_params, ty_results) = module.types.params_results(ty);

    if !ty_results.is_empty() {
        anyhow::bail!(
            "can only multi-value transform functions that don't return any \
             results (since they should be returned on the stack via a pointer)"
        );
    }

    match ty_params.get(return_pointer_index) {
        Some(walrus::ValType::I32) => {}
        None => anyhow::bail!("the return pointer parameter doesn't exist"),
        Some(_) => anyhow::bail!("the return pointer parameter is not `i32`"),
    }

    let new_params: Vec<_> = ty_params
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(i, ty)| {
            if i == return_pointer_index {
                None
            } else {
                Some(ty)
            }
        })
        .collect();

    // The locals for the function parameters.
    let params: Vec<_> = new_params.iter().map(|ty| module.locals.add(*ty)).collect();

    // A local to hold our stack-allocated return pointer.
    let return_pointer = module.locals.add(walrus::ValType::I32);

    let mut wrapper = walrus::FunctionBuilder::new(&mut module.types, &new_params, results);
    let mut body = wrapper.func_body();

    // Allocate space in the stack for the call.
    body.global_get(stack_pointer)
        .i32_const(results_size as i32)
        .binop(walrus::ir::BinaryOp::I32Sub)
        .local_tee(return_pointer)
        .global_set(stack_pointer);

    // Push the parameters for calling our wrapped function -- including the
    // return pointer! -- on to the stack.
    for (i, local) in params.iter().enumerate() {
        if i == return_pointer_index {
            body.local_get(return_pointer);
        }
        body.local_get(*local);
    }
    if return_pointer_index == params.len() {
        body.local_get(return_pointer);
    }

    // Call our wrapped function.
    body.call(func);

    // Copy the return values from our stack-allocated space and onto the Wasm stack.
    let mut offset = 0;
    for ty in results {
        debug_assert!(offset < results_size);
        body.local_get(return_pointer);
        match ty {
            walrus::ValType::I32 => {
                debug_assert_eq!(offset % 4, 0);
                body.load(
                    memory,
                    walrus::ir::LoadKind::I32 { atomic: false },
                    walrus::ir::MemArg { align: 4, offset },
                );
                offset += 4;
            }
            walrus::ValType::I64 => {
                offset = round_up_to_alignment(offset, 8);
                body.load(
                    memory,
                    walrus::ir::LoadKind::I64 { atomic: false },
                    walrus::ir::MemArg { align: 8, offset },
                );
                offset += 8;
            }
            walrus::ValType::F32 => {
                debug_assert_eq!(offset % 4, 0);
                body.load(
                    memory,
                    walrus::ir::LoadKind::F32,
                    walrus::ir::MemArg { align: 4, offset },
                );
                offset += 4;
            }
            walrus::ValType::F64 => {
                offset = round_up_to_alignment(offset, 8);
                body.load(
                    memory,
                    walrus::ir::LoadKind::F64,
                    walrus::ir::MemArg { align: 8, offset },
                );
                offset += 8;
            }
            walrus::ValType::V128 => {
                offset = round_up_to_alignment(offset, 16);
                body.load(
                    memory,
                    walrus::ir::LoadKind::V128,
                    walrus::ir::MemArg { align: 16, offset },
                );
                offset += 16;
            }
            walrus::ValType::Ref(_) => unreachable!(),
        }
    }

    // Finally, restore the stack pointer.
    body.local_get(return_pointer)
        .i32_const(results_size as i32)
        .binop(walrus::ir::BinaryOp::I32Add)
        .global_set(stack_pointer);

    let wrapper = wrapper.finish(params, &mut module.funcs);
    if let Some(name) = &module.funcs.get(func).name {
        module.funcs.get_mut(wrapper).name = Some(format!("{name} multivalue shim"));
    }

    Ok(wrapper)
}

#[cfg(test)]
mod tests;
