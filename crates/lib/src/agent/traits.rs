use std::path::Path;

pub trait Download {
    type Error;

    /// Start downloading the file.
    fn download(&self, out: &Path) -> Result<(), Self::Error>;
}
