use std::{convert::TryFrom, fmt};

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    Ts,
    DTs,
    Js,
}

impl TryFrom<&str> for Target {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == Target::Ts.to_string() {
            Ok(Target::Ts)
        } else if value == Target::DTs.to_string() {
            Ok(Target::DTs)
        } else if value == Target::Js.to_string() {
            Ok(Target::Js)
        } else {
            Err(format!("{value} is invalid target"))
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ts => "ts",
                Self::DTs => "d.ts",
                Self::Js => "js",
            }
        )
    }
}
