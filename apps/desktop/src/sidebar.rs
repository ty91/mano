use std::collections::HashSet;

use gpui::{
    AnyElement, Context, FocusHandle, FontWeight, SharedString, Window, div, point, prelude::*, px,
    svg,
};

use crate::{assets::Icon, menu, theme::Theme};

const WIDTH: f32 = 220.0;
const TITLEBAR_HEIGHT: f32 = 34.0;
const ROW_HEIGHT: f32 = 28.0;
const INDENT: f32 = 18.0;
const SECTION_HEADER_HEIGHT: f32 = 24.0;
const HEADER_BUTTON_SIZE: f32 = 22.0;
const MENU_GAP: f32 = 4.0;

struct Entry {
    id: &'static str,
    label: &'static str,
    icon: Icon,
    children: &'static [Entry],
}

struct Section {
    title: Option<&'static str>,
    has_new_menu: bool,
    entries: &'static [Entry],
}

const fn leaf(id: &'static str, label: &'static str, icon: Icon) -> Entry {
    Entry {
        id,
        label,
        icon,
        children: &[],
    }
}

const SECTIONS: &[Section] = &[
    Section {
        title: None,
        has_new_menu: false,
        entries: &[
            leaf("workspace", "Mano", Icon::Notebook),
            leaf("settings", "설정", Icon::Settings),
            leaf("trash", "휴지통", Icon::Trash),
            leaf("search", "검색", Icon::Search),
        ],
    },
    Section {
        title: Some("데일리 노트"),
        has_new_menu: false,
        entries: &[leaf("today", "오늘", Icon::Calendar)],
    },
    Section {
        title: Some("브라우저"),
        has_new_menu: true,
        entries: &[
            leaf("welcome", "안녕 Mano!", Icon::Smile),
            leaf("tasks", "Mano 사용해보기", Icon::SquareCheck),
            Entry {
                id: "tour",
                label: "데모 둘러보기",
                icon: Icon::Folder,
                children: &[
                    leaf("journal", "일기", Icon::Feather),
                    leaf("projects", "프로젝트", Icon::LayoutGrid),
                    leaf("photos", "사진", Icon::Image),
                    leaf("reading", "읽을거리", Icon::BookOpen),
                ],
            },
        ],
    },
];

struct NewMenu {
    focus_handle: FocusHandle,
    previous_focus: Option<FocusHandle>,
}

