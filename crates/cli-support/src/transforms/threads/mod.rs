use crate::wasm_conventions;
use anyhow::{anyhow, bail, Error};
use std::cmp;
use walrus::ir::{BinaryOp, UnaryOp, Value};
use walrus::FunctionBuilder;
use walrus::{
    ir::MemArg, ConstExpr, ExportItem, FunctionId, GlobalId, InstrSeqBuilder, Memory, MemoryId,
    Module, ValType,
};

pub const PAGE_SIZE: u32 = 1 << 16;
const DEFAULT_THREAD_STACK_SIZE: u32 = 1 << 21; // 2MB
const ATOMIC_MEM_ARG: MemArg = MemArg {
    align: 4,
    offset: 0,
};

#[derive(Clone, Copy)]
pub struct ThreadCount(walrus::LocalId);

/// Is threaded Wasm enabled?
pub fn is_enabled(module: &Module) -> bool {
    // Compatibility with older LLVM outputs. Newer LLVM outputs, when
    // atomics are enabled, emit a shared memory. That's a good indicator
    // that we have work to do. If shared memory isn't enabled, though then
    // this isn't an atomic module so there's nothing to do. We still allow,
    // though, an environment variable to force us to go down this path to
    // remain compatible with older LLVM outputs.
    match wasm_conventions::get_memory(module) {
        Ok(memory) => module.memories.get(memory).shared,
        Err(_) => false,
    }
}

/// Execute the transformation on the parsed Wasm module specified.
///
/// This function will prepare `Module` to be run on multiple threads,
/// performing steps such as:
///
/// * All data segments are switched to "passive" data segments to ensure
///   they're only initialized once (coming later)
/// * If memory is exported from this module, it is instead switched to
///   being imported (with the same parameters).
/// * The imported memory is required to be `shared`, ensuring it's backed
///   by a `SharedArrayBuffer` on the web.
/// * A `global` for a thread ID is injected.
/// * Four bytes in linear memory are reserved for the counter of thread
///   IDs.
/// * A `start` function is injected (or prepended if one already exists)
///   which initializes memory for the first thread and otherwise allocates
///   thread ids for all threads.
/// * Some stack space is prepared for each thread after the first one.
///
/// More and/or less may happen here over time, stay tuned!
pub fn run(module: &mut Module) -> Result<Option<ThreadCount>, Error> {
    if !is_enabled(module) {
        return Ok(None);
    }

    let memory = wasm_conventions::get_memory(module)?;

    let mem = module.memories.get_mut(memory);
    assert!(mem.shared);

    let memory64 = mem.memory64;
    if mem.import.is_none() {
        bail!("threading requires memory to be imported; add `-Clink-arg=--import-memory` to RUSTFLAGS");
    }
    if !mem.data_segments.is_empty() {
        bail!("threading requires memory to not have active data segments");
    }

    // Now we need to allocate extra static memory for:
    // - A thread id counter.
    // - A temporary stack for calls to `malloc()` and `free()`.
    // - A lock to synchronize usage of the above stack.
    // For this, we allocate 1 extra page of memory (should be enough as temporary
    // stack) and grab the first 2 _aligned_ i32 words to use as counter and lock.
    let static_data_align = 4;
    let static_data_pages = 1;
    let (base, addr) = allocate_static_data(mem, static_data_pages, static_data_align)?;

    let abi = Abi::of(module, memory64)?;

    let tls = Tls {
        init: delete_synthetic_func(module, "__wasm_init_tls")?,
        size: delete_synthetic_global(module, "__tls_size", abi.ptr)?,
        align: delete_synthetic_global(module, "__tls_align", abi.ptr)?,
        base: wasm_conventions::get_tls_base(module)
            .ok_or_else(|| anyhow!("failed to find tls base"))?,
    };

    let thread_counter_addr = addr;

    let stack_alloc = module
        .globals
        .add_local(abi.ptr, true, false, abi.ptr_const_expr(0));

    // Make sure the temporary stack is aligned down
    let temp_stack =
        (base + u64::from(static_data_pages * PAGE_SIZE)) & !u64::from(static_data_align - 1);

    const _: () = assert!(DEFAULT_THREAD_STACK_SIZE % PAGE_SIZE == 0);

    let stack = Stack {
        pointer: wasm_conventions::get_stack_pointer(module)
            .ok_or_else(|| anyhow!("failed to find stack pointer"))?,
        temp: temp_stack,
        temp_lock: thread_counter_addr + 4,
        alloc: stack_alloc,
        size: module.globals.add_local(
            abi.ptr,
            true,
            false,
            abi.ptr_const_expr(DEFAULT_THREAD_STACK_SIZE.into()),
        ),
    };

    let _ = module.exports.add("__stack_alloc", stack.alloc);

    let thread_count = inject_start(module, abi, &tls, &stack, thread_counter_addr, memory)?;

    // we expose a `__wbindgen_thread_destroy()` helper function that deallocates stack space.
    //
    // ## Safety
    // After calling this function in a given agent, the instance should be considered
    // "destroyed" and any further invocations into it will trigger UB. This function
    // should not be called from an agent that cannot block (e.g. the main document thread).
    //
    // You can also call it from a "leader" agent, passing appropriate values, if said leader
    // is in charge of cleaning up after a "follower" agent. In that case:
    // - The "appropriate values" are the values of the `__tls_base` and `__stack_alloc` globals
    //   and the stack size from the follower thread, after initialization.
    // - The leader does _not_ need to block.
    // - Similar restrictions apply: the follower thread should be considered unusable afterwards,
    //   the leader should not call this function with the same set of parameters twice.
    // - Moreover, concurrent calls can lead to UB: the follower could be in the middle of a
    //   call while the leader is destroying its stack! You should make sure that this cannot happen.
    inject_destroy(module, abi, &tls, &stack, memory)?;

    Ok(Some(thread_count))
}

