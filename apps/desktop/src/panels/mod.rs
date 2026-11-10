mod home;

use gpui::{AnyView, App, Window};

pub type PanelFactory = fn(&mut Window, &mut App) -> AnyView;

pub const DEFAULT_PANEL: PanelFactory = home::create;
