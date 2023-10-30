use crate::nature::{Nature, Primitive};
#[derive(Clone, Debug)]
pub enum Composite {
    Vec(Option<Box<Nature>>),
    HashMap(Option<Primitive>, Option<Box<Nature>>),
    Tuple(Vec<Box<Nature>>),
    Option(Option<Box<Nature>>),
    Result(Option<Box<Nature>>, Option<Box<Nature>>),
    Undefined,
    // (Vec(Args), Output, async, constructor)
    Func(Vec<Box<Nature>>, Option<Box<Nature>>, bool, bool),
}
