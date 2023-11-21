use node_bindgen::derive::node_bindgen;
use tslink::tslink;

struct MyScruct {}

#[tslink(class)]
#[node_bindgen]
impl MyScruct {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn inc_my_number(&self, a: i32) -> i32 {
        a + 1
    }
}
