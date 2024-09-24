use node_bindgen::derive::node_bindgen;
use serde::{Deserialize, Serialize};
use tslink::tslink;

#[tslink]
#[derive(Serialize, Deserialize)]
struct ErrorWithOption {
    msg: Option<String>,
    code: usize,
}

impl From<serde_json::Error> for ErrorWithOption {
    fn from(value: serde_json::Error) -> Self {
        ErrorWithOption {
            msg: Some(value.to_string()),
            code: 1,
        }
    }
}

#[tslink]
#[derive(Serialize, Deserialize)]
struct ObjectWithOptions {
    pub a: Option<i32>,
    pub b: Option<String>,
    pub c: Option<(i32, i32)>,
}

struct StructWithOptions {}

#[tslink(class)]
#[node_bindgen]
impl StructWithOptions {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[tslink(
        options = "ObjectWithOptions",
        error = "json",
        snake_case_naming,
        exception_suppression
    )]
    #[node_bindgen]
    fn parsing_options(&self, options: String) -> Result<i32, ErrorWithOption> {
        Ok(666)
    }

    #[tslink(error = "json", snake_case_naming, exception_suppression)]
    #[node_bindgen]
    fn get_err_with_option_some(&self) -> Result<(), ErrorWithOption> {
        Err(ErrorWithOption {
            msg: Some(String::from("test")),
            code: 1,
        })
    }

    #[tslink(error = "json", snake_case_naming, exception_suppression)]
    #[node_bindgen]
    fn get_err_with_option_none(&self) -> Result<(), ErrorWithOption> {
        Err(ErrorWithOption { msg: None, code: 1 })
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn optional_a(&self, a: Option<i32>, b: Option<i32>) -> i32 {
        if let (Some(a), Some(b)) = (a, b) {
            a + b
        } else {
            a.unwrap_or(b.unwrap_or(1))
        }
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn optional_b(&self, a: Option<i32>, b: Option<i32>) -> (Option<i32>, Option<i32>) {
        (a, b)
    }
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn optional_test_a(a: Option<i32>, b: Option<i32>) -> i32 {
    if let (Some(a), Some(b)) = (a, b) {
        a + b
    } else {
        a.unwrap_or(b.unwrap_or(1))
    }
}

#[tslink(snake_case_naming)]
#[node_bindgen]
fn optional_test_b(a: Option<i32>, b: Option<i32>) -> (Option<i32>, Option<i32>) {
    (a, b)
}
