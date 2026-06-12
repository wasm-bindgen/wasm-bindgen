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
3. Reduce the closure-cast helper to a data-driven trampoline: replace the five
   scanned `i32.const` immediates with one referenced `CastRecord`, and delete the
   `FromBuf`/`ToBuf` scratch buffers and the 5-immediate scanner.

The shipped `__wasm_bindgen_descriptors` wire format and `Descriptor::decode` stay
**unchanged**; only schema *composition* (runtime) and the *cast path* change.

**Scope (per Phase 0 decision): Layer 1 only.** The manufactured cast import keeps
deriving its Wasm signature by cloning the trampoline's emitted `.ty()`, so a
minimal signature guard stays on the trampoline. Layer 2 (CLI-derived import typing
from descriptors) is deferred to a follow-up — Phase 0 #4 showed it is achievable
but parity-heavy, and is not required to land the bloat/cap wins.

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

### Phase 0 — Spike (COMPLETE — gate passed, GO)

Validated the load-bearing assumptions with throwaway compiles (stable
`rustc 1.96.0`, `wasm-tools 1.252.0`; wasm64 via `nightly -Z build-std`). Every
data segment was decoded to verify structure, not just compilation.

**#1 — Generic Schema-tree static promotion → PASS.**
`const SCHEMA: &'static Schema = &Schema::node(WRAP, &[OPTIONAL], &[T::SCHEMA])`
compiles and promotes to `'static` for generic wrappers on stable with no
`generic_const_exprs`. Decoded `Option<Vec<&i32>>`, a cat node, and a 7-deep nest:
all byte-correct under `#[repr(C)]` and survived `--release`+LTO.

- **Design correction (folded into Phase 1):** the originally-proposed
  `wrap(words, child)` / `cat(children)` builders are **not implementable** — you
  cannot promote a 1-element children array *inside* a `const fn` (`E0515`,
  returns reference to temporary). Promotion must happen at the **call site** via
  an array literal. Use a single general builder
  `Schema::node(tag, words: &'static [u32], children: &'static [&'static Schema])`
  and write `&[T::SCHEMA]` at each impl site. Empty runs use a `'static` sentinel
  (`EMPTY_WORD` / `EMPTY_CHILD_PTR`) so base pointers are always valid; `*_len == 0`
  still marks emptiness.

**#2 — `invoke as *const ()` const-eval + table-index relocation → PASS (wasm32 AND wasm64).**
`fn as *const ()` is const-eval-able inside a `#[repr(C)]` record stored in a
`'static`. Decoded data segment confirmed the closure record's `invoke` field holds
the resolved **table index** (matching the `elem` segment), and `null` for plain
casts. wasm64 behaved identically with 8-byte pointers / `i64` table.

- **CLI note:** the data-segment parser must be pointer-width-aware. On wasm64,
  `Schema`/`CastRecord` carry 4 bytes of padding between the `u32 tag` and the first
  8-byte-aligned pointer; `read_schema_tree` must honor target alignment.

**#3 — `--release` linker GC of the record→schema graph → PASS.**
Modeled the real trampoline pattern (body = `i32.const <record>; call
__wbindgen_cast_marker`, no keepalive tricks). Under `--release`+LTO the **whole
transitive graph survived**, including multi-word leaf runs and multi-child cat runs,
with only element-0 base pointers referenced. wasm-ld GC is symbol-granular and
`&'static` anchoring suffices — the `*const` + `#[used]` fallback is **not needed**.

**#4 — CLI deriving import type from descriptors (gates Layer 2) → ACHIEVABLE, but the dominant risk.**
A 15-pair `(From, To)` trampoline matrix confirmed zero-sized/`()`, 64-bit
(`i64`/`u64`→`i64`), and multi-prim→sret/multivalue returns are all derivable from
descriptors. **Catch:** derivation must replicate type-specific niche ABI rules —
e.g. `Option<i32>` lowers to a single **`f64`** prim (sentinel niche,
`src/convert/impls.rs:194`) while `Option<f64>`→`(i32, f64)` and
`Option<i64>`→`(i32, i64)`. The inner type is present in the descriptor so it is
recoverable, but the CLI would have to maintain a hand-written parity table
mirroring every `IntoWasmAbi::Abi` / `WasmAbi` impl (incl. memory64 variants).

- **Also confirmed:** under `--release` the optimizer mangles trampoline signatures
  when the body lacks genuine prim/ret data flow (observed `Option<i32>` params
  collapsing to `f64` junk). This is *why* the status quo needs `keep_prims_alive`
  / `make_ret`, and it means a minimal signature guard must remain if we clone the
  trampoline `.ty()`.

**Decision (this branch): ship Layer 1 only.** Layers 1–3 are fully de-risked.
Layer 2 (descriptor-derived import typing) is achievable but is the sole
parity-heavy risk, and the Layer-1 path (minimal signature guard + clone the
trampoline `.ty()`) is already proven by the current branch. We take the schema-tree
+ `CastRecord` + bloat wins now under Layer 1, and defer Layer 2 to a follow-up.

### Phase 1 — Runtime schema representation

