use std::path::Path;

pub trait FromFile {
    type Error;

    fn from_file(path: &Path) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
