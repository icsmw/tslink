use node_bindgen::derive::node_bindgen;
use tslink::tslink;

struct StructConstructorA {}

#[tslink(class)]
#[node_bindgen]
impl StructConstructorA {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }
}

struct StructConstructorB {
    _a: i32,
    _b: Option<i32>,
}

#[tslink(class)]
#[node_bindgen]
impl StructConstructorB {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new(a: i32, b: Option<i32>) -> Self {
        Self { _a: a, _b: b }
    }
}
