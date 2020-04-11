use std::borrow::Cow;

pub mod runtime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZenError {
    description: Cow<'static, str>,
}

impl ZenError {
    pub fn new<S>(description: S) -> Self
        where
            S: Into<Cow<'static, str>>,
    {
        ZenError {
            description: description.into(),
        }
    }
}

impl std::error::Error for ZenError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for ZenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Zen error: ")?;
        f.write_str(&self.description)
    }
}

impl From<std::io::Error> for ZenError {
    fn from(error: std::io::Error) -> Self {
        ZenError::new(format!(
            "IO error occurred! {}",
            error.to_string()
        ))
    }
}
