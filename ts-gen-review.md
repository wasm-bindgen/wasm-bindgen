# IMPORTS RESOLUTION

### 20. `export *` wildcard re-exports not resolved
`parse/first_pass.rs:282-291` — A wildcard import `("*", "*")` is registered, but `resolve_imports` only handles named lookups. `export * from "..."` is registered in Phase 1 but skipped with a warning in Phase 2 — effectively a no-op. ?

# TRUE GENERICS
`parse/first_pass.rs:617-689` — `register_type` inserts placeholder `TypeDeclaration`s during Phase 1. Phase 2 inserts the *real* declarations separately. The placeholders remain in the type arena forever, which means `iter_types()` returns both placeholders and real declarations.

| Generic class/interface *definitions* | High |
| Deeply nested generics | Low |

# buffers

array buffer test cases

# negative tests

malformed input etc

## Summary: End-to-End Generic Flow

The complete pipeline for a TypeScript generic like `class Foo<T> { bar(x: T): Promise<T>; }`:

| Stage | Location | What happens to `T` |
|-------|----------|-------------------|
| Parse class type_params | `converters.rs:25` | `TypeParam { name: "T", constraint: None, default: None }` stored in `ClassDecl.type_params` |
| Parse method params | `members.rs:210` → `types.rs:368` | `convert_formal_params` (unscoped) → `TypeRef::Named("T")` |
| Parse method return | `members.rs:211-215` | `convert_ts_type_scoped` with method-only scope → still `TypeRef::Named("T")` (T is class-level, not in method scope) |
| Codegen type decl | `classes.rs:351` | `pub type Foo;` — no generics emitted |
| Codegen method param | `signatures.rs:581` → `typemap.rs:480` | `Named("T")` → `emit_type_name("T")` → error + `use JsValue as T;` → emits `T` (aliased to `JsValue`) |
| Codegen method return | `typemap.rs:398` | `Promise(Named("T"))` → inner `Named("T")` → `JsValue` → elided → bare `Promise` |

**If the generic param were method-level** (e.g., `json<T>(): Promise<T>`), the return type path changes:
- `members.rs:113-117` builds scope with `T`
- `members.rs:123` calls `convert_ts_type_scoped` → at `types.rs:180-182`, `T` is in scope → **erased to `TypeRef::Any`**
- But `members.rs:119` calls `convert_formal_params` (unscoped) for params → `T` in params still becomes `Named("T")`
</task_result>
```

---

## Assistant (Plan · claude-opus-4-6 · 79.5s)

Excellent — this is exactly what I needed. Now let me write the complete implementation plan.

---

# TRUE GENERICS Implementation Plan for ts-gen

## Current State Summary

- **IR has the structure**: `TypeParam` already has `name`, `constraint`, `default`. `TypeRef::GenericInstantiation(name, args)` already exists for instantiated generics.
- **Parse preserves type args on unknown types**: `GenericInstantiation` is produced for types with args.
- **Parse erases type params to `Any`**: When `T` is in scope, it becomes `TypeRef::Any` — all generic information lost.
- **Codegen drops `GenericInstantiation` args**: Explicitly with a `TODO(Phase 3)` comment.
- **Class-level type params never reach members**: Only method-level params create a scope.
- **Codegen never reads `type_params`**: They're stored in the IR but completely ignored during code generation.

## Architecture

The approach: **preserve generic type parameters through the entire pipeline**, only erasing when the final codegen output requires it (e.g., for wasm-bindgen extern types that don't support generics at the type declaration level).

### Step 1: IR — Add `TypeRef::TypeParam`

**File:** `ir.rs`

Add a new variant to `TypeRef`:
```rust
/// Reference to a generic type parameter (e.g., `T`, `K`, `V`).
/// Distinguished from `Named` so codegen can emit it as a bare identifier
/// (not resolved through scope chains or external maps).
TypeParam(String),
```

This variant represents "T as declared by a type parameter" — distinct from `Named("T")` which means "a type called T that should be looked up."

**Impact:** Every `match` on `TypeRef` needs a new arm. Add `TypeParam(n) => ...` to:
- `codegen/typemap.rs:to_syn_type` — emit as bare ident
- `codegen/typemap.rs:to_return_type` — handle same as in arg position
- `codegen/signatures.rs:flatten_type` — treat as leaf (not expandable)
- `codegen/signatures.rs:type_snake_name` — `to_snake_case(n)`
- `codegen/signatures.rs:is_void_return` — false
- Any other exhaustive matches (compiler will flag these)

**serde:** If the `serde` feature is enabled, `TypeParam` needs to serialize.

### Step 2: Parse — Preserve Type Params Instead of Erasing

**File:** `parse/types.rs`

Change `convert_type_reference_scoped` (line 180-182):
```rust
// Before:
if scope.contains(name.as_str()) {
    return TypeRef::Any;
}

