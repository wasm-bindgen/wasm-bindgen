use crate::descriptor::VectorKind;
use crate::transforms::externref::Context;
use crate::wit::{AdapterKind, Instruction, NonstandardWitSection};
use crate::wit::{AdapterType, InstructionData, StackChange, WasmBindgenAux};
use anyhow::Result;
use std::collections::HashMap;
use walrus::ElementItems;
use walrus::{ir::Value, ConstExpr, ElementKind, Module};

pub fn process(module: &mut Module) -> Result<()> {
    let mut cfg = Context::default();
    cfg.prepare(module)?;
    let section = module
        .customs
        .get_typed_mut::<NonstandardWitSection>()
        .expect("wit custom section should exist");

    let implements = section
        .implements
        .iter()
        .cloned()
        .map(|(core, _, adapter)| (adapter, core))
        .collect::<HashMap<_, _>>();

    // Transform all exported functions in the module, using the bindings listed
    // for each exported function.
    for (id, adapter) in &mut section.adapters {
        let instructions = match &mut adapter.kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => continue,
        };
        if let Some(id) = implements.get(id) {
            import_xform(
                &mut cfg,
                *id,
                instructions,
                &mut adapter.params,
                &mut adapter.results,
            );
            continue;
        }
        export_xform(&mut cfg, instructions);
    }

    let meta = cfg.run(module)?;

    let mut aux = module
        .customs
        .delete_typed::<WasmBindgenAux>()
        .expect("wit custom section should exist");
    let section = module
        .customs
        .get_typed_mut::<NonstandardWitSection>()
        .expect("wit custom section should exist");

    // If the module looks like it's going to use some of these exports, store
    // them in the aux section to get used.
    //
    // FIXME: this is not great, we should ideally have precise tracking of what
    // requires what. These are used by catch clauses and externref slices going
    // in/out of wasm. The catch clauses are a bit weird but externref slices
    // should ideally track in their own instructions what table/functions
    // they're referencing. This doesn't fit well in today's model of
    // slice-related instructions, though, so let's just cop out and only enable
    // these coarsely.
    aux.externref_table = Some(meta.table);
    if module_needs_externref_metadata(&aux, section) {
        aux.externref_alloc = meta.alloc;
        aux.externref_drop = meta.drop;
        aux.externref_drop_slice = meta.drop_slice;
    }

    // Additionally we may need to update some adapter instructions other than
    // those found for the externref pass. These are some general "fringe support"
    // things necessary to get absolutely everything working.
    for adapter in &mut section.adapters.values_mut() {
        let instrs = match &mut adapter.kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => continue,
        };
        for instr in instrs {
            match instr.instr {
                // Optional externref values are now managed in the Wasm module, so
                // we need to store where they're managed.
                Instruction::I32FromOptionExternref {
                    ref mut table_and_alloc,
                } => {
                    *table_and_alloc = meta.alloc.map(|id| (meta.table, id));
                }

                Instruction::ExternrefLoadOwned {
                    ref mut table_and_drop,
                }
                | Instruction::UnwrapResult {
                    ref mut table_and_drop,
                }
                | Instruction::UnwrapResultString {
                    ref mut table_and_drop,
                } => {
                    *table_and_drop = meta.drop.map(|id| (meta.table, id));
                }
                Instruction::CachedStringLoad { ref mut table, .. } => *table = Some(meta.table),
                _ => continue,
            };
        }
    }

    module.customs.add(*aux);

    Ok(())
}

// A helper to cleanup the adapter instruction list with separate
// `retain` callbacks for before and after the core call instruction.
// Returns the new call position.
fn retain_instrs(
    instrs: &mut Vec<InstructionData>,
    is_call: impl Fn(&Instruction) -> bool,
    mut retain_before: impl FnMut(&mut InstructionData) -> bool,
    mut retain_after: impl FnMut(&mut InstructionData) -> bool,
) -> Option<usize> {
    let mut call_idx = None;
    let mut i = 0;

    instrs.retain_mut(|instr| {
        let retain = if call_idx.is_some() {
            retain_after(instr)
        } else if is_call(&instr.instr) {
            call_idx = Some(i);
            true
        } else {
            retain_before(instr)
        };
        if retain {
            i += 1;
        }
        retain
    });

    call_idx
}

