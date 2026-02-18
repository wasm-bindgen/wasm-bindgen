#![cfg(target_arch = "wasm32")]
#![allow(renamed_and_removed_lints)] // clippy::drop_ref will be renamed to drop_ref
#![allow(clippy::drop_ref, clippy::drop_non_drop)]

extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;
extern crate wasm_bindgen_test_crate_a;
extern crate wasm_bindgen_test_crate_b;

#[cfg(feature = "serde-serialize")]
#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

#[path = "3944.rs"]
pub mod _3944;
pub mod api;
pub mod arg_names;
pub mod async_vecs;
pub mod bigint;
pub mod char;
pub mod classes;
pub mod closures;
pub mod comments;
pub mod duplicate_deps;
pub mod duplicates;
pub mod enum_vecs;
pub mod enums;
#[path = "final.rs"]
pub mod final_;
pub mod futures;
pub mod gc;
pub mod generics;
pub mod getters_and_setters;
pub mod ignore;
pub mod import_class;
pub mod imports;
pub mod inner_self;
pub mod intrinsics;
pub mod js_keywords;
pub mod js_namespace_exports;
pub mod js_objects;
pub mod js_vec;
pub mod jscast;
pub mod link_to;
pub mod macro_rules;
pub mod math;
pub mod memory_growth;
pub mod no_shims;
pub mod node;
pub mod nullable;
pub mod option;
pub mod optional_primitives;
pub mod reexport;
pub mod result;
pub mod result_jserror;
pub mod rethrow;
pub mod should_panic;
pub mod simple;
pub mod slice;
pub mod slice_jsvalue;
pub mod string_vecs;
pub mod struct_vecs;
pub mod structural;
pub mod truthy_falsy;
pub mod try_from_js_value;
#[cfg(all(panic = "unwind", feature = "std"))]
pub mod unwind;
pub mod usize;
pub mod validate_prt;
pub mod variadic;
pub mod vendor_prefix;

// should not be executed
#[wasm_bindgen(start)]
fn start() {
    panic!();
}
