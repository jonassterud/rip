use crate::prelude::*;
use std::pin::Pin;
use std::{future::Future, path::Path};

/// "Downloadable" trait.
pub trait Download {
    type Error;

    /// Get download future.
    fn initiate<'a>(
        &'a self,
        agent: &'a Agent,
        out: &'a Path,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>>>>;

    /// Get total amount uploaded.
    fn get_uploaded(&self) -> usize;

    /// Get total amount downloaded.
    fn get_downloaded(&self) -> usize;

    /// Get total amount left.
    fn get_left(&self) -> usize;
}