/// Adapts the `instrs` given which are an implementation of the import of `id`.
///
/// This function will pattern match outgoing arguments and update the
/// instruction stream to remove any externref-management instructions since
/// we'll be sinking those into the WebAssembly module.
fn import_xform(
    cx: &mut Context,
    id: walrus::ImportId,
    instrs: &mut Vec<InstructionData>,
    params: &mut [AdapterType],
    results: &mut [AdapterType],
) {
    struct Arg {
        idx: usize,
        // Some(false) for a borrowed externref, Some(true) for an owned one
        externref: Option<bool>,
    }

    let mut args = Vec::new();
    let mut ret_externref = false;

    // Gather args and delete all unnecessary externref management instructions.
    retain_instrs(
        instrs,
        |instr| matches!(instr, Instruction::CallAdapter(_)),
        |instr| match instr.instr {
            Instruction::ExternrefLoadOwned { .. } | Instruction::TableGet => {
                let owned = !matches!(instr.instr, Instruction::TableGet);
                let mut arg: Arg = match args.pop().unwrap() {
                    Some(arg) => arg,
                    None => panic!("previous instruction must be `arg.get`"),
                };
                arg.externref = Some(owned);
                match params[arg.idx] {
                    AdapterType::I32 => {}
                    _ => panic!("must be `i32` type"),
                }
                params[arg.idx] = AdapterType::Externref;
                args.push(Some(arg));
                false
            }
            Instruction::ArgGet(n) => {
                args.push(Some(Arg {
                    idx: n as usize,
                    externref: None,
                }));
                true
            }
            _ => match instr.stack_change {
                StackChange::Modified { pushed, popped } => {
                    for _ in 0..popped {
                        args.pop();
                    }
                    for _ in 0..pushed {
                        args.push(None);
                    }
                    true
                }
                StackChange::Unknown => {
                    panic!("must have stack change data");
                }
            },
        },
        |instr| {
            if matches!(instr.instr, Instruction::I32FromExternrefOwned) {
                assert_eq!(results.len(), 1);
                assert!(matches!(results[0], AdapterType::I32), "must be `i32` type");
                results[0] = AdapterType::Externref;
                ret_externref = true;
                false
            } else {
                true
            }
        },
    );

    // Filter down our list of arguments to just the ones that are externref
    // values.
    let args = args
        .iter()
        .filter_map(|arg| arg.as_ref())
        .filter_map(|arg| arg.externref.map(|owned| (arg.idx, owned)))
        .collect::<Vec<_>>();

    // ... and register this entire transformation with the externref
    // transformation pass.
    cx.import_xform(id, &args, ret_externref);
}

/// Adapts the `instrs` of an adapter function that calls an export.
///
/// The `instrs` must be generated by wasm-bindgen itself and follow the
/// pattern matched below to pass off to the externref transformation pass. The
/// signature of the adapter doesn't change (it remains as externref-aware) but the
/// signature of the export we're calling will change during the transformation.
fn export_xform(cx: &mut Context, instrs: &mut Vec<InstructionData>) {
    let mut args = Vec::new();
    let mut uses_retptr = false;
    let mut ret_externref = false;

    let call_idx = retain_instrs(
        instrs,
        |instr| {
            matches!(
                instr,
                Instruction::CallExport(_) | Instruction::CallTableElement(_)
            )
        },
        // Mutate instructions leading up to the `CallExport` instruction. We
        // maintain a stack of indicators whether the element at that stack slot is
        // unknown (`None`) or whether it's an owned/borrowed externref
        // (`Some(owned)`).
        //
        // We also want to delete the `I32FromExternref*` instructions.
        |instr| match instr.instr {
            Instruction::I32FromExternrefOwned => {
                args.pop();
                args.push(Some(true));
                false
            }
            Instruction::I32FromExternrefBorrow => {
                args.pop();
                args.push(Some(false));
                false
            }
            _ => match instr.stack_change {
                StackChange::Modified { pushed, popped } => {
                    for _ in 0..popped {
                        args.pop();
                    }
                    for _ in 0..pushed {
                        args.push(None);
                    }
                    true
                }
                StackChange::Unknown => {
                    panic!("must have stack change data");
                }
            },
        },
        // If one of the instructions after the call is an `ExternrefLoadOwned`,
        // and a retptr isn't used, the function must return an externref.
        // Currently `&'static Externref` can't be done as a return value,
        // so we don't need to handle that possibility.
        |instr| match instr.instr {
            Instruction::LoadRetptr { .. } => {
                uses_retptr = true;
                true
            }
            Instruction::ExternrefLoadOwned { .. } if !uses_retptr => {
                ret_externref = true;
                false
            }
            _ => true,
        },
    )
    .unwrap();

    // Filter down our list of arguments to just the ones that are externref
    // values.
    let args = args
        .iter()
        .enumerate()
        .filter_map(|(i, owned)| owned.map(|owned| (i, owned)))
        .collect::<Vec<_>>();

    // ... and register this entire transformation with the externref
    // transformation pass.
    match &mut instrs[call_idx].instr {
        Instruction::CallExport(id) => {
            cx.export_xform(*id, &args, ret_externref);
        }
        Instruction::CallTableElement(idx) => {
            if let Some(new_idx) = cx.table_element_xform(*idx, &args, ret_externref) {
                *idx = new_idx;
            }
        }
        _ => unreachable!(),
    }
}

