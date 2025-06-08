use crate::{config, error::E};
use std::{
    fs,
    fs::{create_dir_all, File, OpenOptions},
    io::{BufWriter, Write},
};
use toml::Value;

/// Extracts a string value from the given [`toml::Value`] under the specified key,
/// returning an error if the key is missing or is not a string.
///
/// This function is typically used to retrieve `name` and `version` from `[package]` in `Cargo.toml`.
///
/// # Parameters
/// - `value`: A reference to a TOML table (e.g., `[package]` section).
/// - `key`: The key to extract (e.g., `"name"` or `"version"`).
///
/// # Returns
/// The value as a `String` if present and valid.
///
/// # Errors
/// Returns an `E::Other` variant if the key is missing or cannot be converted to a string.
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

/// Generates a `package.json` file for the compiled Node.js module using data from `Cargo.toml` and configuration.
///
/// This function:
/// - Reads `name` and `version` from `[package]` section of `Cargo.toml`.
/// - Reads `node_mod_filename` and `node_mod_dist` from `[tslink]` or `[package.metadata.tslink]`.
/// - Creates or replaces `package.json` in the target distribution folder.
/// - Writes out a minimal structure pointing to the generated module, JS, and declaration files.
///
/// # Example Output
/// ```ignore
/// {
///   "name": "your-package",
///   "version": "0.1.0",
///   "files": ["lib.node", "lib.js", "lib.d.ts"],
///   "module": "lib.js",
///   "main": "lib.js",
///   "types": "lib.d.ts"
/// }
/// ```
///
/// # Errors
/// - If any required value is missing from configuration or Cargo metadata.
/// - If the output directory is invalid or file system operations fail.
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
