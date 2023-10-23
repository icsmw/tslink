pub mod detached;
pub mod enums;
pub mod structs;

use detached::Detached;
use std::collections::HashMap;

pub type Entities = HashMap<String, Entity>;

#[derive(Debug, Clone)]
pub enum Entity {
    Struct(structs::Structs),
    Enum(enums::Enums),
    Detached(Detached),
}
