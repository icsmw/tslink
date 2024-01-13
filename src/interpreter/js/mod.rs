mod refered;

use crate::{
    config,
    error::E,
    interpreter::Offset,
    nature::{Nature, Natures, Refered},
};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
};

pub trait Interpreter {
    fn declaration(
        &self,
        _natures: &Natures,
        _buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), E> {
        Ok(())
    }
}

pub fn write(natures: &Natures) -> Result<(), E> {
    let config = config::get()?;
    let dist = config
        .node_mod_dist
        .clone()
        .ok_or(E::InvalidConfiguration(String::from(
            "No path to folder with node module. Set correct path in [tslink] of Cargo.toml; field \"node\"",
        )))?;
    let node_module = config
        .node_mod_filename
        .clone()
        .ok_or(E::InvalidConfiguration(String::from(
            "No node module file name. Set correct path in [tslink] of Cargo.toml; field \"node\"",
        )))?;
    let lib_file = dist.join("lib.js");
    drop(config);
    if lib_file.exists() {
        fs::remove_file(&lib_file)?;
    }
    File::create(&lib_file)?;
    let file = OpenOptions::new().append(true).open(&lib_file)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(
        format!(
            "\"use strict\";
Object.defineProperty(exports, \"__esModule\", {{ value: true }});

const path = require(\"path\");
const fs = require(\"fs\");

function native() {{
    const modulePath = path.resolve(module.path, './{node_module}');
    if (!fs.existsSync(modulePath)) {{
        throw new Error(`Fail to find native module in: ${{modulePath}}`);
    }}
    return require(modulePath);
}}
const nativeModuleRef = native();
"
        )
        .as_bytes(),
    )?;
    for en_nature in natures.filter(|n| matches!(n, Nature::Refered(Refered::Enum(_, _, _)))) {
        if let Nature::Refered(en_nature) = en_nature {
            if en_nature.is_enum_flat()? {
                en_nature.declaration(natures, &mut buf_writer, Offset::new())?;
            }
        }
    }
    for (_, filtered) in natures.iter().filter(|(_, nature)| match nature {
        Nature::Refered(Refered::Struct(_, context, _)) => context.as_class(),
        Nature::Refered(Refered::Func(_, _, _)) => true,
        _ => false,
    }) {
        if let Nature::Refered(nature) = filtered {
            nature.declaration(natures, &mut buf_writer, Offset::new())?;
        }
    }
    buf_writer.flush()?;
    Ok(())
}
