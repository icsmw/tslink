use crate::{
    interpreter::ts::{Export, Import},
    TS_IMPORTS,
};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufWriter, Error, ErrorKind, Write},
    path::PathBuf,
};

/// Responsible for generating a single `.ts` file, including buffered content,
/// `import` statements, and registration of cross-file `export`s.
///
/// Used during the TypeScript code generation phase to assemble declarations,
/// resolve imports, and write output in a deterministic and append-safe manner.
///
/// # Fields
/// - `buf_writer`: Buffered writer for efficient file output.
/// - `file_name`: Full path to the `.ts` file being written.
/// - `location`: Parent directory of the output file (used for export indexing).
/// - `buffer`: Temporary in-memory buffer for main body (declarations, etc.).
/// - `imports`: Set of all imports that should be emitted at the top of the file.
pub struct Writer {
    buf_writer: BufWriter<File>,
    file_name: PathBuf,
    location: PathBuf,
    buffer: String,
    imports: HashSet<Import>,
}

impl Writer {
    /// Creates a new `Writer` for the given file.
    ///
    /// # Arguments
    /// - `file`: A writable handle to the output `.ts` file.
    /// - `file_name`: Full path to the file (used for resolving parent location).
    ///
    /// # Errors
    /// Returns an error if the parent directory cannot be determined.
    pub fn new(file: File, file_name: PathBuf) -> Result<Self, Error> {
        let location = file_name
            .parent()
            .ok_or(Error::new(
                ErrorKind::NotFound,
                format!("Fail to find parent of: {}", file_name.display()),
            ))?
            .to_path_buf();
        Ok(Self {
            buf_writer: BufWriter::new(file),
            file_name,
            location,
            buffer: String::new(),
            imports: HashSet::new(),
        })
    }

    /// Finalizes file generation by writing all collected `import` statements
    /// and the buffered code content to disk.
    ///
    /// Ensures the file begins with imports, followed by the body.
    ///
    /// # Errors
    /// Returns an error if writing or flushing fails.
    pub fn write_all(&mut self) -> Result<(), Error> {
        for import in self.imports.iter() {
            self.buf_writer
                .write_all(format!("{import}\n").as_bytes())?;
        }
        self.buf_writer.write_all(self.buffer.as_bytes())?;
        self.buf_writer.flush()?;
        Ok(())
    }

    /// Appends the given string to the internal buffer.
    ///
    /// This buffer accumulates TypeScript declarations or code snippets before being flushed.
    ///
    /// # Arguments
    /// - `data`: The string to append.
    pub fn push<S: AsRef<str>>(&mut self, data: S) {
        self.buffer.push_str(data.as_ref());
    }

    /// Registers a new `import` to be written at the top of the file if it's not already present.
    ///
    /// # Arguments
    /// - `name`: The imported entity name.
    /// - `module`: The source module (relative path, no extension).
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed.
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

    /// Declares that the current file *exports* a given symbol so it can be
    /// re-exported via `index.ts` in its directory.
    ///
    /// Delegates this information to the global `TS_IMPORTS` registry (protected by a `RwLock`)
    /// for cross-file export tracking.
    ///
    /// # Arguments
    /// - `name`: The name of the exported symbol.
    /// - `module`: The module name (typically the file stem).
    ///
    /// # Errors
    /// Returns an error if the export registry cannot be accessed.
    pub fn add_export<N: AsRef<str>, M: AsRef<str>>(
        &self,
        name: N,
        module: M,
    ) -> Result<(), Error> {
        let entity = Export {
            entity: name.as_ref().to_owned(),
            module: module.as_ref().to_owned(),
        };
        TS_IMPORTS
            .write()
            .map_err(|_| Error::other("Fail access to a list of imports"))?
            .add(&self.location, entity);
        Ok(())
    }
}
