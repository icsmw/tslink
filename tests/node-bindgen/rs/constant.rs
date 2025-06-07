use tslink::tslink;

#[tslink]
pub const A: &str = "Hello";

#[tslink]
pub const B: u8 = 42;
