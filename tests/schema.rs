//! Unit tests for the reference-based `wasm_bindgen::describe::Schema`
//! tree — the sole canonical descriptor ABI.
//!
//! These run on the host (`cargo test --test schema`). They deliberately
//! exercise `Schema::words()` / `Schema::children()` at *runtime* (not just
//! const-eval), which is where the base-pointer provenance matters: an empty
//! `words`/`children` run must not dereference its dangling base, and a
//! non-empty run must be reconstructed with provenance over the whole run
//! (so it is sound under Miri / Stacked-Tree Borrows, not just CTFE).
//!
//! The CLI walks this tree structurally (scalars from `words`, sub-descriptors
//! from `children`). The host-side `flatten_oracle` below mirrors a pre-order
//! walk of that structure and is used to assert each tree composes as the
//! macro/runtime intend.

#![cfg(not(target_family = "wasm"))]

extern crate wasm_bindgen;

use wasm_bindgen::describe::{
    Schema, SchemaTag, WasmDescribe, CLAMPED, EXTERNREF, FUNCTION, I32, OPTIONAL, REF, REFMUT,
    RESULT, SLICE, UNIT,
};

/// Pre-order walk of a `Schema` tree's `words` then its children's words,
/// via the public `words()` / `children()` accessors — so it also stresses
/// their provenance at arbitrary depth at runtime.
fn flatten_oracle(s: &Schema) -> Vec<u32> {
    let mut out = s.words().to_vec();
    for child in s.children() {
        out.extend(flatten_oracle(child));
    }
    out
}

#[test]
fn schema_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Schema>();
}

#[test]
fn leaf_exposes_words_and_no_children() {
    const LEAF: &Schema = &Schema::leaf(&[I32, EXTERNREF]);
    assert_eq!(LEAF.tag, SchemaTag::Leaf);
    assert_eq!(LEAF.words(), &[I32, EXTERNREF]);
    assert!(LEAF.children().is_empty());
    assert_eq!(flatten_oracle(LEAF), [I32, EXTERNREF]);
}

#[test]
fn empty_leaf_has_empty_runs() {
    // Regression: an empty `words` run must yield `&[]` without touching its
    // dangling base pointer. Calling the accessors twice also checks the base
    // pointer is stable.
    const EMPTY: &Schema = &Schema::leaf(&[]);
    assert_eq!(EMPTY.tag, SchemaTag::Leaf);
    assert!(EMPTY.words().is_empty());
    assert!(EMPTY.words().is_empty());
    assert!(EMPTY.children().is_empty());
    assert_eq!(flatten_oracle(EMPTY), [] as [u32; 0]);
}

#[test]
fn wrap_node_exposes_words_then_children() {
    const INNER: &Schema = &Schema::leaf(&[I32]);
    const WRAP: &Schema = &Schema::node(SchemaTag::Wrap, &[OPTIONAL], &[INNER]);

    assert_eq!(WRAP.tag, SchemaTag::Wrap);
    assert_eq!(WRAP.words(), &[OPTIONAL]);
    assert_eq!(WRAP.children().len(), 1);
    assert_eq!(WRAP.children()[0].words(), &[I32]);
    assert_eq!(flatten_oracle(WRAP), [OPTIONAL, I32]);
}

#[test]
fn wrap_node_with_empty_words_concatenates_children() {
    // A `Wrap` node with an *empty* `words` run plus children — the exact
    // shape that an element-0 base-pointer bug would have tripped both at
    // runtime and in the CLI walker.
    const A: &Schema = &Schema::leaf(&[I32]);
    const B: &Schema = &Schema::leaf(&[EXTERNREF]);
    const CONCAT: &Schema = &Schema::node(SchemaTag::Wrap, &[], &[A, B]);

    assert_eq!(CONCAT.tag, SchemaTag::Wrap);
    assert!(CONCAT.words().is_empty());
    assert_eq!(CONCAT.children().len(), 2);
    assert_eq!(flatten_oracle(CONCAT), [I32, EXTERNREF]);
}

#[test]
fn function_shaped_node_structure() {
    // Models a function/cast tree: [FUNCTION, shim_idx, nargs] header words
    // followed by arg/ret child schemas.
    const I: &Schema = &Schema::leaf(&[I32]);
    const FUNC: &Schema = &Schema::node(SchemaTag::Wrap, &[FUNCTION, 0, 1], &[I, I, I]);

    assert_eq!(FUNC.words(), &[FUNCTION, 0, 1]);
    assert_eq!(FUNC.children().len(), 3);
    assert_eq!(flatten_oracle(FUNC), [FUNCTION, 0, 1, I32, I32, I32]);
}

