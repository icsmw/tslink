use node_bindgen::derive::node_bindgen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tslink::tslink;

#[tslink]
#[derive(Serialize, Deserialize)]
struct AsyncErrorA {
    msg: String,
    code: usize,
}

impl From<serde_json::Error> for AsyncErrorA {
    fn from(value: serde_json::Error) -> Self {
        AsyncErrorA {
            msg: value.to_string(),
            code: 1,
        }
    }
}

#[tslink]
#[derive(Serialize, Deserialize)]
struct AsyncDataA {
    pub a: i32,
    pub b: i32,
    pub c: String,
}

#[tslink]
#[derive(Serialize, Deserialize)]
struct AsyncDataB {
    pub a: AsyncDataA,
    pub b: Vec<AsyncDataA>,
    pub c: HashMap<String, AsyncDataA>,
    pub d: Option<String>,
}

struct StructAsyncMethods {}

#[tslink(class)]
#[node_bindgen]
impl StructAsyncMethods {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[tslink(
        data = "AsyncDataA",
        result = "json",
        error = "json",
        snake_case_naming,
        exception_suppression
    )]
    #[node_bindgen]
    async fn get_data(&self, data: String) -> Result<AsyncDataA, AsyncErrorA> {
        Ok(AsyncDataA {
            a: data.a + 1,
            b: data.b + 1,
            c: format!("{}{}", data.c, data.c),
        })
    }

    #[tslink(
        data = "AsyncDataA",
        result = "json",
        error = "json",
        snake_case_naming,
        exception_suppression
    )]
    #[node_bindgen]
    async fn get_data_a(&self, data: String) -> Result<AsyncDataB, AsyncErrorA> {
        let mut c: HashMap<String, AsyncDataA> = HashMap::new();
        c.insert(
            "first".to_string(),
            AsyncDataA {
                a: data.a + 1,
                b: data.b + 1,
                c: format!("{}{}", data.c, data.c),
            },
        );
        c.insert(
            "second".to_string(),
            AsyncDataA {
                a: data.a + 1,
                b: data.b + 1,
                c: format!("{}{}", data.c, data.c),
            },
        );
        Ok(AsyncDataB {
            a: AsyncDataA {
                a: data.a + 1,
                b: data.b + 1,
                c: format!("{}{}", data.c, data.c),
            },
            b: vec![
                AsyncDataA {
                    a: data.a + 1,
                    b: data.b + 1,
                    c: format!("{}{}", data.c, data.c),
                },
                AsyncDataA {
                    a: data.a + 1,
                    b: data.b + 1,
                    c: format!("{}{}", data.c, data.c),
                },
            ],
            c,
            d: Some("test".to_string()),
        })
    }

    #[tslink(error = "json", snake_case_naming, exception_suppression)]
    #[node_bindgen]
    async fn test_of_error_support_ok(&self) -> Result<i32, AsyncErrorA> {
        Ok(666)
    }

    #[tslink(error = "json", snake_case_naming, exception_suppression)]
    #[node_bindgen]
    async fn test_of_error_support_err(&self) -> Result<i32, AsyncErrorA> {
        Err(AsyncErrorA {
            msg: "test".to_string(),
            code: 666,
        })
    }
}
