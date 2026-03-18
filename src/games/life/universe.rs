use crate::ext::result_ext::ResultExt;
use crate::types::Display;
use crate::values::{SCREEN_HEIGHT, SCREEN_WIDTH};
use nrf52840_hal::Rng;

pub const WIDTH: usize = SCREEN_WIDTH as usize;
pub const HEIGHT: usize = SCREEN_HEIGHT as usize;
const BUF_SIZE: usize = WIDTH;
const SEEDS_25_COUNT: usize = 8;

pub struct Universe {
    generation: [u64; BUF_SIZE],
    splash: bool,
    pub last_splash: u64,
}

impl Universe {

    pub const fn new() -> Universe {
        Universe {
            generation: [0; BUF_SIZE],
            splash: false,
            last_splash: 0,
        }
    }

    #[allow(unused)]
    pub fn sow_abundantly(&mut self, rng: &mut Rng) {
        // 50%
        let mut atom = rng.random_u64();
        let iron = rng.random_u64();
        // 25%
        for i in (0..BUF_SIZE).step_by(32) {
            self.generation[i] = atom & iron;
            self.generation[i + 1] = !atom & iron;
            self.generation[i + 2] = atom & !iron;
            self.generation[i + 3] = !atom & !iron;
            atom = atom.rotate_left(2);
        }
    }

    pub fn evolve(
        &mut self,
        display: &mut Display,
        rng: &mut Rng,
        splash: bool,
    ) {
        self.splash = true;
        self.sow(rng);
        self.evolution(splash);
        let bytes: &[u8] = bytemuck::cast_slice(&self.generation);
        display.draw(bytes)
            .ignore();
    }

    fn sow(&mut self, rng: &mut Rng) {
        // 50%
        let atom = rng.random_u64();
        let iron = rng.random_u64();
        let spin = rng.random_u64();
        let muon = rng.random_u64();
        // 25%
        let seeds_25: [u64; SEEDS_25_COUNT] = [
            atom & iron, !atom & iron, atom & !iron, !atom & !iron,
            spin & muon, !spin & muon, spin & !muon, !spin & !muon,
        ];
        // 1.5625%
        let chunk_size = BUF_SIZE / SEEDS_25_COUNT;
        for s_idx in 0..SEEDS_25_COUNT {
            let source = seeds_25[s_idx];
            let offset = s_idx * chunk_size;
            for i in 0..(chunk_size as u32) {
                let s1 = source.rotate_left((i * 7 + 13) % 64);
                let s2 = source.rotate_right((i * 11 + 29) % 64);
                let s3 = source.rotate_left((i * 19 + 43) % 64);
                let s4 = (source ^ s1).rotate_right(31);
                self.generation[offset + i as usize] |= source & s1 & s2 & s3 & s4;
            }
        }
    }

    fn evolution(&mut self, splash: bool) {
        let first = self.generation[0];
        let mut left;
        let mut center = self.generation[BUF_SIZE - 1];
        let mut right = first;
        let mut neighbors = [0u64; 8];
        neighbors[3] = center.rotate_left(1);
        neighbors[4] = center.rotate_right(1);
        neighbors[5] = right.rotate_left(1);
        neighbors[7] = right.rotate_right(1);
        for cursor in 0..WIDTH {
            left = center;
            center = right;
            right = match cursor + 1 {
                WIDTH => first,
                next => self.generation[next],
            };
            neighbors[0] = neighbors[3];
            neighbors[1] = left;
            neighbors[2] = neighbors[4];
            neighbors[3] = neighbors[5];
            neighbors[4] = neighbors[7];
            neighbors[5] = right.rotate_left(1);
            neighbors[6] = right;
            neighbors[7] = right.rotate_right(1);
            self.generation[cursor] = calculate_column(center, neighbors, splash);
        }
        self.splash = false;
    }

    pub fn armageddon(&mut self) {
        self.generation.fill(0);
    }
}

fn calculate_column(current: u64, neighbors: [u64; 8], splash: bool) -> u64 {
    let mut box_0 = 0u64;
    let mut box_1 = 0u64;
    let mut box_2 = 0u64;
    for n in neighbors {
        let second = box_0 & n;
        let third = box_1 & second;
        box_0 ^= n;
        box_1 ^= second;
        box_2 ^= third;
    }
    return if splash {
        let can_birth = box_0 & !box_2;
        let can_survive = !box_2;
        (!current & can_birth) | (current & can_survive)
    } else {
        let sum_is_3 = box_0 & box_1 & !box_2;
        let sum_is_2 = !box_0 & box_1 & !box_2;
        sum_is_3 | (sum_is_2 & current)
    }
}
