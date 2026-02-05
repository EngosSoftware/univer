//! # Definition of result and errors

/// Common result type.
pub type Result<T, E = UniverError> = std::result::Result<T, E>;

/// Error definition.
#[derive(Debug, PartialEq, Eq)]
pub struct UniverError(String);

impl std::fmt::Display for UniverError {
  /// Implementation of [Display](std::fmt::Display) trait for [UniverError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl UniverError {
  /// Creates a new [UniverError] with specified error message.
  pub fn new(message: impl AsRef<str>) -> Self {
    Self(message.as_ref().to_string())
  }
}

macro_rules! univer_error {
  ($($arg:tt)*) => {{
    use crate::errors::UniverError;
    UniverError::new(format!($($arg)*))
  }};
}

pub(crate) use univer_error;
