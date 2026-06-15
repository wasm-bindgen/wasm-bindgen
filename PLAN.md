# Review Findings: `logan/descriptor-tree`

Bugs and issues found across the runtime (`src/`), `cli-support`, and
`macro-support`/`shared` crates (committed branch changes + uncommitted working
tree). Only defects are listed below; positives/verified-correct items are omitted.

## Critical

### 1. SYMBOL_REF `0xFF` sentinel collides with literal schema data
- **Where:** `crates/cli-support/src/descriptors_section.rs:204-226` (`resolve_symbols`) ↔ macro string/identifier emission (`crates/macro-support/src/codegen.rs`, `get_string` in `descriptors.rs`).
- **Problem:** `resolve_symbols` blindly scans the flat `u32` stream and treats any
  word equal to `SYMBOL_REF` (`0xFF`, `shared/src/tys.rs:71`) as the start of a
  symbol reference, consuming following words as `name_len` + UTF-8 name. But the
  macro emits literal data words that can legitimately equal `255`:
  - a type-name codepoint `U+00FF` (`'ÿ'`, a valid identifier char),
  - an `ENUM` hole value of `255`,
  - a `STRING_ENUM` `variant_count` of `255` (a 255-variant enum).
  When such a word appears, the resolver misreads a bogus `name_len` and either
  aborts with a confusing error or corrupts the stream. The `tys.rs` design note
  only justifies `0xFF` against *opcode* collisions (range `0..=36`), not against
  embedded literal data.
- **Fix direction:** make the resolver structure-aware (walk like
  `Descriptor::decode`, only honoring `SYMBOL_REF` in type/shim-index positions),
  or have the macro emit an explicit table of SYMBOL_REF slot offsets instead of a
  value-based sentinel.

## High

### 2. Fragile empty-`words` / element-0 slice design
- **Where:** `src/describe.rs:99-126` (`Schema::node`, `words()`, `children()`) ↔ `crates/cli-support/src/descriptors.rs:463,505-518` (`read_schema_tree_into`, `read_bytes`).
- **Problem (runtime):** `words()`/`children()` rebuild an N-element slice from a
  reference to *element 0 only* (`<[u32]>::first()`). Sound under const-eval, but UB
  under Stacked/Tree Borrows if called at runtime — and they are `pub const fn`.
- **Problem (CLI):** `Schema::node` stores a null base when the run is empty; the
  CLI's `read_schema_tree_into` → `read_bytes(0, 0)` bails because address `0` is
  not inside any data segment. Latent today (every emitted node has a non-empty
  `words` run) but the unused `SchemaTag::Cat` variant would be the first to trip
  it.
- **Fix direction:** base the slice on `as_ptr()` (whole-run provenance) instead of
  `&run[0]`; short-circuit `len == 0` in `read_bytes`/`read_u32_slice`; remove or
  document `SchemaTag::Cat` (currently dead/unreachable, `tys.rs:71`).

## Medium

### 3. `MarkerCallScanner::walk` does not recurse into nested instruction sequences
- **Where:** `crates/cli-support/src/descriptors.rs:594-691`.
- **Problem:** `walk` only iterates the top-level `seq`; `Instr::Block`/`Loop`/`IfElse`
  fall into the `_ =>` arm which clears state and never descends (unlike
  `scan_memory_init_seq` at 318-326, which recurses). A non-flat (debug/unoptimized)
  `__wbg_cast_trampoline` body wrapping the `__wbindgen_cast_marker` call inside a
  block/if would silently drop the cast → missing JS glue with no error.
- **Fix direction:** recurse into `Block`/`Loop`/`IfElse` like `scan_memory_init_seq`,
  or assert the trampoline body is flat.

### 4. Non-deterministic cast import naming
- **Where:** `crates/cli-support/src/wit/mod.rs:383-399`.
- **Problem:** `sorted_casts.sort_by` keys only on `sig_comment` (`"{arg:?} -> {ret:?}"`).
  Distinct `Descriptor` keys (e.g. closure casts differing only by `shim_idx`/`inner_ret`,
  not in `sig_comment`) collide on the sort key; stable sort then falls back to
  `HashMap` iteration order. Since `import_name` derives from the post-sort `idx`,
  `__wbindgen_cast_NNN` names can be assigned non-deterministically across runs,
  breaking reproducible builds/snapshots.
- **Fix direction:** break ties on a total order of the full encoded `Descriptor`.

