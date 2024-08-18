use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, enum_utils::FromStr)]
pub(crate) enum Channel {
    Canary,
    Beta,
    RC,
    Release,
    Patch,
}

impl Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Channel::Canary => write!(f, "Canary"),
            Channel::Beta => write!(f, "Beta"),
            Channel::RC => write!(f, "RC"),
            Channel::Release => write!(f, "Release"),
            Channel::Patch => write!(f, "Patch"),
        }
    }
}

impl TryFrom<String> for Channel {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Canary" => Ok(Channel::Canary),
            "Beta" => Ok(Channel::Beta),
            "RC" => Ok(Channel::RC),
            "Release" => Ok(Channel::Release),
            "Patch" => Ok(Channel::Patch),
            _ => Err(format!("Unknown Channel name: {}", value))
        }
    }
}