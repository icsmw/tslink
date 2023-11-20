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

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn test_b<F: Fn(Option<i32>, Option<i32>) + Send + 'static>(&mut self, callback: F) {
        callback(Some(666), Some(666));
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn test_c<F: Fn(Option<i32>, Option<i32>) + Send + 'static>(&mut self, callback: F) {
        callback(None, Some(666));
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn test_d<F: Fn(Option<i32>, Option<i32>) + Send + 'static>(&mut self, callback: F) {
        callback(Some(666), None);
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn test_e<F: Fn(Option<i32>, Option<i32>) + Send + 'static>(&mut self, callback: F) {
        callback(None, None);
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn test_f<F: Fn(i32) + Send + 'static, D: Fn(String) + Send + 'static>(
        &mut self,
        callback_f: F,
        callback_d: D,
    ) {
        callback_f(666);
        callback_d("test".to_string());
    }
}