/// This function shouldn't need to exist, see the fixme at the call-site.
fn module_needs_externref_metadata(aux: &WasmBindgenAux, section: &NonstandardWitSection) -> bool {
    use Instruction::*;

    // our `handleError` intrinsic uses a few pieces of metadata to store
    // indices directly into the Wasm module.
    if !aux.imports_with_catch.is_empty() {
        return true;
    }

    // Look for any instructions which may use `VectorKind::Externref`. If there
    // are any then we'll need our intrinsics/tables/etc, otherwise we shouldn't
    // ever need them.
    section.adapters.iter().any(|(_, adapter)| {
        let instructions = match &adapter.kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => return false,
        };
        instructions.iter().any(|instr| {
            matches!(
                instr.instr,
                VectorToMemory {
                    kind: VectorKind::Externref | VectorKind::NamedExternref(_),
                    ..
                } | MutableSliceToMemory {
                    kind: VectorKind::Externref | VectorKind::NamedExternref(_),
                    ..
                } | OptionVector {
                    kind: VectorKind::Externref | VectorKind::NamedExternref(_),
                    ..
                } | VectorLoad {
                    kind: VectorKind::Externref | VectorKind::NamedExternref(_),
                    ..
                } | OptionVectorLoad {
                    kind: VectorKind::Externref | VectorKind::NamedExternref(_),
                    ..
                } | View {
                    kind: VectorKind::Externref | VectorKind::NamedExternref(_),
                    ..
                } | OptionView {
                    kind: VectorKind::Externref | VectorKind::NamedExternref(_),
                    ..
                }
            )
        })
    })
}

/// In MVP Wasm all element segments must be contiguous lists of function
/// indices. Post-MVP with reference types element segments can have holes.
/// While `walrus` will select the encoding that fits, this function forces the
/// listing of segments to be MVP-compatible.
pub fn force_contiguous_elements(module: &mut Module) -> Result<()> {
    // List of new element segments we're going to be adding.
    let mut new_segments = Vec::new();

    // Here we take a look at all element segments in the module to see if we
    // need to split them.
    for segment in module.elements.iter_mut() {
        let (ty, items) = match &mut segment.items {
            ElementItems::Expressions(ty, items) => {
                // If this segment has no null reference members then it's already
                // contiguous and we can skip it.
                if items
                    .iter()
                    .all(|item| !matches!(item, ConstExpr::RefNull(_)))
                {
                    continue;
                }

                (*ty, items)
            }
            // Function index segments don't have holes.
            ElementItems::Functions(_) => continue,
        };

        // For now active segments are all we're interested in since
        // passive/declared have no hope of being MVP-compatible anyway.
        // Additionally we only handle active segments with i32 offsets, since
        // global offsets get funky since we'd need to add an offset.
        let (table, offset) = match &segment.kind {
            ElementKind::Active {
                table,
                offset: ConstExpr::Value(Value::I32(n)),
            } => (*table, *n),
            _ => continue,
        };

        // `block` keeps track of a block of contiguous segment of functions
        let mut block = None;
        // This keeps track of where we're going to truncate the current segment
        // after we split out all the blocks.
        let mut truncate = 0;
        // This commits a block of contiguous functions into the `new_segments`
        // list, accounting for the new offset which is relative to the old
        // offset.
        let mut commit = |last_idx: usize, block: Vec<_>| {
            let new_offset = offset + (last_idx - block.len()) as i32;
            let new_offset = ConstExpr::Value(Value::I32(new_offset));
            new_segments.push((table, new_offset, ty, block));
        };
        for (i, expr) in items.iter().enumerate() {
            match expr {
                ConstExpr::RefNull(_) => {
                    let block: Vec<_> = match block.take() {
                        Some(b) => b,
                        None => continue,
                    };
                    // If this is the first block (truncate isn't set and the
                    // length of the block means it starts from the beginning),
                    // then we leave it in the original list and don't commit
                    // anything, we'll just edit where to truncate later.
                    // Otherwise we commit this block to the new segment list.
                    if truncate == 0 && block.len() == i {
                        truncate = i;
                    } else {
                        commit(i, block);
                    }
                }
                // If we find a function, then we either start a new block or
                // push it onto the existing block.
                _ => block.get_or_insert(Vec::new()).push(*expr),
            }
        }

        // If there's no trailing empty slots then we commit the last block onto
        // the new segment list.
        if let Some(block) = block {
            commit(items.len(), block);
        }
        items.truncate(truncate);
    }

    for (table, offset, ty, members) in new_segments {
        let id = module.elements.add(
            ElementKind::Active { table, offset },
            ElementItems::Expressions(ty, members),
        );
        module.tables.get_mut(table).elem_segments.insert(id);
    }
    Ok(())
}
