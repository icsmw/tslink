extern crate tslink;
use tslink::tslink;

#[tslink(target = "./output/module.ts")]
pub const A: &str = "Hello";

#[tslink(target = "./output/module.ts")]
pub const B: u8 = 42;

#[tslink(target = "./output/module.ts")]
pub const C: u32 = 42;

#[tslink(target = "./output/module.ts")]
pub const D: i32 = 42;

#[tslink(target = "./output/module.ts")]
pub const F: [u8; 4] = [1, 2, 3, 4];

#[tslink(target = "./output/module.ts")]
pub const G: [&str; 4] = ["1", "2", "3", "4"];
