use std::rc::Rc;

use gpui::{
    App, Div, ElementId, FocusHandle, Pixels, Point, SharedString, Stateful, Window, actions,
    anchored, deferred, div, prelude::*, px, svg,
};

use crate::{assets::Icon, theme::Theme};

actions!(menu, [Dismiss]);

const KEY_CONTEXT: &str = "Menu";
const WIDTH: f32 = 190.0;
const ITEM_HEIGHT: f32 = 28.0;
const WINDOW_MARGIN: f32 = 8.0;

pub fn popover(
    focus_handle: &FocusHandle,
    offset: Point<Pixels>,
    on_dismiss: impl Fn(&mut Window, &mut App) + 'static,
    content: impl IntoElement,
    window: &Window,
) -> impl IntoElement {
    let on_dismiss = Rc::new(on_dismiss);
    let dismiss_from_backdrop = on_dismiss.clone();
    let viewport = window.viewport_size();

    div()
        .absolute()
        .top_0()
        .left_0()
        .child(deferred(
            anchored().position(Point::default()).child(
                div()
                    .id("popover-backdrop")
                    .w(viewport.width)
                    .h(viewport.height)
                    .occlude()
                    .on_any_mouse_down(move |_, window, cx| dismiss_from_backdrop(window, cx)),
            ),
        ))
        .child(
            deferred(
                anchored()
                    .offset(offset)
                    .snap_to_window_with_margin(px(WINDOW_MARGIN))
                    .child(
                        div()
                            .track_focus(focus_handle)
                            .key_context(KEY_CONTEXT)
                            .on_action(move |_: &Dismiss, window, cx| on_dismiss(window, cx))
                            .child(content),
                    ),
            )
            .with_priority(1),
        )
}

pub fn container(theme: &Theme) -> Div {
    div()
        .w(px(WIDTH))
        .flex()
        .flex_col()
        .p_1()
        .rounded_lg()
        .border_1()
        .border_color(theme.border)
        .bg(theme.elevated_background)
        .shadow_lg()
        .text_size(px(13.0))
        .text_color(theme.text)
}

pub fn header(label: impl Into<SharedString>, theme: &Theme) -> Div {
    div()
        .px_2()
        .py_1()
        .text_xs()
        .text_color(theme.text_muted)
        .child(label.into())
}

pub fn item(
    id: impl Into<ElementId>,
    icon: Icon,
    label: impl Into<SharedString>,
    theme: &Theme,
) -> Stateful<Div> {
    let hover = theme.hover;

    div()
        .id(id)
        .h(px(ITEM_HEIGHT))
        .flex()
        .items_center()
        .gap_2()
        .px_2()
        .rounded_md()
        .cursor_pointer()
        .hover(move |style| style.bg(hover))
        .child(
            svg()
                .path(icon.path())
                .size_4()
                .flex_none()
                .text_color(theme.icon),
        )
        .child(label.into())
}
