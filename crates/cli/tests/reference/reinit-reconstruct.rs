// FLAGS: --target=bundler

// `schedule_reinit()` enables the reinit machinery, so the exported classes
// get `__wbg_inst` stamping and stale-wrapper checks. `ReinitCounter` and
// the `ReinitChild` chain opt into `reinit = "reconstruct"`; `ReinitPlain`
// keeps the default stale-wrapper error.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn reinit_schedule() {
    wasm_bindgen::handler::schedule_reinit();
}

#[wasm_bindgen(reinit = "reconstruct")]
pub struct ReinitCounter {
    count: u32,
}

#[wasm_bindgen]
impl ReinitCounter {
    #[wasm_bindgen(constructor)]
    pub fn new(start: u32, label: &str) -> ReinitCounter {
        let _ = label;
        ReinitCounter { count: start }
    }

    pub fn increment(&mut self) -> u32 {
        self.count += 1;
        self.count
    }

    #[wasm_bindgen(getter)]
    pub fn count(&self) -> u32 {
        self.count
    }

    #[wasm_bindgen(setter)]
    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    pub fn wrapped(start: u32) -> ReinitCounter {
        ReinitCounter { count: start }
    }
}

#[wasm_bindgen]
pub fn reinit_read_counter(counter: &ReinitCounter) -> u32 {
    counter.count
}

#[wasm_bindgen]
pub struct ReinitParent {
    name: String,
}

#[wasm_bindgen]
impl ReinitParent {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> ReinitParent {
        ReinitParent { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen(extends = ReinitParent, reinit = "reconstruct")]
pub struct ReinitChild {
    extra: u32,
}

#[wasm_bindgen]
impl ReinitChild {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, extra: u32) -> ReinitChild {
        ReinitChild {
            parent: ReinitParent::new(name).into(),
            extra,
        }
    }

    pub fn extra(&self) -> u32 {
        self.extra
    }
}

#[wasm_bindgen]
pub struct ReinitPlain {
    value: u32,
}

#[wasm_bindgen]
impl ReinitPlain {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> ReinitPlain {
        ReinitPlain { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