- `src/describe.rs`: delete `SCHEMA_MAX`, `SCHEMA_BUF`, `SCHEMA_LEN`,
  `leaf_schema`, `wrap_schema`, `cat_schema`, `schema_from_slice`. Add `Schema` +
  a single `const fn Schema::node(tag, words: &'static [u32], children: &'static
  [&'static Schema]) -> Schema` builder (plus thin `leaf` convenience), the
  `'static` empty sentinels (`EMPTY_WORD` / `EMPTY_CHILD_PTR`), and
  `flatten_len(&Schema) -> usize` + `flatten_into::<const N>(&Schema) -> [u32; N]`.
  Change the trait to `const SCHEMA: &'static Schema`. Rewrite all leaf/wrapper
  impls (`&T`, `&mut T`, `[T]`, `Option<T>`, `Result<T,E>`, `Clamped<T>`,
  `MaybeUninit<T>`, `NonNull`, raw pointers, `()`, `JsError`, `AssertUnwindSafe`,
  isize/usize/str/String variants).
  - **Per Phase 0 #1:** each impl writes the children array literal at the
    const-init site, e.g. `const SCHEMA: &'static Schema = &Schema::node(WRAP,
    &[OPTIONAL], &[T::SCHEMA]);`. Do **not** try to build the children run inside a
    `const fn` (rvalue promotion only happens at the call site — `E0515` otherwise).
- `src/convert/closures.rs`, `src/closure.rs`: rebuild the closure-trait-object,
  `OwnedClosure`, `BorrowedClosure`, and `ScopedClosure` schemas via
  `Schema::node` / `leaf`.
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

### Phase 3 — Cast trampoline refactor (Layer 1)

- `src/rt/mod.rs`:
  - Delete `FromBuf`, `ToBuf`. Replace the five scanned `i32.const` immediates with
    a single referenced `CastRecord`.
  - Replace `breaks_if_inlined` / `breaks_if_inlined_closure` with a trampoline
    `__wbg_cast_trampoline::<From, To, T, UW>(prims) -> WasmRet<To::Abi>` whose body
    is `__wbindgen_cast_marker(&RECORD as *const _ as *const ())`.
  - **Keep a minimal signature guard.** Phase 0 #4 confirmed that without genuine
    prim/ret data flow the optimizer mangles the trampoline's Wasm signature under
    `--release`. Since Layer 1 still recovers the import signature by cloning this
    trampoline's `.ty()`, retain the smallest guard that preserves the canonical
    `(From::Abi prims) -> WasmRet<To::Abi>` shape (the existing `keep_prims_alive` /
    `make_ret` pattern, trimmed). `black_box`/`_keepalive` archive-pull anchoring of
    the old schema buffers can go once the `CastRecord` is the live anchor.
  - Add `RECORD: &'static CastRecord` as a monomorphized associated const;
    `invoke` from `T::invoke_addr::<UW>()` for closures, null otherwise.
- `src/lib.rs`: replace the `__wbindgen_describe_cast` import with
  `__wbindgen_cast_marker(record: *const ())`.

### Phase 4 — CLI consumer (Layer 1)

- `crates/cli-support/src/descriptors.rs`:
  - `execute_casts`: scan for the single record-pointer operand of
    `__wbindgen_cast_marker` (tolerate debug `local.set`/`local.get` round-trips).
  - `DataSegmentView`: add typed, pointer-width-aware reads + `read_schema_tree(
    root_ptr)` that walks `Schema` nodes and emits the flat `u32` stream
    (byte-identical to today), feeding the unchanged `compose_cast_descriptor` +
    `Descriptor::decode`. Honor wasm64 pointer width and `tag`→pointer padding
    (Phase 0 #2).
  - Read `CastRecord` (3 fields) -> from/to roots + relocated `invoke` slot.
- `crates/cli-support/src/wit/mod.rs`: **keep cloning** the manufactured
  `__wbindgen_cast_N` import type from the trampoline's emitted `.ty()` (the current
  `self.module.funcs.get(orig_func_ids[0]).ty()` path); redirect calls as today via
  `handle_duplicate_imports`.

> **Deferred — Layer 2 (follow-up).** Derive the `__wbindgen_cast_N` import type
> from the `From`/`To` descriptors instead of cloning `.ty()`, then drop the
> trampoline signature guard entirely. Phase 0 #4 proved this is achievable but
> requires a CLI-side parity table for the per-type `WasmAbi` lowering (niche
> cases like `Option<i32>`→`f64`, multi-prim→sret/multivalue, memory64 widths). Not
> in scope for this branch.

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

Under Layer 1 it becomes a largely data-driven shim: the 5-immediate scanning,
`black_box` archive-pull anchoring of the old schema buffers, and the `FromBuf`/
`ToBuf` scratch are removed, replaced by a single `CastRecord` reference + static-data
reads + linker relocations. A trimmed signature guard remains (the trampoline `.ty()`
is still the source of the import signature until Layer 2 lands).

## Residual Risks

- **Const fn-ptr-as-pointer + table-index relocation** readability (Phase 0 #2) —
  validated on wasm32 + wasm64.
- **Linker GC** of the record/tree graph under optimization (Phase 0 #3) —
  validated; `&'static` anchoring suffices.
- **Const recursion depth / eval limits** for deeply nested generics — exercised to
  7 levels in Phase 0 #1 without issue.
- **Deferred: Layer 2 import-typing parity** with rustc lowering (Phase 0 #4) — out
  of scope for this branch; Layer 1 sidesteps it by cloning the trampoline `.ty()`.

## Decisions

- Refactor depth: **Layer 1 only** this branch (CastRecord indirection + schema-tree
  + bloat win, retaining `.ty()`-cloned import typing). Layer 2 (CLI-derived import
  typing) deferred to a follow-up per the Phase 0 #4 finding.
- Marker transport: **single pointer argument** to `__wbindgen_cast_marker`.
- Phase 0 gate: **passed (GO)** — see the Phase 0 section for spike results.
