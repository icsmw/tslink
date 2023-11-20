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
    pub d: Option<String>,
}

#[tslink]
#[derive(Serialize, Deserialize)]
struct DataC {
    pub a: u32,
    pub b: u32,
}

#[tslink]
#[derive(Serialize, Deserialize)]
enum EnumA {
    One,
    Two,
    Three,
}

#[tslink]
#[derive(Serialize, Deserialize)]
enum EnumB {
    One(String),
    Two(i32, i32),
    Three(i32),
    Four(Option<u8>),
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
            d: Some("test".to_string()),
        })
    }

    #[tslink(
        data_a = "DataA",
        data_c = "DataC",
        error = "json",
        snake_case_naming,
        exception_suppression
    )]
    #[node_bindgen]
    fn get_multiple_data(&self, data_a: String, data_c: String) -> Result<(i32, i32), ErrorA> {
        Ok((data_a.a + data_c.a as i32, data_a.b + data_c.b as i32))
    }

    #[tslink(
        enum_a = "EnumA",
        result = "json",
        error = "json",
        snake_case_naming,
        exception_suppression
    )]
    #[node_bindgen]
    fn get_enum_a(&self, enum_a: String) -> Result<EnumA, ErrorA> {
        Ok(enum_a)
    }

    #[tslink(
        enum_b = "EnumB",
        result = "json",
        error = "json",
        snake_case_naming,
        exception_suppression
    )]
    #[node_bindgen]
    fn get_enum_b(&self, enum_b: String) -> Result<EnumB, ErrorA> {
        Ok(enum_b)
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