impl ThreadCount {
    pub fn wrap_start(self, builder: &mut FunctionBuilder, start: FunctionId) {
        // We only want to call the start function if we are in the first thread.
        // The thread counter should be 0 for the first thread.
        builder.func_body().local_get(self.0).if_else(
            None,
            |_| {},
            |body| {
                body.call(start);
            },
        );
    }
}

fn delete_synthetic_func(module: &mut Module, name: &str) -> Result<FunctionId, Error> {
    match delete_synthetic_export(module, name)? {
        walrus::ExportItem::Function(f) => Ok(f),
        _ => bail!("`{name}` must be a function"),
    }
}

/// The value of a constant global of the linker. Its type is the address type
/// of the memory: `i32` on wasm32, `i64` on memory64.
fn delete_synthetic_global(module: &mut Module, name: &str, ty: ValType) -> Result<u64, Error> {
    let id = match delete_synthetic_export(module, name)? {
        walrus::ExportItem::Global(g) => g,
        _ => bail!("`{name}` must be a global"),
    };
    let g = match &module.globals.get(id).kind {
        walrus::GlobalKind::Local(g) => g,
        walrus::GlobalKind::Import(_) => bail!("`{name}` must not be an imported global"),
    };
    match (g, ty) {
        (ConstExpr::Value(Value::I32(v)), ValType::I32) => Ok(u64::from(*v as u32)),
        (ConstExpr::Value(Value::I64(v)), ValType::I64) => Ok(*v as u64),
        _ => bail!("`{name}` was not an `{ty}` constant"),
    }
}

fn delete_synthetic_export(module: &mut Module, name: &str) -> Result<ExportItem, Error> {
    let item = module
        .exports
        .iter()
        .find(|e| e.name == name)
        .ok_or_else(|| anyhow!("failed to find `{name}`"))?;
    let ret = item.item;
    let id = item.id();
    module.exports.delete(id);
    Ok(ret)
}

/// Allocates extra space for static data. Returns `(addr, base)`, where:
/// - `base` is the starting address of the extra `pages`.
/// - `addr` is the _first_ address in that chunk that is aligned to `align`.
fn allocate_static_data(memory: &mut Memory, pages: u32, align: u32) -> Result<(u64, u64), Error> {
    // Exclusive end of the address space.
    let (limit, space) = if memory.memory64 {
        (u64::MAX, "memory64")
    } else {
        (1 << 32, "wasm32")
    };
    // Allocate after the module's original initial memory.
    let base = memory
        .initial
        .checked_mul(u64::from(PAGE_SIZE))
        .filter(|base| *base < limit)
        .ok_or_else(|| anyhow!("initial memory size exceeds {space} address space"))?;
    let static_data_size = pages
        .checked_mul(PAGE_SIZE)
        .ok_or_else(|| anyhow!("static data allocation size overflows {space} address space"))?;
    base.checked_add(u64::from(static_data_size))
        .filter(|end| *end <= limit)
        .ok_or_else(|| anyhow!("static data allocation exceeds {space} address space"))?;
    let align = u64::from(align);
    let address = (base + (align - 1)) & !(align - 1); // align up

    memory.initial += u64::from(pages);
    memory.maximum = memory.maximum.map(|m| cmp::max(m, memory.initial));

    Ok((base, address))
}

