use gpui::{
    App, AppContext, Application, Bounds, KeyBinding, Menu, MenuItem, TitlebarOptions,
    WindowBounds, WindowOptions, actions, px, size,
};

use crate::{panels, workspace::Workspace};

actions!(mano, [Quit, CloseWindow]);

pub fn run() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-w", CloseWindow, None),
        ]);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.set_menus(vec![Menu {
            name: "Mano".into(),
            items: vec![MenuItem::action("Quit Mano", Quit)],
        }]);
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
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Workspace::new(panels::DEFAULT_PANEL, window, cx)),
        )
        .expect("failed to open the Mano window");
        cx.activate(true);
    });
}
