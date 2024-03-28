use thiserror::Error;

/// Indicates that an illegal coordinate has been created
#[derive(Error, Debug)]
#[error("illegal coordinate")]
pub struct IllegalCoordinateError;
