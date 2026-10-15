//! Same-named `#[wasm_bindgen(private)]` enums in two modules of one crate.
//! The two modules use different values, so a binding wired to the other
//! module's definition shows up as a wrong value rather than passing.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/same_name_items.js")]
extern "C" {
    fn same_name_call_exports();
}

pub mod a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(private)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SameNameMode {
        Fast = "fast",
        Slow = "slow",
    }

    #[wasm_bindgen(private)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SameNameLevel {
        Low = 1,
        High = 2,
    }

    #[wasm_bindgen(private)]
    #[derive(Debug, PartialEq)]
    pub enum SameNameUnion {
        Mode(SameNameMode),
        Number(f64),
    }

    #[wasm_bindgen(module = "tests/wasm/same_name_items.js")]
    extern "C" {
        pub fn same_name_a_import(
            mode: SameNameMode,
            level: SameNameLevel,
            union: SameNameUnion,
        ) -> SameNameUnion;
    }

    #[wasm_bindgen]
    pub fn same_name_a_mode(mode: SameNameMode) -> SameNameMode {
        mode
    }

    #[wasm_bindgen]
    pub fn same_name_a_level(level: SameNameLevel) -> SameNameLevel {
        level
    }

    #[wasm_bindgen]
    pub fn same_name_a_union(union: SameNameUnion) -> SameNameUnion {
        union
    }
}

pub mod b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(private)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SameNameMode {
        On = "on",
        Off = "off",
    }

    #[wasm_bindgen(private)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SameNameLevel {
        Off = 10,
        On = 20,
    }

    #[wasm_bindgen(private)]
    #[derive(Debug, PartialEq)]
    pub enum SameNameUnion {
        Mode(SameNameMode),
        Number(f64),
    }

    #[wasm_bindgen(module = "tests/wasm/same_name_items.js")]
    extern "C" {
        pub fn same_name_b_import(
            mode: SameNameMode,
            level: SameNameLevel,
            union: SameNameUnion,
        ) -> SameNameUnion;
    }

    #[wasm_bindgen]
    pub fn same_name_b_mode(mode: SameNameMode) -> SameNameMode {
        mode
    }

    #[wasm_bindgen]
    pub fn same_name_b_level(level: SameNameLevel) -> SameNameLevel {
        level
    }

    #[wasm_bindgen]
    pub fn same_name_b_union(union: SameNameUnion) -> SameNameUnion {
        union
    }
}

#[wasm_bindgen_test]
fn imports_receive_their_own_module_values() {
    assert_eq!(
        a::same_name_a_import(
            a::SameNameMode::Slow,
            a::SameNameLevel::High,
            a::SameNameUnion::Mode(a::SameNameMode::Fast),
        ),
        a::SameNameUnion::Mode(a::SameNameMode::Slow),
    );
    assert_eq!(
        b::same_name_b_import(
            b::SameNameMode::Off,
            b::SameNameLevel::On,
            b::SameNameUnion::Mode(b::SameNameMode::On),
        ),
        b::SameNameUnion::Number(2.5),
    );
}

#[wasm_bindgen_test]
fn exports_round_trip_their_own_module_values() {
    same_name_call_exports();
}
