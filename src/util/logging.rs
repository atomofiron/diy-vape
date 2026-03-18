use crate::types::Rslt;
use core::convert::Infallible;
use nrf52840_hal::nvmc::NvmcError;
use sequential_storage::Error as SequentialError;

pub trait SoftUnwrap<R> {
    fn soft_unwrap(self) -> Option<R>;
    fn soft_unwrap_or(self, default: R) -> R;
}

impl<R> SoftUnwrap<R> for Result<R, SequentialError<NvmcError>> {

    fn soft_unwrap(self, /*logger: Logger*/) -> Option<R> {
        match self {
            Ok(r) => Some(r),
            Err(e) => match e {
                SequentialError::Storage { value } => match value {
                    NvmcError::Unaligned |
                    NvmcError::OutOfBounds => None,
                },
                SequentialError::FullStorage |
                SequentialError::Corrupted { .. } |
                SequentialError::LogicBug { .. } |
                SequentialError::BufferTooBig |
                SequentialError::BufferTooSmall(_) |
                SequentialError::SerializationError(_) |
                SequentialError::ItemTooBig |
                _ => None,
            }
        }
    }

    fn soft_unwrap_or(self, default: R) -> R {
        self.soft_unwrap().unwrap_or(default)
    }
}

impl<R> SoftUnwrap<R> for Result<R, Infallible> {

    fn soft_unwrap(self, /*logger: Logger*/) -> Option<R> {
        match self {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }

    fn soft_unwrap_or(self, default: R) -> R {
        self.soft_unwrap().unwrap_or(default)
    }
}

impl<R> SoftUnwrap<R> for Rslt<R> {

    fn soft_unwrap(self, /*logger: Logger*/) -> Option<R> {
        match self {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }

    fn soft_unwrap_or(self, default: R) -> R {
        self.soft_unwrap().unwrap_or(default)
    }
}
