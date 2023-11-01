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
        { Ok::<Option<String>, String>(self.b.clone()) }.expect("bla")
    }

    // fn m(data: String) -> Result<String, String> {
    //     match {
    //         use serde_json;
    //         #[allow(unused_mut)]
    //         let mut data: Data = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    //         println!("{}", data.s);
    //         Ok(Data {
    //             a: 1,
    //             b: 2,
    //             s: String::from("test"),
    //         })
    //     } {
    //         Ok(res) => Ok(serde_json::to_string(&res).map_err(|e| e.to_string())?),
    //         Err(err) => Err(err),
    //     }
    // }

    #[tslink(data = "Data", result = "json", snake_case_naming)]
    #[node_bindgen]
    fn get_data(&self, data: String) -> Result<Data, String> {
        println!("{}", data.s);
        Ok(Data {
            a: 1,
            b: 2,
            s: String::from("test"),
        })
    }
}

#[tslink(data = "Data", result = "json", snake_case_naming)]
#[node_bindgen]
fn get_data_func(data: String, a: i64, b: i64) -> Result<Data, String> {
    println!("{}", data.s);
    Ok(Data {
        a: 1,
        b: 2,
        s: String::from("test"),
    })
}
