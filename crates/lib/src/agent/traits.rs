use std::path::Path;

pub trait Download {
    type E;

    /// Start downloading the file.
    fn download(&self, out: &Path) -> Result<(), Self::E>;
}
