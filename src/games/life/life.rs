use nrf52840_hal::Rng;
use crate::core::timer::Timer;
use crate::games::life::universe::{Universe, HEIGHT, WIDTH};
use crate::types::Display;

static mut UNIVERSE: Universe = Universe::new();

pub fn alive(
    display: &mut Display,
    timer: &mut Timer,
    rng: &mut Rng,
) {
    let universe_ptr = &raw mut UNIVERSE;
    let universe = unsafe { &mut *universe_ptr };

    let mut counter = 0;
    universe.sow(rng);
    loop {
        display.set_draw_area((0, 0), (WIDTH as u8, HEIGHT as u8)).unwrap();
        display.draw(&universe.curr_gen).unwrap();
        universe.evolution();
        timer.sleep_ms(30);
        if counter == 30 {
            counter = 0;
            universe.splash()
        }
        counter += 1;
    }
}
