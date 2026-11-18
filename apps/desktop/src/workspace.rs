use gpui::{Context, Entity, FocusHandle, Subscription, Window, actions, div, prelude::*, px, svg};

use crate::{
    assets::Icon,
    panels::{Panel, PanelFactory},
    sidebar::Sidebar,
    theme::Theme,
};

actions!(workspace, [NewTab, CloseTab]);

const TAB_BAR_HEIGHT: f32 = 42.0;
const TAB_HEIGHT: f32 = 28.0;
const TAB_MAX_WIDTH: f32 = 200.0;

struct Tab {
    panel: Panel,
    last_active: u64,
}

pub struct Workspace {
    sidebar: Entity<Sidebar>,
    new_panel: PanelFactory,
    tabs: Vec<Tab>,
    active: usize,
    activations: u64,
    focus_handle: FocusHandle,
    _appearance_subscription: Subscription,
}

impl Workspace {
    pub fn new(new_panel: PanelFactory, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);
        let mut workspace = Self {
            sidebar: cx.new(|_| Sidebar::new()),
            new_panel,
            tabs: Vec::new(),
            active: 0,
            activations: 0,
            focus_handle,
            _appearance_subscription: cx.observe_window_appearance(window, |_, window, _| {
                window.refresh();
            }),
        };
        workspace.open_tab(window, cx);
        workspace
    }

    fn open_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let panel = (self.new_panel)(window, cx);
        self.tabs.push(Tab {
            panel,
            last_active: 0,
        });
        self.activate_tab(self.tabs.len() - 1, cx);
    }

    fn activate_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        self.activations += 1;
        self.tabs[index].last_active = self.activations;
        self.active = index;
        cx.notify();
    }

    fn close_active_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.tabs.len() == 1 {
            window.remove_window();
            return;
        }
        self.tabs.remove(self.active);
        let next = self
            .tabs
            .iter()
            .enumerate()
            .max_by_key(|(_, tab)| tab.last_active)
            .map(|(index, _)| index)
            .unwrap_or_default();
        self.activate_tab(next, cx);
    }

    fn render_tab_bar(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let hover = theme.hover;

        div()
            .h(px(TAB_BAR_HEIGHT))
            .flex_none()
            .flex()
            .items_center()
            .gap_1()
            .px_3()
            .text_size(px(13.0))
            .children(self.tabs.iter().enumerate().map(|(index, tab)| {
                div()
                    .id(("tab", tab.panel.view.entity_id().as_u64() as usize))
                    .h(px(TAB_HEIGHT))
                    .max_w(px(TAB_MAX_WIDTH))
                    .min_w_0()
                    .flex()
                    .items_center()
                    .px_2()
                    .rounded_md()
                    .cursor_pointer()
                    .hover(move |style| style.bg(hover))
                    .text_color(if index == self.active {
                        theme.text
                    } else {
                        theme.text_muted
                    })
                    .child(div().truncate().child(tab.panel.title.clone()))
                    .on_click(cx.listener(move |this, _, _, cx| this.activate_tab(index, cx)))
            }))
            .child(
                div()
                    .id("new-tab")
                    .size(px(TAB_HEIGHT))
                    .flex_none()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .cursor_pointer()
                    .hover(move |style| style.bg(hover))
                    .child(
                        svg()
                            .path(Icon::Plus.path())
                            .size_4()
                            .text_color(theme.icon),
                    )
                    .on_click(cx.listener(|this, _, window, cx| this.open_tab(window, cx))),
            )
            .child(
                div()
                    .id("tab-bar-space")
                    .flex_1()
                    .h_full()
                    .on_click(|event, window, _| {
                        if event.click_count() == 2 {
                            window.titlebar_double_click();
                        }
                    }),
            )
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(window);

        div()
            .size_full()
            .flex()
            .bg(theme.background)
            .text_color(theme.text)
            .key_context("Workspace")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &NewTab, window, cx| this.open_tab(window, cx)))
            .on_action(
                cx.listener(|this, _: &CloseTab, window, cx| this.close_active_tab(window, cx)),
            )
            .child(self.sidebar.clone())
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .child(self.render_tab_bar(&theme, cx))
                    .child(
                        div()
                            .flex_1()
                            .min_h_0()
                            .child(self.tabs[self.active].panel.view.clone()),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use gpui::{Modifiers, TestAppContext, VisualTestContext, WindowHandle, point, px};

    use super::*;
    use crate::{keymap, panels};

    fn open(cx: &mut TestAppContext) -> (WindowHandle<Workspace>, VisualTestContext) {
        cx.update(|cx| {
            let bindings = keymap::load(keymap::DEFAULT_BINDINGS, cx).unwrap();
            cx.bind_keys(bindings);
        });
        let window = cx.add_window(|window, cx| Workspace::new(panels::DEFAULT_PANEL, window, cx));
        let cx = VisualTestContext::from_window(window.into(), cx);
        (window, cx)
    }

    fn tab_ids(window: &WindowHandle<Workspace>, cx: &mut VisualTestContext) -> Vec<u64> {
        window
            .read_with(cx, |workspace, _| {
                workspace
                    .tabs
                    .iter()
                    .map(|tab| tab.panel.view.entity_id().as_u64())
                    .collect()
            })
            .unwrap()
    }

    fn active_id(window: &WindowHandle<Workspace>, cx: &mut VisualTestContext) -> u64 {
        window
            .read_with(cx, |workspace, _| {
                workspace.tabs[workspace.active]
                    .panel
                    .view
                    .entity_id()
                    .as_u64()
            })
            .unwrap()
    }

    #[gpui::test]
    fn shortcuts_open_and_close_tabs(cx: &mut TestAppContext) {
        let (window, mut cx) = open(cx);
        assert_eq!(tab_ids(&window, &mut cx).len(), 1);

        cx.simulate_keystrokes("cmd-t cmd-t");
        let ids = tab_ids(&window, &mut cx);
        assert_eq!(ids.len(), 3);
        assert_eq!(active_id(&window, &mut cx), ids[2]);

        cx.simulate_keystrokes("cmd-w");
        assert_eq!(tab_ids(&window, &mut cx), ids[..2]);
        assert_eq!(active_id(&window, &mut cx), ids[1]);
    }

    #[gpui::test]
    fn closing_returns_to_the_most_recently_active_tab(cx: &mut TestAppContext) {
        let (window, mut cx) = open(cx);
        cx.simulate_keystrokes("cmd-t cmd-t");
        let ids = tab_ids(&window, &mut cx);

        window
            .update(&mut cx, |workspace, _, cx| workspace.activate_tab(0, cx))
            .unwrap();
        cx.simulate_keystrokes("cmd-w");

        assert_eq!(tab_ids(&window, &mut cx), ids[1..]);
        assert_eq!(active_id(&window, &mut cx), ids[2]);
    }

    #[gpui::test]
    fn closing_the_last_tab_closes_the_window(cx: &mut TestAppContext) {
        let (_window, mut cx) = open(cx);
        cx.simulate_keystrokes("cmd-t cmd-w cmd-w");
        cx.run_until_parked();
        assert!(cx.windows().is_empty());
    }

    fn new_menu_state(
        window: &WindowHandle<Workspace>,
        cx: &mut VisualTestContext,
    ) -> (bool, bool) {
        window
            .update(cx, |workspace, window, cx| {
                (
                    workspace.sidebar.read(cx).is_new_menu_open(),
                    workspace.focus_handle.is_focused(window),
                )
            })
            .unwrap()
    }

    #[gpui::test]
    fn new_menu_opens_from_the_sidebar_and_returns_focus(cx: &mut TestAppContext) {
        let (window, mut cx) = open(cx);
        let button = cx.debug_bounds("sidebar-new").unwrap().center();
        let closed = (false, true);
        let open = (true, false);

        cx.simulate_click(button, Modifiers::none());
        assert_eq!(new_menu_state(&window, &mut cx), open);
        let menu = cx.debug_bounds("new-menu").unwrap();
        assert_eq!(
            menu.origin.y,
            cx.debug_bounds("sidebar-new").unwrap().bottom() + px(4.0)
        );
        assert!(cx.debug_bounds("new-menu-folder").is_some());
        cx.simulate_keystrokes("escape");
        assert_eq!(new_menu_state(&window, &mut cx), closed);

        cx.simulate_click(button, Modifiers::none());
        let note = cx.debug_bounds("new-menu-note").unwrap().center();
        cx.simulate_click(note, Modifiers::none());
        assert_eq!(new_menu_state(&window, &mut cx), closed);

        cx.simulate_click(button, Modifiers::none());
        cx.simulate_click(point(px(700.0), px(400.0)), Modifiers::none());
        assert_eq!(new_menu_state(&window, &mut cx), closed);

        cx.simulate_click(button, Modifiers::none());
        cx.simulate_click(button, Modifiers::none());
        assert_eq!(new_menu_state(&window, &mut cx), closed);

        cx.simulate_keystrokes("cmd-t");
        assert_eq!(tab_ids(&window, &mut cx).len(), 2);
    }
}
