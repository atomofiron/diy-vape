use crate::core::timer::Timer;
use crate::ext::pin_ext::LedExt;
use crate::games::life::universe::{Universe, HEIGHT, WIDTH};
use crate::types::{Display, POPP};
use nrf52840_hal::Rng;
use ssd1306::command::AddrMode;

pub fn alive(
    display: &mut Display,
    timer: &mut Timer,
    rng: &mut Rng,
    with_splashes: bool,
    green: &mut POPP,
) {
    let mut universe = Universe::new();

    let mut time = timer.now();
    display.set_addr_mode(AddrMode::Vertical)
        .unwrap();
    display.set_draw_area((0, 0), (WIDTH as u8, HEIGHT as u8)).
        unwrap();
    let mut flag = true;
    loop {
        match flag {
            true => green.on(),
            false => green.off(),
        }
        flag = !flag;
        let splash = with_splashes && (timer.now() >= time + 2044); // ≈2 sec
        if splash {
            time = timer.now();
        }
        universe.evolve(display, rng, splash);
    }
}