/// The value types of the code that this transform emits.
#[derive(Clone, Copy)]
struct Abi {
    /// The type of an address: `i32` on wasm32, `i64` on memory64.
    ptr: ValType,
    /// The type of a size or an address in the parameters of `__wbindgen_malloc`
    /// and `__wbindgen_free`: `i32` on wasm32, and `f64` on memory64 (the
    /// number ABI of `WasmWord`, which gives a JS number).
    word: ValType,
}

impl Abi {
    /// Read the types from the memory and from `__wbindgen_malloc` and
    /// `__wbindgen_free`.
    fn of(module: &Module, memory64: bool) -> Result<Abi, Error> {
        let ptr = if memory64 { ValType::I64 } else { ValType::I32 };
        let malloc = module.types.get(
            module
                .funcs
                .get(find_function(module, "__wbindgen_malloc")?)
                .ty(),
        );
        let free = module.types.get(
            module
                .funcs
                .get(find_function(module, "__wbindgen_free")?)
                .ty(),
        );
        let word = match malloc.params() {
            [word, align] if word == align => *word,
            _ => bail!("`__wbindgen_malloc` must take a size and an alignment of the same type"),
        };
        let supported = match ptr {
            ValType::I32 => word == ValType::I32,
            _ => word == ValType::F64,
        };
        if !supported {
            bail!("`__wbindgen_malloc` takes `{word}`, which does not suit `{ptr}` addresses");
        }
        if malloc.results() != [word] {
            bail!("`__wbindgen_malloc` must return a `{word}`");
        }
        if free.params() != [word, word, word] || !free.results().is_empty() {
            bail!("`__wbindgen_free` must take three `{word}` values");
        }
        Ok(Abi { ptr, word })
    }

    /// The `add` instruction of addresses.
    fn add(self) -> BinaryOp {
        match self.ptr {
            ValType::I32 => BinaryOp::I32Add,
            _ => BinaryOp::I64Add,
        }
    }

    fn ptr_value(self, value: u64) -> Value {
        match self.ptr {
            ValType::I32 => Value::I32(value as i32),
            _ => Value::I64(value as i64),
        }
    }

    fn ptr_const_expr(self, value: u64) -> ConstExpr {
        ConstExpr::Value(self.ptr_value(value))
    }

    /// Push the address `value`.
    fn ptr_const(self, body: &mut InstrSeqBuilder<'_>, value: u64) {
        body.const_(self.ptr_value(value));
    }

    /// Push the word `value`, an argument of `__wbindgen_malloc` or `__wbindgen_free`.
    fn word_const(self, body: &mut InstrSeqBuilder<'_>, value: u64) {
        body.const_(match self.word {
            ValType::I32 => Value::I32(value as i32),
            _ => Value::F64(value as f64),
        });
    }

    /// Change the address on the stack into a word.
    fn ptr_to_word(self, body: &mut InstrSeqBuilder<'_>) {
        if self.word == ValType::F64 {
            body.unop(UnaryOp::F64ConvertUI64);
        }
    }

    /// Change the word on the stack into an address. A word that is not a
    /// number (`undefined` from JS on the number ABI) becomes 0.
    fn word_to_ptr(self, body: &mut InstrSeqBuilder<'_>) {
        if self.word == ValType::F64 {
            body.unop(UnaryOp::I64TruncUSatF64);
        }
    }

    /// Change the `i32` on the stack, a size from JS, into an address.
    fn i32_to_ptr(self, body: &mut InstrSeqBuilder<'_>) {
        if self.ptr == ValType::I64 {
            body.unop(UnaryOp::I64ExtendUI32);
        }
    }

    /// Change the address on the stack into the `i32` condition "not zero".
    fn ptr_is_nonzero(self, body: &mut InstrSeqBuilder<'_>) {
        if self.ptr == ValType::I64 {
            body.i64_const(0).binop(BinaryOp::I64Ne);
        }
    }

