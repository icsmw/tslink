use node_bindgen::derive::node_bindgen;
use tslink::tslink;

struct Struct {
    a: String,
    b: Option<String>,
}

#[tslink(class)]
#[node_bindgen]
impl Struct {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new(a: String, b: Option<String>) -> Self {
        Self { a, b }
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn get_a(&self) -> String {
        self.a.clone()
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn get_b(&self) -> Option<String> {
        self.b.clone()
    }
}
