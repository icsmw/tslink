use node_bindgen::derive::node_bindgen;
use serde::{Deserialize, Serialize};
use tslink::tslink;

#[tslink]
#[derive(Serialize, Deserialize)]
struct Data {
    pub a: i32,
    pub b: i32,
    pub s: String,
}

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

    #[tslink(data = "Data")]
    #[node_bindgen]
    fn get_data(&self, data: String) -> Result<(), String> {
        println!("{}", data.s);
        Ok(())
    }
}
