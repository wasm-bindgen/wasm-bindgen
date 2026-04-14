use wasm_bindgen::prelude::*;

/// Two structs with the same js_name in different namespaces should not collide.

#[derive(Clone)]
#[wasm_bindgen(js_namespace = foo, js_name = "Point")]
pub struct FooPoint {
    pub x: f64,
}

#[wasm_bindgen]
impl FooPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64) -> FooPoint {
        FooPoint { x }
    }
}

#[derive(Clone)]
#[wasm_bindgen(js_namespace = bar, js_name = "Point")]
pub struct BarPoint {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl BarPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> BarPoint {
        BarPoint { x, y }
    }
}

/// Two enums with the same js_name in different namespaces should not collide.

#[derive(Clone, Copy)]
#[wasm_bindgen(js_namespace = foo, js_name = "Status")]
pub enum FooStatus {
    Active = 0,
    Inactive = 1,
}

#[derive(Clone, Copy)]
#[wasm_bindgen(js_namespace = bar, js_name = "Status")]
pub enum BarStatus {
    Pending = 0,
    Complete = 1,
    Failed = 2,
}

/// Two functions with the same js_name in different namespaces should not collide.

#[wasm_bindgen(js_namespace = foo, js_name = "greet")]
pub fn foo_greet() -> String {
    "hello from foo".to_string()
}

#[wasm_bindgen(js_namespace = bar, js_name = "greet")]
pub fn bar_greet() -> String {
    "hello from bar".to_string()
}

/// Two structs with the same js_name in nested namespaces should not collide.

#[derive(Clone)]
#[wasm_bindgen(js_namespace = ["foo", "nested"], js_name = "Point")]
pub struct FooNestedPoint {
    pub z: f64,
}

#[wasm_bindgen]
impl FooNestedPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(z: f64) -> FooNestedPoint {
        FooNestedPoint { z }
    }
}

#[derive(Clone)]
#[wasm_bindgen(js_namespace = ["bar", "nested"], js_name = "Point")]
pub struct BarNestedPoint {
    pub magnitude: f64,
}

#[wasm_bindgen]
impl BarNestedPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(magnitude: f64) -> BarNestedPoint {
        BarNestedPoint { magnitude }
    }
}

/// Same js_name reused across different namespace depths should not collide.

#[derive(Clone, Copy)]
#[wasm_bindgen(js_namespace = ["foo", "nested"], js_name = "Status")]
pub enum FooNestedStatus {
    Cold = 0,
    Warm = 1,
}

/// Different exported kinds with the same js_name across namespace depths should not collide.

#[wasm_bindgen(js_namespace = ["foo", "nested", "deep"], js_name = "Status")]
pub fn foo_nested_status() -> String {
    "nested status function".to_string()
}

#[wasm_bindgen(js_namespace = ["foo", "nested"], js_name = "greet")]
pub fn foo_nested_greet() -> String {
    "hello from foo nested".to_string()
}

#[wasm_bindgen(js_namespace = foo, js_name = "RefToBar")]
pub struct FooBridge {
    bar_point: BarPoint,
    bar_status: BarStatus,
}

