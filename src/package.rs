use crate::{
    defs::{Entities, Entity},
    CONFIG,
};
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{BufWriter, Error, ErrorKind, Write},
};
use toml::Value;

fn value(value: &Value, key: &str) -> Result<String, Error> {
    Ok(value
        .get(key)
        .ok_or(Error::new(
            ErrorKind::Other,
            format!("Fail to find \"{key}\" in [package] section of Cargo.toml"),
        ))?
        .as_str()
        .ok_or(Error::new(
            ErrorKind::Other,
            format!("Fail to convert \"{key}\" into string ([package] section of Cargo.toml)"),
        ))?
        .to_string())
}
pub fn create() -> Result<(), Error> {
    let config = CONFIG
        .read()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    let package = config.get_cargo().get("package").ok_or(Error::new(
        ErrorKind::Other,
        "Fail to find [package] in Cargo.toml",
    ))?;
    let name = value(package, "name")?;
    let version = value(package, "version")?;
    let dist = config.node_mod_dist.clone().ok_or(Error::new(
        ErrorKind::Other,
        "No path to folder with node module. Set correct path in tslink.toml; field \"node\"",
    ))?;
    let node_module = config.node_mod_filename.clone().ok_or(Error::new(
        ErrorKind::Other,
        "No node module file name. Set correct path in tslink.toml; field \"node\"",
    ))?;
    drop(config);
    let package_file = dist.join("package.json");
    if package_file.exists() {
        fs::remove_file(&package_file)?;
    }
    File::create(&package_file)?;
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&package_file)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(
        format!(
            "{{
    \"name\": \"{name}\",
    \"version\": \"{version}\",
    \"files\": [
        \"{node_module}\",
        \"lib.js\",
        \"lib.d.ts\"
    ],
    \"module\": \"lib.js\",
    \"types\": \"lib.d.ts\"
}}"
        )
        .as_bytes(),
    )?;
    buf_writer.flush()
}
