use core::str::FromStr;
use heapless::String;

pub trait StrExt {
    fn string<const N: usize>(&self) -> String<N>;
}

impl StrExt for str {

    fn string<const N: usize>(&self) -> String<N> {
        string::<N>(self)
    }
}

pub fn string<const N: usize>(value: &str) -> String<N> {
    String::<N>::from_str(value).unwrap()
}