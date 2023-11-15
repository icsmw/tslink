use node_bindgen::derive::node_bindgen;
use tslink::tslink;

#[tslink(snake_case_naming)]
#[node_bindgen]
fn types_a(a: i64, b: i32) -> (i64, i32) {
    (a, b)
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn types_b(a: Option<i64>) -> Option<i64> {
    a
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn types_c(a: Option<i64>, b: Option<i64>) -> (Option<i64>, Option<i64>) {
    (a, b)
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn types_d(a: Vec<i64>) -> Vec<i64> {
    a
}
