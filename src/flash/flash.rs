use crate::values::STORAGE_RANGE;
use core::slice::from_raw_parts_mut;
use embedded_storage::nor_flash as sync_nf;
use embedded_storage::nor_flash::{ErrorType, ReadNorFlash};
use embedded_storage_async::nor_flash as async_nf;
use nrf52840_hal::nvmc::{Nvmc, NvmcError};
use nrf52840_hal::pac::NVMC;
use sequential_storage::cache::NoCache;
use sequential_storage::map::{MapConfig, MapStorage};
use sync_nf::NorFlash;

unsafe extern "C" {
    static mut FLASH_STORAGE_START: u8;
}

type NrfNvmc = Nvmc<NVMC>;
pub type Storage = MapStorage<u8, AsyncFlash, NoCache>;

pub struct AsyncFlash(pub NrfNvmc);

impl AsyncFlash {

    pub fn from(nvmc: NVMC) -> AsyncFlash {
        let scratch: &'static mut [u8] = unsafe {
            from_raw_parts_mut(&raw mut FLASH_STORAGE_START, STORAGE_RANGE.len())
        };
        let nvmc = Nvmc::new(nvmc, scratch);
        return AsyncFlash(nvmc)
    }

    pub fn storage(self) -> Storage {
        let config = MapConfig::new(STORAGE_RANGE);
        return MapStorage::new(self, config, NoCache::new())
    }
}

impl ErrorType for AsyncFlash {
    type Error = NvmcError;
}

impl async_nf::NorFlash for AsyncFlash  {

    const WRITE_SIZE: usize = NrfNvmc::WRITE_SIZE;
    const ERASE_SIZE: usize = NrfNvmc::ERASE_SIZE;

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.0.erase(from, to)
    }

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.write(offset, bytes)
    }
}

impl async_nf::ReadNorFlash for AsyncFlash {

    const READ_SIZE: usize = NrfNvmc::READ_SIZE;

    async fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.0.read(offset, bytes)
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }
}
