use super::Interpreter;
use crate::{
    context,
    error::E,
    interpreter::Offset,
    nature::{Composite, Nature, Natures, Refered},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    ops::Deref,
};
/*


const nativeModuleRef = native();

const { Struct } = nativeModuleRef;

class StructBound {
    #_origin;
    get prop() {
        return this.#_origin.prop;
    }
    set prop(v) {
        this.#_origin.prop = v;
    }
    constructor() {
        this.#_origin = new Struct();
    }
    init() {
        return this.#_origin.init();
    }
}

exports.Struct = StructBound;
*/

impl Interpreter for Composite {
    fn declaration(
        &self,
        _natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        // match self {
        //     Self::Func(args, out, asyncness, constructor) => {
        //         buf.write_all(format!("\n{name}() {{}}").as_bytes())?;
        //     }
        //     _ => {}
        // }
        Ok(())
    }
}
