# `reinit = "reconstruct"`

When the Wasm instance is [reinitialized](../../handling-aborts.md#reinitialization),
every JS wrapper of an exported struct that was created by the previous
instance becomes stale: its pointer refers to memory that no longer exists, so
calling a method on it, or passing it back into Rust, throws
`Invalid stale object from previous Wasm instance`.

For long-lived objects held by the host whose state can be rebuilt from their
constructor arguments, throwing is worse than simply constructing the object
again. The `reinit` attribute opts a class into doing that transparently:

```rust
#[wasm_bindgen(reinit = "reconstruct")]
pub struct Counter {
    count: u32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new(start: u32) -> Counter {
        Counter { count: start }
    }

    pub fn increment(&mut self) -> u32 {
        self.count += 1;
        self.count
    }
}
```

```js
const counter = new Counter(5);
counter.increment(); // 6
counter.increment(); // 7
// ... a hard abort reinitializes the module ...
counter.increment(); // 6: the counter was rebuilt with `new Counter(5)`
```

## Semantics

* The class must have a `#[wasm_bindgen(constructor)]`.
* When the constructor succeeds, the generated JS stores the arguments it was
  called with on the wrapper, by reference. Nothing is cloned, so a
  reconstructed object sees the same JS values the first construction saw,
  including any mutations made to them since.
* When a method, getter, setter, or an argument check on a wrapper from a
  previous instance finds that the instance has changed, it calls the
  constructor again with the stored arguments, adopts the new pointer, and
  then carries on with the original operation. The JS object keeps its
  identity and prototype chain; only the Rust value behind it is new.
* If the constructor throws during reconstruction, that error propagates from
  the call that triggered it. The wrapper stays stale and the next call tries
  again.
* Objects returned from Rust rather than constructed from JS (for example a
  `Counter` returned from a `pub fn make() -> Counter`) have no constructor
  arguments to replay. They are stamped like any other wrapper and still throw
  the stale-object error after a reinit.

## With `extends`

Each instance of a class in an [`extends`](extends.md) chain holds one pointer
per ancestor. Reconstruction re-runs the most-derived constructor only, which
rebuilds the ancestors through the `parent` field, and then refreshes every
ancestor pointer on the wrapper, so parent methods dispatched through the
prototype chain see the fresh values too.

Because of this, `reinit` is only meaningful on the most-derived class. A
child that opts in while its parent does not is fine. A parent that opts in
while some child does not is rejected by `wasm-bindgen`: instances of the child
have no stored child-constructor call to replay.

## When it applies

The attribute only changes the generated glue when reinitialization is in
use, that is when `wasm_bindgen::handler::schedule_reinit()` is reachable from
the module or `--experimental-reset-state-function` is passed. Otherwise there
are no stale-object checks and nothing is stored on the wrapper.

Free functions, imported types, and closures are unaffected.

`"reconstruct"` is the only policy currently accepted; other values are a
compile error.
