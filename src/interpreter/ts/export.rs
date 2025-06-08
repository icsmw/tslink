use std::fmt::Display;

/// Represents a TypeScript export statement for a specific entity from a module.
///
/// This structure is used to track and render TypeScript export declarations like:
/// `export { MyClass } from "./my_module";`
///
/// # Fields
/// - `entity`: The name of the exported item (e.g., a class, function, or type).
/// - `module`: The name of the module file (without extension) from which the item is exported.
///
/// # Usage
/// This struct is typically collected during code generation to emit final export blocks.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Export {
    pub entity: String,
    pub module: String,
}

/// Implements string rendering for a TypeScript export line.
///
/// Produces a line in the following format:
/// `export { EntityName } from "./module_name";`
impl Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "export {{ {} }} from \"./{}\";",
            self.entity, self.module
        )
    }
}
