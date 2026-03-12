use core::convert::Infallible;
use nrf52840_hal::nvmc::NvmcError;
use sequential_storage::Error;

pub trait SoftUnwrap<R> {
    fn soft_unwrap(self) -> Option<R>;
}

impl<R> SoftUnwrap<R> for Result<R, Error<NvmcError>> {

    fn soft_unwrap(self, /*logger: Logger*/) -> Option<R> {
        match self {
            Ok(r) => Some(r),
            Err(e) => match e {
                Error::Storage { value } => match value {
                    NvmcError::Unaligned |
                    NvmcError::OutOfBounds => None,
                },
                Error::FullStorage |
                Error::Corrupted { .. } |
                Error::LogicBug { .. } |
                Error::BufferTooBig |
                Error::BufferTooSmall(_) |
                Error::SerializationError(_) |
                Error::ItemTooBig |
                _ => None,
            }
        }
    }
}

impl<R> SoftUnwrap<R> for Result<R, Infallible> {

    fn soft_unwrap(self, /*logger: Logger*/) -> Option<R> {
        match self {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }
}
