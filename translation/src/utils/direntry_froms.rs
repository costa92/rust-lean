use crate::prelude::*;
use std::fs::DirEntry;

impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;
    fn try_from(val: W<&DirEntry>) -> Result<String> {
        val.0
            .path()
            .to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::Generic(f!("Invalied path {:?}", val.0)))
    }
}
