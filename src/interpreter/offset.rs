use std::fmt;

#[derive(Debug, Clone)]
pub struct Offset {
    tab: usize,
}

impl Offset {
    pub fn new() -> Self {
        Self { tab: 0 }
    }

    pub fn inc(&self) -> Self {
        Self { tab: self.tab + 1 }
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", " ".repeat(self.tab * 4))
    }
}
