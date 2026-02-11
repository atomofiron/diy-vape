SECTIONS {
  .bootloader_settings (NOLOAD) : {
    KEEP(*(.bootloader_settings*))
  } > RAM
}