/// Generic Result type with [Error] as its error variant
pub type Result<T> = std::result::Result<T, Error>;

/// Error Type for nucleid
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An Error caused by a failed IOCTL
    #[error("Ioctl Failure")]
    Ioctl(#[from] nix::Error),

    /// An Error caused by I/O with the Device
    #[error("Couldn't access the DRM device")]
    Io(#[from] std::io::Error),

    /// Something was empty when it wasn't supposed to
    #[error("Empty Data")]
    Empty,

    /// An integer was out of its valid range
    #[error("Out of Range Value")]
    IntegerOutOfRange(#[from] std::num::TryFromIntError),

    /// An error occured while converting a String
    #[error("UTF-8 Convertion error")]
    StringConversion(#[from] std::str::Utf8Error),
}
