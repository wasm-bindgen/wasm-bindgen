use wasm_bindgen::prelude::*;

// ===== Type Parameter Bounds Tests =====

#[wasm_bindgen]
extern "C" {
    // Test: Single trait bound on type parameter
    pub type TypeSingleBound<T: Clone>;
    
    // Test: Multiple trait bounds on type parameter
    pub type TypeMultipleBounds<T: Clone + Send>;
    
    // Test: Trait bounds with lifetimes (double fail - both lifetime and bounds)
    pub type TypeBoundWithLifetime<T: 'static + Clone>;
    
    // Test: Higher-ranked trait bounds
    pub type TypeHigherRankedBound<T: for<'a> Fn(&'a str)>;
    
    // Test: Associated type bounds
    pub type TypeAssociatedTypeBound<T: Iterator<Item = u32>>;
    
    // Test: ?Sized bounds
    pub type TypeMaybeSized<T: ?Sized>;
}

// ===== Where Clause Tests =====

#[wasm_bindgen]
extern "C" {
    // Test: Simple where clause on type
    pub type TypeSimpleWhere<T> where T: Clone;
    
    // Test: Multiple predicates in where clause
    pub type TypeMultipleWhere<T, U> where T: Clone, U: Send;
    
    // Test: Where clause with lifetime bounds
    pub type TypeWhereLifetimeBound<T> where T: 'static;
    
    // Test: Where clause with associated types
    pub type TypeWhereAssocType<T> where T: Iterator<Item = u32>;
    
    // Test: Function with where clause
    pub fn func_where_clause<T, U>(x: T) -> U where T: Clone, U: Default;
    
    // Test: Where clause with higher-ranked lifetimes
    pub type TypeWhereHigherRanked<T> where for<'a> T: Fn(&'a str);
}

// ===== Lifetime Parameters Tests =====

#[wasm_bindgen]
extern "C" {
    // Test: Single lifetime parameter
    pub type TypeSingleLifetime<'a>;
    
    // Test: Multiple lifetime parameters
    pub type TypeMultipleLifetimes<'a, 'b, T>;
    
    // Test: Lifetime bounds
    pub type TypeLifetimeBounds<'a: 'b, 'b, T>;
    
    // Test: Function with lifetime parameters
    pub fn func_lifetime<'a, T>(x: &'a T) -> &'a T;
    
    // Test: Static lifetime in generics
    pub type TypeStaticLifetime<'static, T>;
}

// ===== Const Parameters Tests =====

#[wasm_bindgen]
extern "C" {
    // Test: Simple const parameter
    pub type TypeConstParam<const N: usize>;
    
    // Test: Multiple const parameters
    pub type TypeMultipleConst<const N: usize, const M: i32>;
    
    // Test: Mixed const and type parameters
    pub type TypeMixedConstType<T, const N: usize>;
    
    // Test: Function with const parameter
    pub fn func_const_param<const N: usize>() -> [u8; N];
    
    // Test: Const parameter with complex type
    pub type TypeConstBool<const B: bool>;
}

// ===== Method Self Generics Tests =====

#[wasm_bindgen]
extern "C" {
    pub type GenericClass<T, U>;
    
    #[wasm_bindgen(method)]
    pub fn method_missing_generics(this: &GenericClass) -> u32;
    
    #[wasm_bindgen(method)]
    pub fn method_wrong_generic_args(this: &GenericClass<String>) -> u32;
    
    #[wasm_bindgen(method)]
    pub fn method_extra_bounds<V: Clone>(this: &GenericClass<T, U>, v: V) -> u32;
    
    #[wasm_bindgen(method)]
    pub fn method_where_clause_mixing<V>(this: &GenericClass<T, U>, v: V) -> u32 
    where 
        T: Clone,
        V: Send;
}

// ===== Complex Trait Bounds Tests =====

#[wasm_bindgen]
extern "C" {
    // Test: Trait object bounds
    pub type TypeTraitObject<T: dyn Clone>;
    
    // Test: Impl trait in bounds
    pub type TypeImplTrait<T: impl Clone>;
    
    // Test: Negative bounds (unstable feature)
    pub type TypeNegativeBound<T: !Send>;
    
    // Test: Parenthesized bounds
    pub type TypeParenthesizedBound<T: (Clone)>;
    
    // Test: Path with generics in bounds
    pub type TypeGenericBound<T: Iterator<Item = String>>;
}

// ===== Mixed Invalid Cases =====

#[wasm_bindgen]
extern "C" {
    // Test: Everything combined
    pub type TypeEverything<'a, T: Clone + 'a, const N: usize> 
    where 
        T: Iterator<Item = &'a str>,
        for<'b> T: Fn(&'b str) -> &'b str;
    
    // Test: Function with all invalid features
    pub fn func_everything<'a, T: Clone, const N: usize>(x: &'a T) -> [T; N]
    where
        T: 'a + Send;
}

// ===== Valid Cases (should pass) =====

#[wasm_bindgen]
extern "C" {
    // Valid: Simple generic type without bounds
    pub type ValidGenericType<T>;
    
    // Valid: Multiple type parameters without bounds
    pub type ValidMultipleParams<T, U, V>;
    
    // Valid: Generic with default
    pub type ValidWithDefault<T = JsValue>;
    
    // Valid: Function with simple generics
    pub fn valid_generic_func<T>() -> ValidGenericType<T>;
    
    // Valid: Method with matching generics
    #[wasm_bindgen(method)]
    pub fn valid_method(this: &GenericClass<T, U>) -> u32;
}

fn main() {}