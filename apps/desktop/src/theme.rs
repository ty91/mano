use gpui::{Hsla, Window, WindowAppearance, rgb, rgba};

pub struct Theme {
    pub background: Hsla,
    pub sidebar_background: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub icon: Hsla,
    pub hover: Hsla,
}

impl Theme {
    pub fn of(window: &Window) -> Self {
        match window.appearance() {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self {
                background: rgb(0x191919).into(),
                sidebar_background: rgb(0x202020).into(),
                text: rgb(0xe3e2df).into(),
                text_muted: rgb(0x7d7c79).into(),
                icon: rgb(0x9b9a97).into(),
                hover: rgba(0xffffff0f).into(),
            },
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self {
                background: rgb(0xffffff).into(),
                sidebar_background: rgb(0xf7f7f5).into(),
                text: rgb(0x2f2e2b).into(),
                text_muted: rgb(0x9b9a97).into(),
                icon: rgb(0x73726e).into(),
                hover: rgba(0x0000000a).into(),
            },
        }
    }
}
