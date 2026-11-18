use gpui::{App, Context, Window, div, prelude::*};

use crate::panels::Panel;

pub fn create(_window: &mut Window, cx: &mut App) -> Panel {
    Panel {
        title: "새 탭".into(),
        view: cx.new(|_| Blank).into(),
    }
}

struct Blank;

impl Render for Blank {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
