use nrf52840_hal::Rng;
use crate::core::timer::Timer;
use crate::games::life::universe::{Universe, HEIGHT, WIDTH};
use crate::types::Display;

static mut UNIVERSE: Universe = Universe::new();

pub fn alive(
    display: &mut Display,
    timer: &mut Timer,
    rng: &mut Rng,
    with_splashes: bool,
) {
    let universe_ptr = &raw mut UNIVERSE;
    let universe = unsafe { &mut *universe_ptr };

    let mut time = timer.now();
    universe.sow(rng);
    loop {
        if with_splashes {
            if timer.now() >= time + 2044 { // ~2 sec
                time = timer.now();
                universe.splash()
            }
        }
        display.set_draw_area((0, 0), (WIDTH as u8, HEIGHT as u8)).unwrap();
        display.draw(&universe.curr_gen).unwrap();
        universe.evolution();
    }
}
