use std::borrow::Cow;

pub(super) struct Selected<'a, Message> {
    pub(super) selected_rows: Cow<'a, [bool]>,
    pub(super) on_selected: Box<dyn Fn(Vec<bool>) -> Message + 'a>,
}
