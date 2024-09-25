use std::{
    collections::HashSet,
    fmt::Display,
    fs::{self, File},
    io::{BufWriter, Error, Write},
    path::PathBuf,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Import {
    pub entity: String,
    pub module: String,
}

impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "import {{ {} }} from \"./{}\";",
            self.entity, self.module
        )
    }
}

pub struct Writer {
    buf_writer: BufWriter<File>,
    file_name: PathBuf,
    buffer: String,
    imports: HashSet<Import>,
}

impl Writer {
    pub fn new(file: File, file_name: PathBuf) -> Self {
        Self {
            buf_writer: BufWriter::new(file),
            file_name,
            buffer: String::new(),
            imports: HashSet::new(),
        }
    }
    pub fn write_all(&mut self) -> Result<(), Error> {
        for import in self.imports.iter() {
            self.buf_writer
                .write_all(format!("{import}\n").as_bytes())?;
        }
        self.buf_writer.write_all(self.buffer.as_bytes())?;
        self.buf_writer.flush()?;
        Ok(())
    }
    pub fn push<S: AsRef<str>>(&mut self, data: S) {
        self.buffer.push_str(data.as_ref());
    }
    pub fn add_import<N: AsRef<str>, M: AsRef<str>>(
        &mut self,
        name: N,
        module: M,
    ) -> Result<(), Error> {
        let entity = Import {
            entity: name.as_ref().to_owned(),
            module: module.as_ref().to_owned(),
        };
        let content = fs::read_to_string(&self.file_name)?;
        if !content.contains(&entity.to_string()) {
            self.imports.insert(entity);
        }
        Ok(())
    }
}
