use crate::types::{FigureStyle, Icon, IconRaw};

#[derive(Clone)]
pub struct TabView {
    pub background: FigureStyle,
    pub icon: IconRaw<'static>,
}

impl TabView {

    pub const fn new(background: FigureStyle, icon: IconRaw<'static>) -> Self {
        Self { background, icon }
    }
}

pub struct TabViewSelector {
    pub normal: TabView,
    pub selected: TabView,
}

impl TabViewSelector {

    pub fn peek(&self, selected: bool) -> TabView {
        match selected {
            true => self.selected.clone(),
            false => self.normal.clone(),
        }
    }
}
