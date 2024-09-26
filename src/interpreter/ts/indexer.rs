use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{Error, Write},
    path::{Path, PathBuf},
};

use crate::interpreter::ts::Export;

#[derive(Default)]
pub struct Indexer {
    pub exports: HashSet<(PathBuf, Export)>,
    pub inited: bool,
}

impl Indexer {
    pub fn add<P: AsRef<Path>>(&mut self, dest: P, export: Export) {
        self.exports.insert((dest.as_ref().to_owned(), export));
    }
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
