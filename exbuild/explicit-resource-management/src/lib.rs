use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use wasm_bindgen::prelude::*;

// simple counting allocator tracking
struct Counter;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Relaxed);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Relaxed);
    }
}

#[global_allocator]
static A: Counter = Counter;

#[wasm_bindgen]
pub fn current_allocation() -> usize {
    ALLOCATED.load(Relaxed)
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct MyStruct {
    _x: Vec<i64>,
    _y: Vec<i64>,
    name: String,
}

#[wasm_bindgen]
impl MyStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> MyStruct {
        Self {
            name,
            _x: (0..50).collect(),
            _y: (0..50).collect(),
        }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        log(&format!("Goodbye from {}!", self.name)); // should output "Goodbye from Rust!"
    }
}
