// Byte-identical `#[wasm_bindgen(private)]` enums in two modules of one
// crate: the second module's copies get numbered identities.

macro_rules! items {
    ($m:ident, $mode:ident, $level:ident, $union:ident) => {
        pub mod $m {
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(private)]
            #[derive(Clone, Copy)]
            pub enum Mode {
                Fast = "fast",
                Slow = "slow",
            }

            #[wasm_bindgen(private)]
            #[derive(Clone, Copy)]
            pub enum Level {
                Low = 1,
                High = 2,
            }

            #[wasm_bindgen(private)]
            pub enum ModeOrNumber {
                Mode(Mode),
                F64(f64),
            }

            #[wasm_bindgen]
            pub fn $mode(mode: Mode) -> Mode {
                mode
            }

            #[wasm_bindgen]
            pub fn $level(level: Level) -> Level {
                level
            }

            #[wasm_bindgen]
            pub fn $union(value: ModeOrNumber) -> ModeOrNumber {
                value
            }
        }
    };
}

items!(a, a_mode, a_level, a_union);
items!(b, b_mode, b_level, b_union);
