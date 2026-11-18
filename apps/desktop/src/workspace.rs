use gpui::{AnyView, Context, Entity, FocusHandle, Subscription, Window, div, prelude::*};

use crate::{app::CloseWindow, panels::PanelFactory, sidebar::Sidebar, theme::Theme};

pub struct Workspace {
    sidebar: Entity<Sidebar>,
    panel: AnyView,
    focus_handle: FocusHandle,
    _appearance_subscription: Subscription,
}

impl Workspace {
    pub fn new(factory: PanelFactory, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(|_| Sidebar::new());
        let panel = factory(window, cx);
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);
        Self {
            sidebar,
            panel,
            focus_handle,
            _appearance_subscription: cx.observe_window_appearance(window, |_, window, _| {
                window.refresh();
            }),
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(window);

        div()
            .size_full()
            .flex()
            .bg(theme.background)
            .text_color(theme.text)
            .track_focus(&self.focus_handle)
            .on_action(|_: &CloseWindow, window, _| window.remove_window())
            .child(self.sidebar.clone())
            .child(div().flex_1().min_w_0().h_full().child(self.panel.clone()))
    }
}
