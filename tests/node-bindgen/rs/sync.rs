use node_bindgen::{
    core::{val::JsEnv, NjError, TryIntoJs},
    derive::node_bindgen,
    sys::napi_value,
};
use serde::{Deserialize, Serialize};
use std::convert::From;
use tslink::tslink;

#[derive(Serialize, Deserialize)]
struct MyCustomError {
    msg: String,
    code: usize,
}

impl From<serde_json::Error> for MyCustomError {
    fn from(value: serde_json::Error) -> Self {
        MyCustomError {
            msg: value.to_string(),
            code: 1,
        }
    }
}

fn test(data: Data) -> Result<String, MyCustomError> {
    serde_json::to_string(&data).map_err(|e| Into::<MyCustomError>::into(e))
}

struct MyCustomErrorWrapped(MyCustomError);

impl From<serde_json::Error> for MyCustomErrorWrapped {
    fn from(value: serde_json::Error) -> Self {
        MyCustomErrorWrapped(MyCustomError {
            msg: value.to_string(),
            code: 1,
        })
    }
}

impl TryIntoJs for MyCustomErrorWrapped {
    /// serialize into json object
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        match serde_json::to_string(&self.0) {
            Ok(s) => js_env.create_string_utf8(&s),
            Err(e) => Err(NjError::Other(format!(
                "Could not convert Callback event to json: {e}"
            ))),
        }
    }
}

// #[tslink]
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
        { Ok::<Option<String>, String>(self.b.clone()) }.expect("bla")
    }

    #[tslink(data = "Data", result = "json", snake_case_naming)]
    #[node_bindgen]
    fn get_data(&self, data: String) -> Result<Data, MyCustomErrorWrapped> {
        Ok(Data {
            a: data.a + 1,
            b: data.b + 1,
            s: format!("{}{}", data.s, data.s),
        })
    }

    #[tslink(data = "Data", result = "json", error = "json", snake_case_naming)]
    #[node_bindgen]
    fn get_data_a(&self, data: String) -> Result<Data, MyCustomError> {
        Ok(Data {
            a: data.a + 1,
            b: data.b + 1,
            s: format!("{}{}", data.s, data.s),
        })
    }
}

fn test2(arg: u8, arg2: u64) -> Result<(), u8> {
    let res: Result<(), String> = (move || {
        Some("ffff").ok_or(String::from("fff"))?;
        println!("{}", arg);
        Ok(())
    })();
    res.map_err(|e: String| 6)
}

// #[tslink(data = "Data", result = "json", snake_case_naming)]
// #[node_bindgen]
// fn get_data_func(data: String, a: i32, b: i32) -> Result<Data, MyCustomError> {
//     Ok(Data {
//         a: data.a + a,
//         b: data.b + b,
//         s: format!("{}{}", data.s, data.s),
//     })
// }

// #[derive(Serialize, Deserialize)]
// struct CustomError {
//     a: String,
//     b: String,
// }

// pub(crate) struct CustomErrorWrapper(pub CustomError);

// impl TryIntoJs for CustomErrorWrapper {
//     fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
//         let value = serde_json::to_value(self.0).map_err(|e| NjError::Other(format!("{e}")))?;
//         value.try_to_js(js_env)
//     }
// }

// #[node_bindgen]
// fn test_error() -> Result<(), CustomErrorWrapper> {
//     Err(CustomErrorWrapper(CustomError {
//         a: "a".to_string(),
//         b: "b".to_string(),
//     }))
// }