pub struct Sidebar {
    selected: &'static str,
    expanded: HashSet<&'static str>,
    new_menu: Option<NewMenu>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            selected: "tasks",
            expanded: HashSet::from(["tour"]),
            new_menu: None,
        }
    }

    fn open_new_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let focus_handle = cx.focus_handle();
        let previous_focus = window.focused(cx);
        focus_handle.focus(window);
        self.new_menu = Some(NewMenu {
            focus_handle,
            previous_focus,
        });
        cx.notify();
    }

    fn close_new_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(new_menu) = self.new_menu.take() {
            if let Some(previous_focus) = new_menu.previous_focus {
                previous_focus.focus(window);
            }
            cx.notify();
        }
    }

    #[cfg(test)]
    pub fn is_new_menu_open(&self) -> bool {
        self.new_menu.is_some()
    }

    fn render_section_header(
        &self,
        section: &Section,
        title: &'static str,
        theme: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let group = SharedString::from(title);

        div()
            .group(group.clone())
            .h(px(SECTION_HEADER_HEIGHT))
            .flex_none()
            .mt_4()
            .flex()
            .items_center()
            .justify_between()
            .pl_1p5()
            .text_xs()
            .text_color(theme.text_muted)
            .child(title)
            .when(section.has_new_menu, |header| {
                header.child(self.render_new_button(group, theme, window, cx))
            })
            .into_any_element()
    }

    fn render_new_button(
        &self,
        group: SharedString,
        theme: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let hover = theme.hover;
        let sidebar = cx.entity().downgrade();

        div()
            .relative()
            .child(
                div()
                    .id("sidebar-new")
                    .debug_selector(|| "sidebar-new".into())
                    .size(px(HEADER_BUTTON_SIZE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .cursor_pointer()
                    .map(|button| {
                        if self.new_menu.is_some() {
                            button.bg(hover)
                        } else {
                            button
                                .opacity(0.0)
                                .group_hover(group, |style| style.opacity(1.0))
                        }
                    })
                    .hover(move |style| style.bg(hover))
                    .child(
                        svg()
                            .path(Icon::Plus.path())
                            .size_3p5()
                            .text_color(theme.icon),
                    )
                    .on_click(cx.listener(|this, _, window, cx| this.open_new_menu(window, cx))),
            )
            .when_some(self.new_menu.as_ref(), |wrapper, new_menu| {
                wrapper.child(menu::popover(
                    &new_menu.focus_handle,
                    point(px(0.0), px(HEADER_BUTTON_SIZE + MENU_GAP)),
                    move |window, cx| {
                        sidebar
                            .update(cx, |this, cx| this.close_new_menu(window, cx))
                            .ok();
                    },
                    menu::container(theme)
                        .debug_selector(|| "new-menu".into())
                        .child(menu::header("새로 만들기", theme))
                        .child(
                            menu::item("new-note", Icon::File, "노트", theme)
                                .debug_selector(|| "new-menu-note".into())
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.close_new_menu(window, cx)
                                })),
                        )
                        .child(
                            menu::item("new-folder", Icon::Folder, "폴더", theme)
                                .debug_selector(|| "new-menu-folder".into())
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.close_new_menu(window, cx)
                                })),
                        ),
                    window,
                ))
            })
    }

    fn push_rows(
        &self,
        rows: &mut Vec<AnyElement>,
        entry: &'static Entry,
        depth: usize,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) {
        let is_folder = !entry.children.is_empty();
        let expanded = is_folder && self.expanded.contains(entry.id);
        let icon = if expanded {
            Icon::FolderOpen
        } else {
            entry.icon
        };
        let hover = theme.hover;

        rows.push(
            div()
                .id(entry.id)
                .h(px(ROW_HEIGHT))
                .flex()
                .flex_none()
                .items_center()
                .gap_2()
                .pl(px(6.0 + depth as f32 * INDENT))
                .pr_1p5()
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
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .when(self.selected == entry.id, |label| {
                            label.font_weight(FontWeight::SEMIBOLD)
                        })
                        .child(entry.label),
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    if is_folder {
                        if !this.expanded.remove(entry.id) {
                            this.expanded.insert(entry.id);
                        }
                    } else {
                        this.selected = entry.id;
                    }
                    cx.notify();
                }))
                .into_any_element(),
        );

        if expanded {
            for child in entry.children {
                self.push_rows(rows, child, depth + 1, theme, cx);
            }
        }
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(window);
        let mut rows = Vec::new();

        for section in SECTIONS {
            if let Some(title) = section.title {
                rows.push(self.render_section_header(section, title, &theme, window, cx));
            }
            for entry in section.entries {
                self.push_rows(&mut rows, entry, 0, &theme, cx);
            }
        }

        div()
            .w(px(WIDTH))
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .bg(theme.sidebar_background)
            .text_size(px(13.0))
            .child(
                div()
                    .id("sidebar-titlebar")
                    .h(px(TITLEBAR_HEIGHT))
                    .flex_none()
                    .on_click(|event, window, _| {
                        if event.click_count() == 2 {
                            window.titlebar_double_click();
                        }
                    }),
            )
            .child(
                div()
                    .id("sidebar-rows")
                    .flex_1()
                    .flex()
                    .flex_col()
                    .px_2()
                    .pb_3()
                    .overflow_y_scroll()
                    .children(rows),
            )
    }
}
