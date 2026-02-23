pub mod blocking;
pub mod logging;

pub fn round(value: f32) -> i32 {
    (value + 0.5) as i32
}
