use crate::{config, error::E};
use std::{
    fs,
    fs::{create_dir_all, File, OpenOptions},
    io::{BufWriter, Write},
};
use toml::Value;

pub fn value(value: &Value, key: &str) -> Result<String, E> {
    Ok(value
        .get(key)
        .ok_or(E::Other(format!(
            "Fail to find \"{key}\" in [package] section of Cargo.toml"
        )))?
        .as_str()
        .ok_or(E::Other(format!(
            "Fail to convert \"{key}\" into string ([package] section of Cargo.toml)"
        )))?
        .to_string())
}

pub fn create() -> Result<(), E> {
    let config = config::get()?;
    let package = config
        .get_cargo()
        .get("package")
        .ok_or(E::Other(String::from(
            "Fail to find [package] in Cargo.toml",
        )))?;
    let name = value(package, "name")?;
    let version = value(package, "version")?;
    let dist = config.node_mod_dist.clone().ok_or(E::Other(String::from(
        "No path to folder with node module. Set correct path in [tslink] of Cargo.toml; field \"node\"",
    )))?;
    let node_module = config
        .node_mod_filename
        .clone()
        .ok_or(E::Other(String::from(
            "No node module file name. Set correct path in [tslink] of Cargo.toml; field \"node\"",
        )))?;
    drop(config);
    let package_file = dist.join("package.json");
    if package_file.exists() {
        fs::remove_file(&package_file)?;
    } else if let Some(basepath) = package_file.parent() {
        if !basepath.exists() {
            create_dir_all(basepath)?;
        }
    } else {
        return Err(E::FileNotFound(format!(
            "Fail to get basepath from: {}",
            package_file.to_string_lossy()
        )));
    }
    File::create(&package_file)?;
    let file = OpenOptions::new().append(true).open(&package_file)?;
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
    \"main\": \"lib.js\",
    \"types\": \"lib.d.ts\"
}}"
        )
        .as_bytes(),
    )?;
    buf_writer.flush()?;
    Ok(())
}
