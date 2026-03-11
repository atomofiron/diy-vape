
pub trait OptionOptionExt<T> {
    fn flat(self) -> Option<T>;
}

pub trait OptionExt<T> {
    fn if_some(self, action: impl Fn(&T)) -> Self;
}

impl<T> OptionOptionExt<T> for Option<Option<T>> {

    fn flat(self) -> Option<T> {
        self?
    }
}

impl<T> OptionExt<T> for Option<T> {

    fn if_some(self, action: impl Fn(&T)) -> Self {
        if let Some(v) = &self {
            action(v)
        }
        return self
    }
}