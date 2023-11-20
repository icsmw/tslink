extern crate tslink;
use tslink::tslink;

#[tslink(class)]
struct Struct {
    field_a: i32,
    field_b: u8,
}

#[tslink(class)]
impl Struct {
    #[tslink]
    fn mythod_a(&mut self) -> i32 {
        666
    }
}
