use std::path::Path;

/// "Downloadable" trait.
pub trait Download {
    type Error;

    /// Start downloading the file.
    fn download(&self, out: &Path) -> Result<(), Self::Error>;

    /// Get total amount uploaded.
    fn get_uploaded(&self) -> usize;

    /// Get total amount downloaded.
    fn get_downloaded(&self) -> usize;

    /// Get total amount left.
    fn get_left(&self) -> usize;
}
