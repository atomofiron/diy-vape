use crate::flash::flash::Storage;
use crate::flash::flash_value::FlashValue;
use crate::flash::savable::Savable;
use nrf52840_hal::nvmc::NvmcError;
use sequential_storage::Error;

#[allow(async_fn_in_trait)]
pub trait FlashStorage {

    async fn read<T : Savable>(
        &mut self,
        buffer: &mut [u8],
    ) -> Result<Option<T>, Error<NvmcError>>;

    async fn save<T : Savable>(
        &mut self,
        buffer: &mut [u8],
        data: T,
    ) -> Result<(), Error<NvmcError>>;
}

impl FlashStorage for Storage {

    async fn read<T : Savable>(
        &mut self,
        buffer: &mut [u8],
    ) -> Result<Option<T>, Error<NvmcError>> {
        self.fetch_item::<FlashValue<T>>(buffer, &T::FLASH_KEY)
            .await.map_value()
    }

    async fn save<T : Savable>(
        &mut self,
        buffer: &mut [u8],
        data: T,
    ) -> Result<(), Error<NvmcError>> {
        self.store_item(buffer, &T::FLASH_KEY, &FlashValue(data))
            .await
    }
}

trait FlashValueResult<T : Savable> {
    fn map_value(self) -> Result<Option<T>, Error<NvmcError>>;
}

impl<T : Savable> FlashValueResult<T> for Result<Option<FlashValue<T>>, Error<NvmcError>> {
    fn map_value(self) -> Result<Option<T>, Error<NvmcError>> {
        self.map(|it| it.map(|it| it.0))
    }
}
