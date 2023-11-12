use node_bindgen::derive::node_bindgen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tslink::tslink;

#[tslink]
#[derive(Serialize, Deserialize)]
struct ErrorC {
    msg: String,
    code: usize,
}

impl From<serde_json::Error> for ErrorC {
    fn from(value: serde_json::Error) -> Self {
        ErrorC {
            msg: value.to_string(),
            code: 1,
        }
    }
}

struct StructErrorHandeling {}

#[tslink(class)]
#[node_bindgen]
impl StructErrorHandeling {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[tslink(snake_case_naming, exception_suppression)]
    #[node_bindgen]
    fn test_of_exception_suppression(&self) -> Result<i32, String> {
        Err("test".to_string())
    }

    #[tslink(error = "json", snake_case_naming, exception_suppression)]
    #[node_bindgen]
    fn test_of_exception_suppression_with_custom_error(&self) -> Result<i32, ErrorC> {
        Err(ErrorC {
            msg: "test".to_string(),
            code: 666,
        })
    }

    #[tslink(snake_case_naming)]
    #[node_bindgen]
    fn test_of_exception(&self) -> Result<i32, String> {
        Err("test".to_string())
    }

    #[tslink(error = "json", snake_case_naming)]
    #[node_bindgen]
    fn test_of_exception_with_custom_error(&self) -> Result<i32, ErrorC> {
        Err(ErrorC {
            msg: "test".to_string(),
            code: 666,
        })
    }
}
