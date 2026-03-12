
pub trait OptionOptionExt<T> {
    fn flat(self) -> Option<T>;
}

pub trait OptionExt<T> {
    fn if_some(self, action: impl Fn(&T)) -> Self;
    fn keep_if(self, predicate: bool) -> Self;
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

    fn keep_if(self, predicate: bool) -> Self {
        match predicate {
            true => self,
            _ => None,
        }
    }
}