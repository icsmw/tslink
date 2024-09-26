use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Export {
    pub entity: String,
    pub module: String,
}

impl Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "export {{ {} }} from \"./{}\";",
            self.entity, self.module
        )
    }
}
