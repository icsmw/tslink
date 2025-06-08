use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{Error, Write},
    path::{Path, PathBuf},
};

use crate::interpreter::ts::Export;

/// Collects and manages export declarations for TypeScript index files (`index.ts`)
/// to ensure correct module linkage when multiple structures are distributed
/// across separate files using `tslink`.
///
/// This is essential for generating valid `import/export` chains when cross-referencing
/// between types defined in different `.ts` files.
///
/// # Fields
/// - `exports`: A set of all `Export` declarations associated with their target file paths.
/// - `inited`: A one-time flag to clean existing `index.ts` files before first write.
#[derive(Default)]
pub struct Indexer {
    pub exports: HashSet<(PathBuf, Export)>,
    pub inited: bool,
}

impl Indexer {
    /// Adds a new export to be tracked for the given target directory.
    ///
    /// # Arguments
    /// - `dest`: The destination path (directory) where the `index.ts` should be written.
    /// - `export`: The export definition (entity and module) to include.
    ///
    /// If the same export is added multiple times for the same destination, it will not be duplicated.
    pub fn add<P: AsRef<Path>>(&mut self, dest: P, export: Export) {
        self.exports.insert((dest.as_ref().to_owned(), export));
    }

    /// Writes or updates `index.ts` files in all tracked destinations with their associated exports.
    ///
    /// For each destination:
    /// - Creates `index.ts` if it does not exist.
    /// - Removes existing file once on first invocation if present (to avoid duplicates).
    /// - Appends new export lines if they are not already included.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The destination folder does not exist.
    /// - File operations (create, read, write) fail.
    pub fn write(&mut self) -> Result<(), Error> {
        for (dest, export) in self.exports.iter() {
            if !dest.exists() {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Index dest folder isn't found: {}", dest.display()),
                ));
            }
            let file_name = dest.join("index.ts");
            if !self.inited && file_name.exists() {
                fs::remove_file(&file_name)?;
                self.inited = true;
            }
            if !file_name.exists() {
                File::create(&file_name)?;
            }
            let content = fs::read_to_string(&file_name)?;
            let export_str = export.to_string();
            if content.contains(&export_str) {
                continue;
            }
            let mut file = OpenOptions::new().append(true).open(&file_name)?;
            file.write_all(format!("{export}\n").as_bytes())?;
        }
        Ok(())
    }
}
