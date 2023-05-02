use iced_core::Background;
use iced_style::container;
use iced_style::table::StyleSheet;

use crate::widget::Table;

#[derive(Debug, Copy, Clone)]
enum RowType {
    Header(Option<bool>),
    Content(Option<bool>),
}

impl RowType {
    fn new(has_header: bool, is_striped: bool) -> Self
    {
        let striped = is_striped.then_some(false);
        if has_header {
            Self::Header(striped)
        } else {
            Self::Content(striped)
        }
    }

    fn next(&mut self) -> Self {
        let current = *self;
        match self {
            RowType::Header(striped) => *self = RowType::Content(*striped),
            RowType::Content(striped) => *self = RowType::Content(striped.map(|s| !s)),
        };
        current
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) struct RowBackground {
    normal: Background,
    header: Background,
    striped: Background,
    selected: Background,

    current_type: RowType,
}

impl RowBackground {
    pub(super) fn new<M, R>(table: &Table<'_, M, R>, theme: &R::Theme) -> Self
        where R: crate::Renderer,
              R::Theme: StyleSheet + container::StyleSheet,
    {
        Self {
            normal: theme.active(&table.style).background,
            header: theme.header_background(&table.style),
            striped: theme.striped_background(&table.style),
            selected: theme.selected_background(&table.style),
            current_type: RowType::new(table.header.is_some(), table.is_striped),
        }
    }

    pub(super) fn next(&mut self) -> Background {
        match self.current_type.next() {
            RowType::Header(_) => self.header,
            RowType::Content(striped) => {
                match striped {
                    None | Some(false) => self.normal,
                    Some(true) => self.striped,
                }
            }
        }
    }
}
