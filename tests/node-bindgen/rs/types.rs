use node_bindgen::derive::node_bindgen;
use tslink::tslink;

#[tslink(snake_case_naming)]
#[node_bindgen]
fn types_a(a: i32, b: i32) -> (i32, i32) {
    (a, b)
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn types_b(a: Option<i32>) -> Option<i32> {
    a
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn types_c(a: Option<i32>, b: Option<i32>) -> (Option<i32>, Option<i32>) {
    (a, b)
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn types_d(a: Vec<i32>) -> Vec<i32> {
    a
}
