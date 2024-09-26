use tslink::tslink;

#[tslink(target = "./output/module_a.ts", module = "module_a")]
pub enum FieldA {
    One,
    Two,
    Three,
}

#[tslink(target = "./output/module_a.ts", module = "module_a")]
pub enum FieldB {
    One(String),
    Two((u32, u32)),
    Three(FieldA),
}

#[tslink(target = "./output/module_a.ts", module = "module_a")]
pub struct StructA {
    pub a: FieldA,
    pub b: FieldB,
}