#[wasm_bindgen(js_class = "RefToBar")]
impl FooBridge {
    #[wasm_bindgen(constructor)]
    pub fn new(bar_point: BarPoint, bar_status: BarStatus) -> FooBridge {
        FooBridge {
            bar_point,
            bar_status,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn bar_point(&self) -> BarPoint {
        self.bar_point.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_bar_point(&mut self, bar_point: BarPoint) {
        self.bar_point = bar_point;
    }

    #[wasm_bindgen(getter)]
    pub fn bar_status(&self) -> BarStatus {
        self.bar_status
    }

    #[wasm_bindgen(setter)]
    pub fn set_bar_status(&mut self, bar_status: BarStatus) {
        self.bar_status = bar_status;
    }

    pub fn reflect_point(&self, point: BarPoint) -> BarPoint {
        point
    }

    pub fn reflect_status(&self, status: BarStatus) -> BarStatus {
        status
    }
}

#[wasm_bindgen(js_namespace = bar, js_name = "RefToFoo")]
pub struct BarBridge {
    foo_point: FooPoint,
    foo_status: FooStatus,
}

#[wasm_bindgen(js_class = "RefToFoo")]
impl BarBridge {
    #[wasm_bindgen(constructor)]
    pub fn new(foo_point: FooPoint, foo_status: FooStatus) -> BarBridge {
        BarBridge {
            foo_point,
            foo_status,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn foo_point(&self) -> FooPoint {
        self.foo_point.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_foo_point(&mut self, foo_point: FooPoint) {
        self.foo_point = foo_point;
    }

    #[wasm_bindgen(getter)]
    pub fn foo_status(&self) -> FooStatus {
        self.foo_status
    }

    #[wasm_bindgen(setter)]
    pub fn set_foo_status(&mut self, foo_status: FooStatus) {
        self.foo_status = foo_status;
    }

    pub fn reflect_point(&self, point: FooPoint) -> FooPoint {
        point
    }

    pub fn reflect_status(&self, status: FooStatus) -> FooStatus {
        status
    }
}

#[wasm_bindgen]
pub struct NamespaceConsumer {
    foo_x: f64,
    bar_x: f64,
    bar_y: f64,
    foo_status: FooStatus,
    bar_status: BarStatus,
    foo_points: Vec<FooPoint>,
    bar_points: Vec<BarPoint>,
}

#[wasm_bindgen]
impl NamespaceConsumer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        foo_point: FooPoint,
        bar_point: BarPoint,
        foo_status: FooStatus,
        bar_status: BarStatus,
    ) -> NamespaceConsumer {
        NamespaceConsumer {
            foo_x: foo_point.x,
            bar_x: bar_point.x,
            bar_y: bar_point.y,
            foo_status,
            bar_status,
            foo_points: vec![foo_point],
            bar_points: vec![bar_point],
        }
    }

    #[wasm_bindgen(getter)]
    pub fn foo_point(&self) -> FooPoint {
        FooPoint { x: self.foo_x }
    }

    #[wasm_bindgen(setter)]
    pub fn set_foo_point(&mut self, foo_point: FooPoint) {
        self.foo_x = foo_point.x;
    }

    #[wasm_bindgen(getter)]
    pub fn bar_point(&self) -> BarPoint {
        BarPoint {
            x: self.bar_x,
            y: self.bar_y,
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_bar_point(&mut self, bar_point: BarPoint) {
        self.bar_x = bar_point.x;
        self.bar_y = bar_point.y;
    }

    #[wasm_bindgen(getter)]
    pub fn foo_status(&self) -> FooStatus {
        self.foo_status
    }

    #[wasm_bindgen(setter)]
    pub fn set_foo_status(&mut self, foo_status: FooStatus) {
        self.foo_status = foo_status;
    }

    #[wasm_bindgen(getter)]
    pub fn bar_status(&self) -> BarStatus {
        self.bar_status
    }

    #[wasm_bindgen(setter)]
    pub fn set_bar_status(&mut self, bar_status: BarStatus) {
        self.bar_status = bar_status;
    }

    #[wasm_bindgen(getter)]
    pub fn foo_points(&self) -> Vec<FooPoint> {
        self.foo_points.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_foo_points(&mut self, foo_points: Vec<FooPoint>) {
        self.foo_points = foo_points;
    }

    #[wasm_bindgen(getter)]
    pub fn bar_points(&self) -> Vec<BarPoint> {
        self.bar_points.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_bar_points(&mut self, bar_points: Vec<BarPoint>) {
        self.bar_points = bar_points;
    }

    pub fn rotate_foo(&self, point: FooPoint) -> FooPoint {
        FooPoint { x: point.x + 1.0 }
    }

    pub fn normalize_bar(&self, point: BarPoint) -> BarPoint {
        BarPoint {
            x: point.x - self.bar_x,
            y: point.y - self.bar_y,
        }
    }

    pub fn next_foo_status(&self, status: FooStatus) -> FooStatus {
        match status {
            FooStatus::Active => FooStatus::Inactive,
            FooStatus::Inactive => FooStatus::Active,
        }
    }

    pub fn next_bar_status(&self, status: BarStatus) -> BarStatus {
        match status {
            BarStatus::Pending => BarStatus::Complete,
            BarStatus::Complete => BarStatus::Failed,
            BarStatus::Failed => BarStatus::Pending,
        }
    }

    pub fn duplicate_foo_points(&self, points: Vec<FooPoint>) -> Vec<FooPoint> {
        points
    }

    pub fn duplicate_bar_points(&self, points: Vec<BarPoint>) -> Vec<BarPoint> {
        points
    }
}

/// A top-level export colliding with an inner namespace export should not collide.

#[wasm_bindgen(js_name = "Point")]
pub struct TopLevelPoint {
    pub value: f64,
}

#[wasm_bindgen]
impl TopLevelPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(value: f64) -> TopLevelPoint {
        TopLevelPoint { value }
    }
}

/// A top-level enum colliding with an inner namespace export should not collide.

#[derive(Clone, Copy)]
#[wasm_bindgen(js_name = "Status")]
pub enum TopLevelStatus {
    Ready = 0,
    Done = 1,
}

/// A top-level function colliding with an inner namespace export should not collide.

#[wasm_bindgen(js_name = "greet")]
pub fn top_level_greet() -> String {
    "hello from top level".to_string()
}
