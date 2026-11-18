use gpui::{
    App, AppContext, Application, Bounds, Menu, MenuItem, TitlebarOptions, WindowBounds,
    WindowOptions, actions, point, px, size,
};

use crate::{
    assets::Assets,
    keymap, panels,
    workspace::{CloseTab, NewTab, Workspace},
};

actions!(app, [Quit]);

pub fn run() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        let bindings =
            keymap::load(keymap::DEFAULT_BINDINGS, cx).expect("the default keymap is invalid");
        cx.bind_keys(bindings);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.set_menus(vec![
            Menu {
                name: "Mano".into(),
                items: vec![MenuItem::action("Quit Mano", Quit)],
            },
            Menu {
                name: "File".into(),
                items: vec![
                    MenuItem::action("New Tab", NewTab),
                    MenuItem::action("Close Tab", CloseTab),
                ],
            },
        ]);
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let bounds = Bounds::centered(None, size(px(960.0), px(640.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(480.0), px(320.0))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Mano".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(14.0), px(14.0))),
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Workspace::new(panels::DEFAULT_PANEL, window, cx)),
        )
        .expect("failed to open the Mano window");
        cx.activate(true);
    });
}
