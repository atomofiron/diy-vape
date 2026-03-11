use crate::ext::result_ext::ResultExt;
use crate::games::life::universe::{Universe, HEIGHT, WIDTH};
use crate::types::{Display, Time};
use nrf52840_hal::Rng;
use ssd1306::command::AddrMode;

static mut UNIVERSE: Universe = Universe::new();

pub fn draw_life(
    display: &mut Display,
    rng: &mut Rng,
    with_splashes: bool,
    restart: bool,
    now: Time,
) {
    let universe_ptr = &raw mut UNIVERSE;
    let universe = unsafe { &mut *universe_ptr };

    if restart {
        universe.armageddon();
    }

    display.set_addr_mode(AddrMode::Vertical)
        .ignore();
    display.set_draw_area((0, 0), (WIDTH as u8, HEIGHT as u8)).
        ignore();
    let splash = with_splashes && (now >= universe.last_splash + 2044); // ≈2 sec
    if splash {
        universe.last_splash = now;
    }
    universe.evolve(display, rng, splash);
}