### 5. Closure-wrapper emission queue only drained on the FUNCTION path
- **Where:** `crates/macro-support/src/codegen.rs:1152` (`take_pending_closure_wrappers`).
- **Problem:** `schema_expr_for_raw_closure` queues a closure invoke wrapper into the
  thread-local `CLOSURE_WRAPPER_EMISSIONS`, drained only in `emit_static_descriptor_entry`.
  Other callers of `schema_expr_for_type` (`emit_static_descriptor_entry_static` ~1257,
  struct getter ~116, DynamicUnion variants ~1065 / `variant_exprs` ~993) never drain
  it. If a closure-shaped type reached one of those contexts, the queued wrapper would
  leak into an unrelated entry's token stream or never be emitted (unresolvable
  SYMBOL_REF). Latent (those contexts aren't closure-typed today) but fragile
  cross-item thread-local state.
- **Fix direction:** factor a single `flush_pending_closure_wrappers(into)` called from
  every descriptor-entry emitter, or assert the queue is empty after building
  static-kind schema exprs.

### 6. `Clamped<Box<[T]>>` silently drops clamping for non-`u8` types
- **Where:** `src/lib.rs` `typed_arrays!` macro (~1900-1945).
- **Problem:** the macro generates `From<Clamped<Box<[$ty]>>>` for all primitive types,
  but only `u8` maps to `__wbindgen_uint8_clamped_array_new`; other types reuse their
  non-clamped constructor. So `JsValue::from(Clamped(box [1u16,2]))` produces a plain
  `Uint16Array`, silently ignoring the `Clamped` marker (the previous path emitted a
  `CLAMPED` descriptor).
- **Fix direction:** restrict the clamped `From` impl to `u8`, or `compile_error!`/doc
  that non-`u8` clamped is an intentional no-op marker.

### 7. `DESCRIPTOR_KIND_CAST` lifecycle is ambiguous
- **Where:** `crates/cli-support/src/descriptors.rs:204-210` (`ingest_section`), `crates/shared/src/lib.rs:93-95`.
- **Problem:** the constant is defined/accepted by `pack_entry` and the section parser,
  but no macro path emits it (casts come from the `rt/mod.rs` trampoline path). The
  `ingest_section` CAST arm silently discards entries while logging a stale "still
  handled by interpreter" message. If the macro never emits CAST, this is dead payload
  masking a potential producer bug; if it does, the discard is wrong.
- **Fix direction:** decide and assert — either `debug_assert` the macro never emits
  CAST into the section, or handle the entries.

## Low

### 8. wasm64 silent truncation / undertested 64-bit paths
- **`crates/cli-support/src/descriptors.rs:415-418`** (`read_ptr`): on wasm64 reads 8
  bytes but builds the result from `bytes[0..4]`, discarding the high word without
  checking it is zero. Same truncation at `descriptors.rs:601` (`Value::I64(n) as i32`)
  and `descriptors.rs:303` (`scan_memory_init`).
- **`crates/cli-support/src/descriptors.rs:815-821`** (`lookup_table_slot_by_name`):
  only matches `ConstExpr::Value(Value::I32(n))`, unlike `function_table_slot_of`/
  `get_function_table_entry` which handle `I64`/`Global`/`Extended`. On wasm64/PIC the
  fallback-A name lookup silently won't fire.
- **`src/rt/mod.rs:407-428`** (`ptr_to_word`/`len_to_word`): `usize as f64` loses
  precision ≥ 2^53 (benign for realistic memory sizes).
- **Fix direction:** `bail!` on nonzero high words instead of truncating; reuse
  `evaluate_const_expr` in `lookup_table_slot_by_name`; add a wasm64 `I64`-offset unit
  test.

### 9. `compose_cast_descriptor` invoke-patch is a positional heuristic
- **Where:** `crates/cli-support/src/descriptors.rs:533-549`.
- **Problem:** patches the "first `FUNCTION` opcode whose next word is `0`", assuming the
  closure's inner `FUNCTION` is the first one in the From schema. A future From schema
  nesting a `FUNCTION` ahead of the closure's would mis-patch; a closure whose `invoke`
  legitimately resolves to table slot `0` would skip patching.
- **Fix direction:** key off the `CLOSURE` opcode position to locate the inner
  `FUNCTION` deterministically.

### 10. `read_volatile` of uninitialized `WasmRet` in `make_ret`
- **Where:** `src/rt/mod.rs:212-218`.
- **Problem:** reads uninitialized memory (`MaybeUninit::<WasmRet<To::Abi>>::uninit()`)
  — UB in the abstract machine. Never executed at runtime (CLI replaces the trampoline),
  intentional signature guard, holds only because current `WasmAbi` prims are plain
  numerics with no validity invariants.
- **Fix direction:** add a comment noting it is an intentionally-never-run optimizer
  guard; confirm no `WasmRet<To::Abi>` instantiation carries a type with validity
  invariants (references/bool/enums).

### 11. `pack_entry` does not reject zero-length shim names
- **Where:** `src/describe.rs:230` (`pack_entry`).
- **Problem:** wire format documents `shim_name_len` as `1..=255`; the assert only
  checks `<= 255`, not `>= 1`. A zero-length name silently encodes `shim_name_len = 0`.
  Defensive only (names are always non-empty in practice).
- **Fix direction:** `assert!(!name.is_empty() && name.len() <= 255, ...)`.

### 12. 255-byte shim-name limit is now a hard const-eval compile error
- **Where:** `src/describe.rs:458` (`pack_entry` assert), `crates/shared/src/tys.rs`.
- **Problem:** the single-byte `shim_name_len` caps names at 255 bytes; over-long names
  (deeply namespaced JS imports) become a const-eval panic rather than a graceful
  diagnostic.
- **Fix direction:** document the limit next to `DESCRIPTORS_SECTION_NAME`/the SYMBOL_REF
  doc, and emit a clearer macro-level diagnostic.

### 13. `function_table_slot_of` returns first match without uniqueness guard
- **Where:** `crates/cli-support/src/wasm_conventions.rs:207-248`, `crates/cli-support/src/descriptors.rs:741-799` (`build_symbol_table`).
- **Problem:** if a function appears in multiple element segments (or a name collision
  occurs across exported table functions), the first hit is silently chosen.
- **Fix direction:** add a debug assertion on uniqueness.

### 14. `BigIntFromI128/U128` codegen emits unparenthesized expression
- **Where:** `crates/cli-support/src/js/mod.rs:6041-6044` — `format!("{} << BigInt(64) | {}", ...)`.
- **Problem:** semantics are correct in isolation and consistent with existing style, but
  if the intrinsic body is ever inlined into a larger expression context, the missing
  outer parens around `a << 64n | b` could combine incorrectly with surrounding operators.
- **Fix direction:** wrap in parens defensively.

### 15. `AsNumber` intrinsic not wired into manufactured aux imports
- **Where:** `crates/cli-support/src/intrinsic.rs:53`, `crates/cli-support/src/js/mod.rs:6027`.
- **Problem:** `AsNumber` has a definition and a JS arm but is not among the manufactured
  aux imports in `wit/mod.rs`. Fine only if it is resolved by name-matching a real
  `__wbindgen_as_number` import declared by the runtime; otherwise the arm is dead.
- **Fix direction:** confirm the runtime declares the import.

## Stale documentation referencing removed machinery

All describe deleted interpreter / `breaks_if_inlined` / `i32.const`-scanner mechanisms
and now mislead about how invoke addresses are filled or whether a fallback exists:

- `src/convert/closures.rs:114-123` — describes the removed `i32.const` immediate /
  scanner invoke transport (now `CastRecord::invoke`, `src/rt/mod.rs:128`).
- `crates/cli-support/src/descriptors.rs:207` — "still handled by interpreter".
- `crates/cli-support/src/descriptors.rs:684` — references `breaks_if_inlined` (now
  `__wbg_cast_trampoline`).
- `crates/cli-support/src/descriptors_section.rs:38-39` — `Entry::name` references the
  removed `breaks_if_inlined::<From, To>` monomorphization.
- `crates/cli-support/src/wasm_conventions.rs:194,206` — refers to a legacy-interpreter
  fallback that no longer exists.
- `crates/macro-support/src/codegen.rs` doc (~205-210) — claims a missing `SCHEMA` falls
  back to `&[]` + interpreter, but `WasmDescribe::SCHEMA` now has no default (missing
  impl is a hard compile error).

## Notes / confirmations needed

- **`SchemaTag` cross-crate layout coupling:** the CLI's `schema_field_offsets`
  (`crates/cli-support/src/descriptors.rs:425`) silently depends on `SchemaTag` staying
  `#[repr(u32)]` (`tys.rs:94`). Add a consumer-side note so a future repr change is
  caught at review.
- **`SCHEMA_VERSION` lag:** `crates/shared/src/lib.rs:7` is `"0.2.122"` while the crate
  builds as `0.2.125`. The hash-approval test only guards `lib.rs` content, so it passes —
  confirm the lag is intentional.
- **Interaction:** items #1, #2, and the `Cat` variant interact — the first `Cat` node
  ever emitted would simultaneously hit the empty-`words` null-pointer bug, so the
  slice/`len == 0` handling should precede any use of `Cat`.
