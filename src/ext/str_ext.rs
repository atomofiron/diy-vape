use crate::ext::result_ext::ResultExt;
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
    String::<N>::try_from(value).unwrap_or_else(|_| {
        let mut string = String::<N>::new();
        for _ in 0..N {
            string.push('?').ignore();
        }
        string
    })
}