    /// Change the word on the stack into the `i32` condition "not zero".
    fn word_is_nonzero(self, body: &mut InstrSeqBuilder<'_>) {
        self.word_to_ptr(body);
        self.ptr_is_nonzero(body);
    }
}

struct Tls {
    init: walrus::FunctionId,
    size: u64,
    align: u64,
    base: GlobalId,
}

struct Stack {
    /// The stack pointer global
    pointer: GlobalId,
    /// The address of a small, "scratch-space" stack
    temp: u64,
    /// The address of a lock for the temporary stack
    temp_lock: u64,
    /// A global to store allocated stack
    alloc: GlobalId,
    /// The size of the stack
    size: GlobalId,
}

fn inject_start(
    module: &mut Module,
    abi: Abi,
    tls: &Tls,
    stack: &Stack,
    thread_counter_addr: u64,
    memory: MemoryId,
) -> Result<ThreadCount, Error> {
    use walrus::ir::*;

    let local = module.locals.add(abi.ptr);
    let thread_count = module.locals.add(ValType::I32);
    // The stack size from JS: an `i32` also on memory64, thus a JS number.
    let stack_size = module.locals.add(ValType::I32);

    let malloc = find_function(module, "__wbindgen_malloc")?;

    let prev_start = wasm_conventions::get_start(module);
    let mut builder = FunctionBuilder::new(&mut module.types, &[ValType::I32], &[]);

    if let Ok(prev_start) | Err(Some(prev_start)) = prev_start {
        builder.func_body().call(prev_start);
    }

    let mut body = builder.func_body();

    // Perform an if/else based on whether we're the first thread or not. Our
    // thread ID will be zero if we're the first thread, otherwise it'll be
    // nonzero (assuming we don't overflow...)
    abi.ptr_const(&mut body, thread_counter_addr);
    body.i32_const(1)
        .atomic_rmw(memory, AtomicOp::Add, AtomicWidth::I32, ATOMIC_MEM_ARG)
        .local_tee(thread_count)
        .if_else(
            None,
            // If our thread id is nonzero then we're the second or greater thread, so
            // we give ourselves a stack and we update our stack
            // pointer as the default stack pointer is surely wrong for us.
            |body| {
                body.local_get(stack_size).if_else(
                    None,
                    |body| {
                        body.local_get(stack_size);
                        abi.i32_to_ptr(body);
                        body.global_set(stack.size);
                    },
                    |_| (),
                );

                // local = malloc(stack.size, align) [aka base]
                with_temp_stack(body, abi, memory, stack, |body| {
                    body.global_get(stack.size);
                    abi.ptr_to_word(body);
                    abi.word_const(body, 16);
                    body.call(malloc);
                    abi.word_to_ptr(body);
                    body.local_tee(local);
                });

                // stack.alloc = base
                body.global_set(stack.alloc);

                // stack_pointer = base + stack.size
                body.global_get(stack.alloc)
                    .global_get(stack.size)
                    .binop(abi.add())
                    .global_set(stack.pointer);
            },
            // If the thread id is zero then the default stack pointer works for
            // us.
            |_| {},
        );

    // Afterwards we need to initialize our thread-local state.
    abi.word_const(&mut body, tls.size);
    abi.word_const(&mut body, tls.align);
    body.call(malloc);
    abi.word_to_ptr(&mut body);
    body.global_set(tls.base)
        .global_get(tls.base)
        .call(tls.init);

    let id = builder.finish(vec![stack_size], &mut module.funcs);
    module.start = Some(id);

    Ok(ThreadCount(thread_count))
}

