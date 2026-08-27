# Releasing Memory with `memory.discard`

> **Note**: this feature is experimental and subject to change. It is only
> enabled when the `--experimental-memory-discard` CLI flag is passed, and
> the import contract WILL change as the underlying proposal evolves.

WebAssembly's linear memory can only grow. Even when an allocator frees pages,
the host engine keeps the physical memory committed, so a temporary allocation
spike stays resident for the lifetime of the instance.

The [memory-control proposal] adds a `memory.discard` instruction which
releases the physical pages backing a region of linear memory back to the
host, zeroing the region in the process (like `madvise(MADV_DONTNEED)` on
Linux).

LLVM cannot emit this instruction directly yet. Instead, when the
`--experimental-memory-discard` flag is passed, `wasm-bindgen` recognizes the
following function import as a request for it:

```wat
(import "env" "__wbindgen_memory_discard" (func (param i32 i32)))
```

or from C:

```c
__attribute__((import_module("env"), import_name("__wbindgen_memory_discard")))
extern void __wbindgen_memory_discard(void *addr, size_t len);
```

During the `wasm-bindgen` CLI post-processing step this import is replaced
with a local function whose body is the `memory.discard` instruction, so no
import survives to instantiation and page discard remains a pure Wasm
operation with no JS involved.

The arguments are a start address and a length in bytes; both follow the
memory's index type (`i64` under memory64). Per the proposal, the range must
be page-aligned (64KiB) or the instruction traps.

Note that a module containing `memory.discard` requires an engine with
memory-control support to validate. The import is typically pulled in
deliberately by a custom allocator whose purging path forwards to
`__wbindgen_memory_discard`; if it is present without the
`--experimental-memory-discard` flag, `wasm-bindgen` reports an error rather
than leaving a dangling import.

[memory-control proposal]: https://github.com/WebAssembly/memory-control
