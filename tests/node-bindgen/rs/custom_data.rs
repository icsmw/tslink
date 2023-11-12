use node_bindgen::derive::node_bindgen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tslink::tslink;

#[tslink]
#[derive(Serialize, Deserialize)]
struct ErrorA {
    msg: String,
    code: usize,
}

impl From<serde_json::Error> for ErrorA {
    fn from(value: serde_json::Error) -> Self {
        ErrorA {
            msg: value.to_string(),
            code: 1,
        }
    }
}

#[tslink]
#[derive(Serialize, Deserialize)]
struct ErrorB {
    msg: String,
    code: usize,
    err: ErrorA,
}

impl From<serde_json::Error> for ErrorB {
    fn from(value: serde_json::Error) -> Self {
        ErrorB {
            msg: value.to_string(),
            code: 1,
            err: ErrorA {
                msg: "test".to_string(),
                code: 1,
            },
        }
    }
}

#[tslink]
#[derive(Serialize, Deserialize)]
struct DataA {
    pub a: i32,
    pub b: i32,
    pub c: String,
}

#[tslink]
#[derive(Serialize, Deserialize)]
struct DataB {
    pub a: DataA,
    pub b: Vec<DataA>,
    pub c: HashMap<String, DataA>,
}

struct StructCustomData {}

#[tslink(class)]
#[node_bindgen]
impl StructCustomData {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[tslink(
        data = "DataA",
        result = "json",
        error = "json",
        snake_case_naming,
        exception_suppression
    )]
    #[node_bindgen]
    fn get_data(&self, data: String) -> Result<DataA, ErrorA> {
        Ok(DataA {
            a: data.a + 1,
            b: data.b + 1,
            c: format!("{}{}", data.c, data.c),
        })
    }

    #[tslink(
        data = "DataA",
        result = "json",
        error = "json",
        snake_case_naming,
        exception_suppression
    )]
    #[node_bindgen]
    fn get_data_a(&self, data: String) -> Result<DataB, ErrorA> {
        let mut c: HashMap<String, DataA> = HashMap::new();
        c.insert(
            "first".to_string(),
            DataA {
                a: data.a + 1,
                b: data.b + 1,
                c: format!("{}{}", data.c, data.c),
            },
        );
        c.insert(
            "second".to_string(),
            DataA {
                a: data.a + 1,
                b: data.b + 1,
                c: format!("{}{}", data.c, data.c),
            },
        );
        Ok(DataB {
            a: DataA {
                a: data.a + 1,
                b: data.b + 1,
                c: format!("{}{}", data.c, data.c),
            },
            b: vec![
                DataA {
                    a: data.a + 1,
                    b: data.b + 1,
                    c: format!("{}{}", data.c, data.c),
                },
                DataA {
                    a: data.a + 1,
                    b: data.b + 1,
                    c: format!("{}{}", data.c, data.c),
                },
            ],
            c,
        })
    }

    #[tslink(error = "json", snake_case_naming, exception_suppression)]
    #[node_bindgen]
    fn test_of_error_support_ok(&self) -> Result<i32, ErrorB> {
        Ok(666)
    }

    #[tslink(error = "json", snake_case_naming, exception_suppression)]
    #[node_bindgen]
    fn test_of_error_support_err(&self) -> Result<i32, ErrorB> {
        Err(ErrorB {
            msg: "test".to_string(),
            code: 666,
            err: ErrorA {
                msg: "Error".to_string(),
                code: 666,
            },
        })
    }
}
