# Plan: Reference-Based Schemas + `breaks_if_inlined` Removal

## Background

On the `descriptors-via-custom-section` branch, every `#[wasm_bindgen]` shim emits
its type descriptor at compile time as bytes in the `__wasm_bindgen_descriptors`
custom section, which `wasm-bindgen-cli` reads structurally (no wasm interpreter).

To sidestep `generic_const_exprs` (rust-lang/rust#76560), every `WasmDescribe` impl
currently carries its schema as a fixed-size buffer plus a length:

```rust
trait WasmDescribe {
    const SCHEMA_LEN: usize;
    const SCHEMA_BUF: [u32; SCHEMA_MAX]; // SCHEMA_MAX = 256
}
```

Wrapper types (`Option<T>`, `Vec<T>`, `&T`, closures, ...) compose into the same
fixed buffer via `const fn`s, which avoids needing generic-length const arrays.

Two costs remain:

1. A hard **256-word cap** per schema (`SCHEMA_MAX`); overflow is a compile error.
2. The closure-cast path (`src/rt/mod.rs`) takes the address of two
   `[u32; 256]` buffers (`FromBuf`/`ToBuf`), shipping ~1 KB each per cast
   monomorphization into the pre-bindgen wasm (stripped later by the CLI, but
   still real toolchain bloat).

Separately, closure casts rely on `breaks_if_inlined*` — a per-monomorphization
helper function whose body calls `__wbindgen_describe_cast` with **five
`i32.const` immediates** that the CLI recovers by scanning the function body.
This is the most fragile part of the system and is propped up by
optimizer-fighting code (`keep_prims_alive`, `make_ret`, `black_box` keepalive,
`#[inline(never)]`, archive-pull anchoring, debug-vs-release scanner shapes).

## Goals

1. Replace `[u32; SCHEMA_MAX]` with a reference-based `Schema` tree, removing the
   256-word cap.
2. Eliminate the `FromBuf`/`ToBuf` scratch buffers (the real bloat target).
3. Reduce the closure-cast helper to a dumb, data-driven trampoline and delete all
   optimizer-fighting code.

The shipped `__wasm_bindgen_descriptors` wire format and `Descriptor::decode` stay
**unchanged**; only schema *composition* (runtime) and the *cast path* change.

## Design

### `Schema` node

`#[repr(C)]` for deterministic CLI parsing; composed by reference so no array
length ever depends on a generic parameter (hence no `generic_const_exprs`):

```rust
#[repr(C)]
pub struct Schema {
    tag: u32,                            // SCHEMA_NODE_LEAF / WRAP / CAT
    words: &'static u32,                 // base of a words_len-long run of opcodes
    words_len: usize,
    children: &'static &'static Schema,  // base of a children_len-long run
    children_len: usize,
}

// const builders:
//   leaf(&[u32]) -> Schema
//   wrap(&[u32], &'static Schema) -> Schema
//   cat(&[&'static Schema]) -> Schema

pub trait WasmDescribe { const SCHEMA: &'static Schema; }
pub trait WasmDescribeVector { const VECTOR_SCHEMA: &'static Schema; }
```

#### Why `&'static` (not `*const`) for the structural fields

- `*const T` and `&'static T` are **ABI-identical** (a single pointer-sized word
  with the same relocation), so `#[repr(C)]` parsing works equally with either —
  raw pointers buy nothing for determinism.
- `&'static` makes the borrow checker guarantee the pointee lives forever and
  keeps it **anchored against linker GC** — directly mitigating Phase 0 risk #3.
  A raw `*const` obtained via `slice.as_ptr()` carries no such guarantee and is
  more likely to be collected out from under the CLI.
- The explicit `words_len` / `children_len` fields are retained **only** because a
  Rust slice reference (`&'static [T]`) is a fat pointer whose field order is not
  `#[repr(C)]`-stable. A thin base pointer + explicit `usize` length is fully
  layout-determined for the CLI parser. Empty runs use `*_len == 0` (no null
  needed).
- `CastRecord.invoke` (below) deliberately keeps `*const ()`: it is genuinely
  nullable (plain casts) and holds a fn-pointer that becomes a table-index
  relocation, which `&'static` cannot express.

Composition examples:

```rust
impl WasmDescribe for i32 {
    const SCHEMA: &'static Schema = &Schema::leaf(&[I32]);
}
impl<T: WasmDescribe> WasmDescribe for Option<T> {
    const SCHEMA: &'static Schema = &Schema::wrap(&[OPTIONAL], T::SCHEMA);
}
```

### `CastRecord`

Replaces the five scanned immediates for closure casts with one referenced record:

```rust
#[repr(C)]
pub struct CastRecord {
    from:   &'static Schema,
    to:     &'static Schema,
    invoke: *const (), // closures: invoke shim addr; plain casts: null
}
```

`invoke` is stored as a **pointer**, not a const integer. `fn as *const ()` is
const-eval-able on stable (pointer cast, preserves provenance), and the linker
emits a table-index relocation that fills the real function-table slot into the
data segment. This removes the const fn-ptr-to-int problem and the scanned
`invoke_addr` immediate. The CLI reads the slot from the data segment.

## Execution Phases

### Phase 0 — Spike (BLOCKING go/no-go gate)

Validate the load-bearing assumptions with throwaway compiles before touching real
impls. Stop and report findings before proceeding.

1. rvalue static promotion of `&Schema::wrap(&[OPTIONAL], T::SCHEMA)` for generic
   wrappers (e.g. `Option<Vec<&i32>>`, deeply nested) on stable 1.79.
2. `invoke: T::invoke as *const ()` is const-eval-able AND yields a readable
   table-index relocation in the data segment (wasm32 + wasm64).
3. `--release` linker keeps the transitive `CastRecord` -> `Schema` graph
   (referenced via a live call argument). Specifically confirm that a
   `&'static u32` / `&'static &'static Schema` to element 0 anchors the *whole*
   run against GC (symbol/segment-granularity GC suggests yes), and that the
   `&'static &'static Schema` children-array form promotes cleanly. If the
   base-pointer-to-array form proves awkward, fall back to `*const` with explicit
   `#[used]`/anchoring — chosen deliberately, not by default.
4. CLI can derive the import Wasm type from the `From`/`To` descriptors, matching
   rustc ABI lowering across zero-sized / `()` / multivalue / 64-bit cases.
   **This gates Layer 2.**

If #4 fails: downshift to Layer-1-only (keep a minimal signature guard). Keep the
Layer-1 fallback scoped in the same branch for a quick downshift.

### Phase 1 — Runtime schema representation

- `src/describe.rs`: delete `SCHEMA_MAX`, `SCHEMA_BUF`, `SCHEMA_LEN`,
  `leaf_schema`, `wrap_schema`, `cat_schema`, `schema_from_slice`. Add `Schema` +
  `const fn` builders + `flatten_len(&Schema) -> usize` and
  `flatten_into::<const N>(&Schema) -> [u32; N]`. Change the trait to
  `const SCHEMA: &'static Schema`. Rewrite all leaf/wrapper impls (`&T`, `&mut T`,
  `[T]`, `Option<T>`, `Result<T,E>`, `Clamped<T>`, `MaybeUninit<T>`, `NonNull`,
  raw pointers, `()`, `JsError`, `AssertUnwindSafe`, isize/usize/str/String
  variants).
- `src/convert/closures.rs`, `src/closure.rs`: rebuild the closure-trait-object,
  `OwnedClosure`, `BorrowedClosure`, and `ScopedClosure` schemas via
  `cat`/`wrap`/`leaf`.
- `crates/shared/src/tys.rs`: add `SCHEMA_NODE_LEAF/WRAP/CAT` tag constants shared
  by producer and consumer.

### Phase 2 — Macro emission (concrete sites)

- `crates/macro-support/src/codegen.rs`: in `emit_static_descriptor_entry` and
  `emit_static_descriptor_entry_static`, replace the `[u32; SCHEMA_MAX]` scratch +
  `cat_schema` + `split_at` with a const flatten over the now-concrete `T::SCHEMA`
  tree:
  - `const __WORDS: usize = flatten_len(<T>::SCHEMA);` (concrete type, legal on stable)
  - `pack_entry::<{entry_byte_len(name_len, __WORDS)}>(name, kind, &flatten_into::<__WORDS>(<T>::SCHEMA))`
- `schema_parts_for_type` and friends return a single `&'static Schema` expression
  instead of `(len, buf)` pairs.

### Phase 3 — Cast trampoline refactor (Layers 1+2)

- `src/rt/mod.rs`:
  - Delete `FromBuf`, `ToBuf`, `keep_prims_alive`, `make_ret`, and the
    `black_box`/`_keepalive` guards.
  - Replace `breaks_if_inlined` / `breaks_if_inlined_closure` with a minimal
    trampoline `__wbg_cast_trampoline::<From, To, T, UW>(prims) -> WasmRet<To::Abi>`:
    real signature (genuine prim/ret data flow) + body
    `__wbindgen_cast_marker(&RECORD as *const _ as *const ())`.
  - Add `RECORD: &'static CastRecord` as a monomorphized associated const;
    `invoke` from `T::invoke_addr::<UW>()` for closures, null otherwise.
- `src/lib.rs`: replace the `__wbindgen_describe_cast` import with
  `__wbindgen_cast_marker(record: *const ())`.

### Phase 4 — CLI consumer

- `crates/cli-support/src/descriptors.rs`:
  - `execute_casts`: scan for the single record-pointer operand of
    `__wbindgen_cast_marker` (tolerate debug `local.set`/`local.get` round-trips).
  - `DataSegmentView`: add typed reads + `read_schema_tree(root_ptr)` that walks
    `Schema` nodes and emits the flat `u32` stream (byte-identical to today),
    feeding the unchanged `compose_cast_descriptor` + `Descriptor::decode`.
  - Read `CastRecord` (3 fields) -> from/to roots + relocated `invoke` slot.
- `crates/cli-support/src/wit/mod.rs`: derive the manufactured `__wbindgen_cast_N`
  import type from the descriptor (Layer 2) instead of cloning the trampoline's
  emitted `.ty()`; redirect calls as today via `handle_duplicate_imports`.

### Phase 5 — Validate & finalize

- Re-bless `crates/cli/tests/reference/*` (`.wat` / `.bg.js`).
- Run the full suite: `crates/cli/tests/wasm-bindgen`, runtime tests, wasm32 +
  wasm64, and `--release`.
- Measure pre-bindgen `.wasm` size delta to confirm the bloat win.
- Update the now-inaccurate doc comments in `descriptors.rs` and `CHANGELOG.md`
  (GCE is no longer the blocker; the fixed buffers are gone).

## What Survives and Why

A per-monomorphization **defined trampoline** must remain, because:

- You cannot declare a generic `extern` import, so generic runtime code cannot call
  a per-`(From, To)`-typed import directly — a monomorphized defined function is the
  only way to materialize that per-signature Wasm type.
- You cannot emit a `#[link_section]` static from generic code, so a closure cast
  cannot become an ordinary section entry.

But it becomes a dumb, data-driven shim: all fragility (5-immediate scanning,
`keep_prims_alive` / `make_ret` / `black_box`, signature preservation, archive-pull
anchoring of cast buffers) is removed, replaced by static-data reads + linker
relocations.

## Residual Risks

- **Layer 2 import-typing parity** with rustc lowering (Phase 0 #4) — the riskiest
  piece; Layer-1 fallback exists.
- **Const fn-ptr-as-pointer + table-index relocation** readability (Phase 0 #2).
- **Linker GC** of the record/tree graph under optimization (Phase 0 #3).
- **Const recursion depth / eval limits** for deeply nested generics.

## Decisions

- Refactor depth: **full Layers 1+2** (CastRecord indirection + CLI-derived import
  typing), with a Layer-1-only fallback kept scoped in the branch.
- Marker transport: **single pointer argument** to `__wbindgen_cast_marker`.
- Phase 0 is a **hard blocking gate**; report spike results before continuing.
