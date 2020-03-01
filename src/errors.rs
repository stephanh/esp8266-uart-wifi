/// A collection of all errors that can occur.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Could not read from serial port.
    SerialRead,
    /// Could not write to serial port.
    SerialWrite,
    /// Invalid command or command parameters.
    CommandError,
    /// The command failed.
    CommandFailed,
    /// Command read back failure
    CommandReadFail,
}

/// A `Result<T, Error>`.
pub type EResult<T> = Result<T, Error>;
