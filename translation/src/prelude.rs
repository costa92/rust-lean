pub use crate::error::Error;

// Alias for a Result to the crate Reuslt
pub type Result<T> = core::result::Result<T, Error>;


pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

// Alias for a Result to the crate Reuslt  with BoxError
pub type BoxResult<T> = core::result::Result<T, BoxError>;

// Generic Wrapper tuple struct for newtype pattern,
// mostly for external type to type From/TryFrom conversion
pub struct W<T>(pub T);

// Personal preference
pub use std::format as f;