#[test]
fn closure_node_carries_invoke() {
    // A closure-bearing FUNCTION node stores the invoke shim address in its
    // out-of-band `invoke` field (not in `words`). Here we just assert the
    // constructor accepts and the structure is preserved; the address is a
    // dummy non-null pointer (never dereferenced on the host).
    const I: &Schema = &Schema::leaf(&[I32]);
    let invoke = 0x1234 as *const ();
    let func = Schema::closure_node(SchemaTag::Wrap, &[FUNCTION, 0, 1], &[I, I, I], invoke);
    assert_eq!(func.words(), &[FUNCTION, 0, 1]);
    assert_eq!(func.children().len(), 3);

    // `with_invoke` copies a node's structure but attaches an invoke address.
    let base = Schema::node(SchemaTag::Wrap, &[FUNCTION, 0, 1], &[I, I, I]);
    let leaked: &'static Schema = Box::leak(Box::new(base));
    let with = Schema::with_invoke(leaked, invoke);
    assert_eq!(with.words(), &[FUNCTION, 0, 1]);
    assert_eq!(with.children().len(), 3);
}

#[test]
fn deeply_nested_tree_walks_in_preorder() {
    // Build an 8-deep right-spine of WRAP(REF) nodes around an I32 leaf and
    // confirm the pre-order walk yields the expected sequence. This stresses
    // recursion + runtime provenance at depth.
    const L0: &Schema = &Schema::leaf(&[I32]);
    const L1: &Schema = &Schema::node(SchemaTag::Wrap, &[REF], &[L0]);
    const L2: &Schema = &Schema::node(SchemaTag::Wrap, &[REF], &[L1]);
    const L3: &Schema = &Schema::node(SchemaTag::Wrap, &[REF], &[L2]);
    const L4: &Schema = &Schema::node(SchemaTag::Wrap, &[REF], &[L3]);
    const L5: &Schema = &Schema::node(SchemaTag::Wrap, &[REF], &[L4]);
    const L6: &Schema = &Schema::node(SchemaTag::Wrap, &[REF], &[L5]);
    const L7: &Schema = &Schema::node(SchemaTag::Wrap, &[REF], &[L6]);

    assert_eq!(
        flatten_oracle(L7),
        vec![REF; 7].into_iter().chain([I32]).collect::<Vec<_>>()
    );
}

#[test]
fn node_with_multiple_children_and_words() {
    // Multiple children combined with a multi-word header.
    const A: &Schema = &Schema::leaf(&[I32]);
    const B: &Schema = &Schema::node(SchemaTag::Wrap, &[OPTIONAL], &[A]);
    const ROOT: &Schema = &Schema::node(SchemaTag::Wrap, &[SLICE, REFMUT], &[A, B, A]);

    assert_eq!(flatten_oracle(ROOT), [SLICE, REFMUT, I32, OPTIONAL, I32, I32]);
}

// --- Tests over the real `WasmDescribe` impls, which compose `Schema`
// --- exactly the way the macro relies on. ---

/// Pre-order walk of a concrete type's `WasmDescribe::SCHEMA`.
fn describe_flat<T: WasmDescribe + ?Sized>() -> Vec<u32> {
    flatten_oracle(T::SCHEMA)
}

#[test]
fn primitive_describe_is_a_single_leaf() {
    assert_eq!(<i32 as WasmDescribe>::SCHEMA.tag, SchemaTag::Leaf);
    assert_eq!(describe_flat::<i32>(), [I32]);
    assert_eq!(describe_flat::<()>(), [UNIT]);
}

#[test]
fn wrapper_describe_composes_by_reference() {
    assert_eq!(<Option<i32> as WasmDescribe>::SCHEMA.tag, SchemaTag::Wrap);
    assert_eq!(describe_flat::<Option<i32>>(), [OPTIONAL, I32]);
    assert_eq!(describe_flat::<&i32>(), [REF, I32]);
    assert_eq!(describe_flat::<&mut i32>(), [REFMUT, I32]);
    assert_eq!(describe_flat::<[i32]>(), [SLICE, I32]);
    assert_eq!(describe_flat::<Clamped>(), [CLAMPED, I32]);
}

// Local alias so the `Clamped` describe path above reads cleanly.
type Clamped = wasm_bindgen::Clamped<i32>;

#[test]
fn nested_wrapper_describe_walks_preorder() {
    // Option<&i32> -> [OPTIONAL, REF, I32]
    assert_eq!(describe_flat::<Option<&i32>>(), [OPTIONAL, REF, I32]);
    // Result<&mut i32, JsValue> -> [RESULT, REFMUT, I32]
    assert_eq!(
        describe_flat::<Result<&mut i32, wasm_bindgen::JsValue>>(),
        [RESULT, REFMUT, I32]
    );
}
