use crate::{
    args::{Args, ArgsWriter},
    types::Types,
};

#[derive(Clone, Debug)]
pub struct Detached {
    pub ty: Types,
    pub args: Args,
}

impl ArgsWriter for Detached {
    fn get_args(&self) -> &Args {
        &self.args
    }
}

impl Detached {
    pub fn new(ty: Types, args: Args) -> Self {
        Self { ty, args }
    }
}
