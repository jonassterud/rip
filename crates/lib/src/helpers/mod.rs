use crate::error::Error;

/// Tries to transform a slice of `N` elements into some value `T`.
pub fn try_from_slice<T, const N: usize>(
    slice: &[u8],
    start: usize,
    f: &dyn Fn([u8; N]) -> T,
) -> Result<T, Error> {
    Ok(f(slice
        .get(start..start + N)
        .ok_or_else(|| Error::Helper(format!("out of range")))?
        .try_into()
        .map_err(|_| Error::Helper(format!("slice too short")))?))
}
