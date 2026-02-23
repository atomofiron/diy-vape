SECTIONS {
  .bootloader_settings (NOLOAD) : {
    KEEP(*(.bootloader_settings*))
  } > RAM
}
FLASH_STORAGE_START = ORIGIN(FLASH_STORAGE);
FLASH_STORAGE_SIZE = LENGTH(FLASH_STORAGE);