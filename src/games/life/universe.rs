use nrf52840_hal::Rng;
use crate::values::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub const WIDTH: usize = SCREEN_WIDTH as usize;
pub const HEIGHT: usize = SCREEN_HEIGHT as usize;
const BUF_SIZE: usize = WIDTH * HEIGHT / 8;

pub struct Universe {
    pub curr_gen: [u8; BUF_SIZE],
    pub next_gen: [u8; BUF_SIZE],
    splash: bool,
}

impl Universe {

    pub const fn new() -> Universe {
        Universe {
            curr_gen: [0; BUF_SIZE],
            next_gen: [0; BUF_SIZE],
            splash: false,
        }
    }

    pub fn sow(&mut self, rng: &mut Rng) {
        for i in (0..BUF_SIZE).step_by(16) {
            let atom = rng.random_u64();
            let iron = rng.random_u64();

            let spin = (atom & iron).to_ne_bytes();
            self.curr_gen[i..i+8].copy_from_slice(&spin);

            let muon = (!atom & !iron).to_ne_bytes();
            self.curr_gen[i+8..i+16].copy_from_slice(&muon);
        }
    }

    pub fn evolution(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let is_alive = self.is_alive(x, y);
                let count = self.count_neighbors(x, y);
                match is_alive {
                    false if self.splash && count == 1 || count == 3 => self.set_cell(x, y, true), // birth
                    true if !self.splash && count < 2 || count > 3 => self.set_cell(x, y, false), // death
                    _ => self.set_cell(x, y, is_alive), // keep
                }
            }
        }
        self.curr_gen = self.next_gen;
        self.splash = false;
    }

    pub fn splash(&mut self) {
        self.splash = true;
    }

    #[inline]
    fn is_alive(&self, x: usize, y: usize) -> bool {
        let page = y >> 3;
        let bit_offset = y & 7;
        let byte_idx = (page << 7) + x;

        let byte = self.curr_gen[byte_idx];
        let mask = 1 << bit_offset;
        return (byte & (mask)) != 0
    }

    fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        let page = y >> 3;
        let bit_offset = y & 7;
        let byte_idx = (page << 7) + x;

        return match alive {
            true => self.next_gen[byte_idx] |= 1 << bit_offset,
            false => self.next_gen[byte_idx] &= !(1 << bit_offset)
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        let x = x as i32;
        let y = y as i32;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx != 0 || dy != 0 {
                    let x = cycle(x + dx, WIDTH);
                    let y = cycle(y + dy, HEIGHT);
                    if self.is_alive(x, y) {
                        count += 1;
                    }
                }
            }
        }
        return count
    }
}

#[inline]
fn cycle(value: i32, limit: usize) -> usize {
    let r = match value {
        _ if value < 0 => value + limit as i32,
        _ if value >= limit as i32 => value & (limit as i32 - 1),
        _ => value,
    };
    return r as usize
}
