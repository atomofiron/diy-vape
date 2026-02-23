
pub trait OptionExt<T> {
    fn flat(self) -> Option<T>;
}

impl<T> OptionExt<T> for Option<Option<T>> {

    fn flat(self) -> Option<T> {
        self?
    }
}