mod blank;

use gpui::{AnyView, App, SharedString, Window};

pub struct Panel {
    pub title: SharedString,
    pub view: AnyView,
}

pub type PanelFactory = fn(&mut Window, &mut App) -> Panel;

pub const DEFAULT_PANEL: PanelFactory = blank::create;
