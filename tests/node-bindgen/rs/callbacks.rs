use node_bindgen::derive::node_bindgen;
use tslink::tslink;

struct StructCallback {}

#[tslink(class)]
#[node_bindgen]
impl StructCallback {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn test_a<F: Fn(i32, i32) + Send + 'static>(&mut self, callback: F) {
        callback(666, 666);
    }
}
