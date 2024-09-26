use crate::{FieldA, FieldB, StructA};
use tslink::tslink;

#[tslink(target = "./output/module_b.ts", module = "module_b")]
pub enum EntityA {
    One,
    Two,
    Three,
}

#[tslink(target = "./output/module_b.ts", module = "module_b")]
pub enum EntityB {
    One(String),
    Two((u32, u32)),
    Three(EntityA),
}

#[tslink(target = "./output/module_b.ts", module = "module_b")]
pub struct OtherStruct {
    pub a: EntityA,
    pub b: EntityB,
    pub c: StructA,
    pub d: FieldA,
    pub e: FieldB,
}