fn inject_destroy(
    module: &mut Module,
    abi: Abi,
    tls: &Tls,
    stack: &Stack,
    memory: MemoryId,
) -> Result<(), Error> {
    let free = find_function(module, "__wbindgen_free")?;

    // The parameters have the type of the words of `__wbindgen_free`: on
    // memory64 with the number ABI, JS gives numbers, and a missing argument
    // (`undefined`, thus NaN) is 0 as on wasm32.
    let mut builder = FunctionBuilder::new(&mut module.types, &[abi.word; 3], &[]);

    builder.name("__wbindgen_thread_destroy".into());

    let mut body = builder.func_body();

    // if no explicit parameters are passed (i.e. their value is 0) then we assume
    // we're being called from the agent that must be destroyed and rely on its globals
    let tls_base = module.locals.add(abi.word);
    let stack_alloc = module.locals.add(abi.word);
    let stack_size = module.locals.add(abi.word);

    // Ideally, at this point, we would destroy the values stored in TLS.
    // We can't really do that without help from the standard library.
    // See https://github.com/wasm-bindgen/wasm-bindgen/pull/2769#issuecomment-1015775467.

    body.local_get(tls_base);
    abi.word_is_nonzero(&mut body);
    body.if_else(
        None,
        |body| {
            body.local_get(tls_base);
            abi.word_const(body, tls.size);
            abi.word_const(body, tls.align);
            body.call(free);
        },
        |body| {
            body.global_get(tls.base);
            abi.ptr_to_word(body);
            abi.word_const(body, tls.size);
            abi.word_const(body, tls.align);
            body.call(free);

            // set tls.base = i32::MIN (i64::MIN on memory64) to trigger invalid memory
            let invalid = match abi.ptr {
                ValType::I32 => u64::from(i32::MIN as u32),
                _ => i64::MIN as u64,
            };
            abi.ptr_const(body, invalid);
            body.global_set(tls.base);
        },
    );

    // free the stack calling `__wbindgen_free(stack.alloc, stack.size)`
    body.local_get(stack_alloc);
    abi.word_is_nonzero(&mut body);
    body.if_else(
        None,
        |body| {
            // we're destroying somebody else's stack, so we can use our own
            body.local_get(stack_alloc).local_get(stack_size);
            abi.word_const(body, DEFAULT_THREAD_STACK_SIZE.into());
            body.local_get(stack_size);
            abi.word_is_nonzero(body);
            body.select(None);
            abi.word_const(body, 16);
            body.call(free);
        },
        |body| {
            with_temp_stack(body, abi, memory, stack, |body| {
                body.global_get(stack.alloc);
                abi.ptr_to_word(body);
                body.global_get(stack.size);
                abi.ptr_to_word(body);
                abi.word_const(body, 16);
                body.call(free);
            });

            // set stack.alloc = 0 to trigger invalid memory
            abi.ptr_const(body, 0);
            body.global_set(stack.alloc);
        },
    );

    let destroy_id = builder.finish(vec![tls_base, stack_alloc, stack_size], &mut module.funcs);

    module.exports.add("__wbindgen_thread_destroy", destroy_id);

    Ok(())
}

fn find_function(module: &Module, name: &str) -> Result<FunctionId, Error> {
    let e = module
        .exports
        .iter()
        .find(|e| e.name == name)
        .ok_or_else(|| anyhow!("failed to find `{name}`"))?;
    match e.item {
        walrus::ExportItem::Function(f) => Ok(f),
        _ => bail!("`{name}` wasn't a function"),
    }
}

/// Wraps the instructions fed by `block()` so that they can assume that the temporary, scratch
/// stack is usable. Clobbers `stack.pointer`.
fn with_temp_stack(
    body: &mut InstrSeqBuilder<'_>,
    abi: Abi,
    memory: MemoryId,
    stack: &Stack,
    block: impl Fn(&mut InstrSeqBuilder<'_>),
) {
    use walrus::ir::*;

    abi.ptr_const(body, stack.temp);
    body.global_set(stack.pointer);

    body.loop_(None, |loop_| {
        let loop_id = loop_.id();

        abi.ptr_const(loop_, stack.temp_lock);
        loop_
            .i32_const(0)
            .i32_const(1)
            .cmpxchg(memory, AtomicWidth::I32, ATOMIC_MEM_ARG)
            .if_else(
                None,
                |body| {
                    abi.ptr_const(body, stack.temp_lock);
                    body.i32_const(1)
                        .i64_const(-1)
                        .atomic_wait(memory, ATOMIC_MEM_ARG, false)
                        .drop()
                        .br(loop_id);
                },
                |_| {},
            );
    });

    block(body);

    abi.ptr_const(body, stack.temp_lock);
    body.i32_const(0)
        .store(memory, StoreKind::I32 { atomic: true }, ATOMIC_MEM_ARG);
    abi.ptr_const(body, stack.temp_lock);
    body.i32_const(1)
        .atomic_notify(memory, ATOMIC_MEM_ARG)
        .drop();
}

#[cfg(test)]
mod tests;
