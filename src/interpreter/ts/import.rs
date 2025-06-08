use std::fmt::Display;

/// Represents a TypeScript import statement for a specific entity from a module.
///
/// This structure is used to generate TypeScript import declarations like:
/// `import { MyClass } from "./my_module";`
///
/// # Fields
/// - `entity`: The name of the imported item (e.g., a class, function, or type).
/// - `module`: The name of the module file (without extension) from which the item is imported.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Import {
    pub entity: String,
    pub module: String,
}

/// Implements string rendering for a TypeScript import line.
///
/// Produces a line in the following format:
/// `import { EntityName } from "./module_name";`
impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "import {{ {} }} from \"./{}\";",
            self.entity, self.module
        )
    }
}
