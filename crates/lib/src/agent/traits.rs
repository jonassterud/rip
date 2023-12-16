use crate::error::Error;
use crate::prelude::*;
use std::path::Path;
use tokio::task::JoinHandle;

/// "Downloadable" trait.
pub trait Download {
    type Error;

    /// Get download future.
    fn initiate<'a>(&'a self, agent: &'a Agent, out: &'a Path) -> JoinHandle<Result<(), Error>>;

    /// Get total amount uploaded.
    fn get_uploaded(&self) -> usize;

    /// Get total amount downloaded.
    fn get_downloaded(&self) -> usize;

    /// Get total amount left.
    fn get_left(&self) -> usize;
}
