use gpui::{AnyView, App, Context, Subscription, Window, WindowAppearance, div, prelude::*, rgb};

pub fn create(window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| Home {
        _appearance_subscription: cx.observe_window_appearance(window, |_, window, _| {
            window.refresh();
        }),
    })
    .into()
}

struct Home {
    _appearance_subscription: Subscription,
}

impl Render for Home {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (background, foreground) = match window.appearance() {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => (0x202020, 0xf2f0eb),
            WindowAppearance::Light | WindowAppearance::VibrantLight => (0xf8f7f4, 0x262626),
        };

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(background))
            .text_color(rgb(foreground))
            .text_2xl()
            .child("Mano")
    }
}
