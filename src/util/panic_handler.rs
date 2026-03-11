use core::sync::atomic::{compiler_fence, Ordering};
use nrf52840_hal::pac::Peripherals;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { // space technologies
    cortex_m::interrupt::disable();
    compiler_fence(Ordering::SeqCst);
    unsafe {
        let pac = Peripherals::steal();
        pac.PWM0.enable.write(|w| w.enable().disabled());
        let _ = pac.PWM0.enable.read();
        pac.P0.outclr.write(|w| w.pin2().bit(true));
    }
    compiler_fence(Ordering::SeqCst);
    nrf52840_hal::pac::SCB::sys_reset();
}
