use gpui::{AnyView, Context, FocusHandle, Window, div, prelude::*};

use crate::{app::CloseWindow, panels::PanelFactory};

pub struct Workspace {
    panel: AnyView,
    focus_handle: FocusHandle,
}

impl Workspace {
    pub fn new(factory: PanelFactory, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let panel = factory(window, cx);
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);
        Self {
            panel,
            focus_handle,
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .track_focus(&self.focus_handle)
            .on_action(|_: &CloseWindow, window, _| window.remove_window())
            .child(self.panel.clone())
    }
}