// After:
if scope.contains(name.as_str()) {
    return TypeRef::TypeParam(name);
}
```

This single change preserves type params through the entire IR. `Promise<T>` becomes `Promise(TypeParam("T"))` instead of `Promise(Any)`.

**Also change `convert_function_type_scoped`** (line 344): The function params call `convert_formal_params` (unscoped). Change to use scoped conversion so params and return types are consistent. This was Item 9 from the review — but now instead of erasing to `Any`, we preserve as `TypeParam`.

### Step 3: Parse — Thread Class-Level Type Params to Members

**Files:** `parse/members.rs`, `parse/first_pass/converters.rs`

The scope for member conversion needs to include class/interface-level type params. Two approaches:

**Approach A (minimal):** Add an optional `parent_scope: &TypeParamScope` parameter to `convert_ts_signature`, `convert_class_element`. Callers in `convert_class_decl` and `convert_interface_decl` build the scope from the class/interface type params and pass it.

**Approach B (context struct):** Create a `MemberConvertCtx` with `parent_type_params: &TypeParamScope` and `docs`, `diag`. Similar to what we did with `PopulateCtx`.

Recommend **Approach A** for now — it's less invasive and the member conversion functions have manageable parameter counts.

**Changes in `converters.rs`:**
- `convert_class_decl`: Build scope from `class.type_parameters`, pass to `convert_class_element`
- `convert_interface_decl`: Build scope from `iface.type_parameters`, pass to `convert_ts_signature`

**Changes in `members.rs`:**
- `convert_ts_signature(sig, parent_scope, docs, diag)` — pass parent scope
- `convert_method_signature`: Merge parent scope + method scope before converting params and return type
- `convert_class_method`: Same merge
- `convert_property_signature`: Use parent scope for property type
- `convert_class_property`: Use parent scope for property type
- `convert_construct_signature`: Use parent scope (constructors share class generics)

### Step 4: Codegen — Emit `TypeParam` as Bare Identifier

**File:** `codegen/typemap.rs`

In `to_syn_type`, add:
```rust
TypeRef::TypeParam(name) => {
    let ident = make_ident(name);
    quote! { #ident }
}
```

No borrowing (`&T` in arg position doesn't make sense for a generic param — it should be `&T` or `T` depending on wasm-bindgen's expectations). For now, treat TypeParam the same as a named JS type — `maybe_ref(quote! { #ident }, borrow)`.

**Also handle `TypeParam` in inner position:** In the inner-position early returns (lines 340-353), `TypeParam` should just return the bare ident (same as outer).

### Step 5: Codegen — Emit Generic Args from `GenericInstantiation`

**File:** `codegen/typemap.rs`

Replace the current TODO stub (lines 490-499):
```rust
TypeRef::GenericInstantiation(name, args) => {
    let base = named_type_to_rust(name, ctx);
    if args.is_empty() {
        maybe_ref(base, borrow)
    } else {
        let inner_pos = pos.to_inner();
        let arg_tokens: Vec<_> = args.iter()
            .map(|a| to_syn_type(a, inner_pos, ctx, scope))
            .collect();
        // Elide if all args are JsValue (the default)
        if arg_tokens.iter().all(|t| is_jsvalue_arg(t)) {
            maybe_ref(base, borrow)
        } else {
            maybe_ref(quote! { #base<#(#arg_tokens),*> }, borrow)
        }
    }
}
```

This handles `ReadableStreamDefaultController<R>` → `ReadableStreamDefaultController<R>` (when `R` is a `TypeParam`), or `ReadableStreamDefaultController<string>` → `ReadableStreamDefaultController<JsString>`.

### Step 6: Codegen — Add Type Params to Class/Interface Extern Blocks

**File:** `codegen/classes.rs`

**Add `type_params` to `ClassConfig`:**
```rust
struct ClassConfig {
    // ... existing fields ...
    type_params: Vec<ir::TypeParam>,
}
```

Populate from `ClassDecl::type_params` / `InterfaceDecl::type_params` in the `from_*` constructors.

**Emit generic params on methods (NOT on the type declaration):**

wasm-bindgen extern type declarations (`pub type Foo;`) don't support Rust generics. But methods CAN have generic params:

```rust
#[wasm_bindgen]
extern "C" {
    pub type ReadableStreamDefaultController;  // no generics here

    #[wasm_bindgen(method)]
    pub fn enqueue<R>(this: &ReadableStreamDefaultController, chunk: &R);  // generic on method
}
```

Wait — this is wrong. wasm-bindgen **does** support generics on the type:
```rust
pub type Array<T = JsValue>;
```

So for class-level generics:
```rust
pub type ReadableStreamDefaultController<R = JsValue>;
```

And methods use the class-level `R`:
```rust
pub fn enqueue<R>(this: &ReadableStreamDefaultController<R>, chunk: &R);
```

**Emit in `generate_type_decl`:**
```rust
// If type_params is non-empty, emit generic params with JsValue defaults
let generics = if config.type_params.is_empty() {
    quote! {}
} else {
    let params: Vec<_> = config.type_params.iter().map(|tp| {
        let name = make_ident(&tp.name);
        quote! { #name = JsValue }
    }).collect();
    quote! { <#(#params),*> }
};
// Then: pub type #rust_ident #generics;
```

**Emit on methods:** Methods that reference class-level type params already get them through `TypeRef::TypeParam("R")`. The `to_syn_type` handler emits `R` as a bare ident. But Rust requires the method to declare `<R>` in its signature if `R` isn't from the enclosing scope.

This is the key question: **Are wasm-bindgen extern methods inside a generic extern type parameterized by the type's generics?**

Looking at js-sys:
```rust
pub type Array<T = JsValue>;

#[wasm_bindgen(method)]
pub fn push<T>(this: &Array<T>, value: &T) -> u32;
```

Yes — the method re-declares `<T>` and uses `Array<T>` as the self type. So each method that uses class-level generics needs to:
1. Declare the class-level type params as method-level generics
2. Use `ClassName<T>` as the self type instead of `ClassName`

**Changes to method generation:**
In `generate_expanded_method`, `generate_expanded_static_method`, `generate_getter`, `generate_setter`:
- If the class has type params, add them to the method's generic params
- Use `ClassName<T, U>` as the `this` type in method signatures
- Only add params that are actually referenced in the method's types (optimization, not required)

### Step 7: Codegen — Emit Trait Bounds for Constraints

For `T extends Foo`, emit `T: Upcast<Foo>` or just `T` (depending on whether wasm-bindgen uses trait bounds for this).

Looking at js-sys:
```rust
pub fn concat<T, U: Upcast<T>>(this: &Array<T>, array: &Array<U>) -> Array<T>;
```

So yes, `Upcast` bounds are used. For ts-gen:
- `T extends Foo` → `T: Upcast<Foo>`  
- Unconstrained `T` → just `T` (which defaults to `JsValue`)

This can be deferred to a follow-up — unconstrained generics work first.

### Step 8: Codegen — Handle Signature Expansion with Generics

**File:** `codegen/signatures.rs`

When expanding optional params and union types, generic type params (`TypeParam("T")`) should be treated as **opaque leaves** — not expanded. This is already the natural behavior since `TypeParam` doesn't match `Union`, `Nullable`, or `Optional`.

`flatten_type` needs a new arm:
```rust
TypeRef::TypeParam(_) => vec![ty.clone()],  // leaf — no expansion
```

`type_snake_name` needs:
```rust
TypeRef::TypeParam(n) => to_snake_case(n),  // "T" → "t" for naming
```

### Step 9: Tests

**New fixture: `generics.d.ts`**

```typescript
// Class-level generics
interface ReadableStreamDefaultController<R> {
    enqueue(chunk?: R): void;
    readonly desiredSize: number | null;
}

// Method-level generics
interface Body {
    json<T>(): Promise<T>;
    text(): Promise<string>;
}

// Multiple type params
interface Map<K, V> {
    get(key: K): V | undefined;
    set(key: K, value: V): this;
    readonly size: number;
}

// Generic with constraint (Phase 2)
// interface Comparable<T extends Comparable<T>> {
//     compareTo(other: T): number;
// }

// Generic with default
interface Container<T = any> {
    value: T;
    map<U>(fn: (t: T) => U): Container<U>;
}

// Generic function
declare function structuredClone<T>(value: T): T;
declare function identity<T>(x: T): T;
```

Expected output should show `<R>`, `<T>`, `<K, V>` on methods, `<R = JsValue>` on type declarations, and proper threading of type params through `Promise<T>`, `Array<R>`, etc.

---

## Execution Order

| Phase | Description | Files | Est. Complexity |
|-------|------------|-------|----------------|
| **1** | Add `TypeRef::TypeParam` to IR | `ir.rs` | Low |
| **2** | Parse: emit `TypeParam` instead of `Any` for in-scope params | `parse/types.rs` | Low |
| **3** | Parse: thread class-level scope to member conversion | `parse/members.rs`, `parse/first_pass/converters.rs` | Medium |
| **4** | Codegen: handle `TypeParam` in `to_syn_type` | `codegen/typemap.rs` | Low |
| **5** | Codegen: emit generic args from `GenericInstantiation` | `codegen/typemap.rs` | Low |
| **6** | Codegen: add type params to `ClassConfig`, emit on type decls and methods | `codegen/classes.rs` | High |
| **7** | Codegen: handle `TypeParam` in signatures | `codegen/signatures.rs` | Low |
| **8** | Tests: generics fixture + bless | `tests/fixtures/generics.d.ts` | Low |
| **9** | (Follow-up) Trait bounds for constraints | `codegen/classes.rs` | Medium |
| **10** | (Follow-up) Default type params | `codegen/classes.rs` | Low |

Phases 1-5 can land as a single PR — they enable generic params to flow through without breaking existing output (params that were `Any` become `TypeParam` → emitted as bare ident instead of `JsValue`).

Phase 6 is the big one — it requires understanding exactly how wasm-bindgen expects generic extern types and methods to be declared, and making the class codegen emit the right syntax.

Phases 9-10 are follow-ups that add polish.

---

## Risk Assessment

**Breaking change risk:** Medium. Existing snapshots will change:
- `use JsValue as T;` aliases disappear
- Methods that had erased generics now show type params
- Some signatures that were `foo(arg: &JsValue)` become `foo<T>(arg: &T)`

**Compilation risk:** High for Phase 6. The generated code must exactly match what wasm-bindgen's macro expects for generic extern types. Need to verify with the js-sys patterns as reference.

**Correctness risk for Phase 2:** Low. `TypeParam("T")` is strictly more information than `Any`. Code that was erasing to `JsValue` now preserves the param, but the default is still `JsValue` so runtime behavior is identical.

---

