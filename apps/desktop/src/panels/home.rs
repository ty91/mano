use gpui::{AnyView, App, Context, Window, div, prelude::*};

pub fn create(_window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|_| Home).into()
}

struct Home;

impl Render for Home {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_2xl()
            .child("Mano")
    }
}
