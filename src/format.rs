use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Input formats supported.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Format {
    Sexp,
}

impl Default for Format {
    fn default() -> Self { Self::Sexp }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct FormatFromStrError(String);
impl std::error::Error for FormatFromStrError {}
impl Display for FormatFromStrError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "`{}` is not a valid format", self.0)
    }
}

impl FromStr for Format {
    type Err = FormatFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "sexp" => Ok(Self::Sexp),
            _ => Err(FormatFromStrError(s.to_owned())),
        }
    }
}
