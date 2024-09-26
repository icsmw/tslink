use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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
