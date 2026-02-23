use crate::flash::savable::Savable;
use sequential_storage::map::{SerializationError, Value};

pub struct FlashValue<T>(pub T) where T : Savable;

impl<'a, T> Value<'a> for FlashValue<T> where T: Savable {

    fn serialize_into(&self, buffer: &mut [u8]) -> Result<usize, SerializationError> {
        let used = postcard::to_slice(&self.0, buffer)
            .map_err(|_| SerializationError::BufferTooSmall)?;
        Ok(used.len())
    }

    fn deserialize_from(buffer: &'a [u8]) -> Result<(Self, usize), SerializationError> {
        let (item, remainder) = postcard::take_from_bytes::<T>(buffer)
            .map_err(|_| SerializationError::InvalidFormat)?;

        let consumed = buffer.len() - remainder.len();
        Ok((FlashValue(item), consumed))
    }
}
