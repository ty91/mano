use std::collections::HashSet;

use gpui::{AnyElement, Context, FontWeight, Window, div, prelude::*, px, svg};

use crate::{assets::Icon, theme::Theme};

const WIDTH: f32 = 220.0;
const TITLEBAR_HEIGHT: f32 = 34.0;
const ROW_HEIGHT: f32 = 28.0;
const INDENT: f32 = 18.0;

struct Entry {
    id: &'static str,
    label: &'static str,
    icon: Icon,
    children: &'static [Entry],
}

struct Section {
    title: Option<&'static str>,
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
        entries: &[
            leaf("workspace", "Mano", Icon::Notebook),
            leaf("settings", "설정", Icon::Settings),
            leaf("trash", "휴지통", Icon::Trash),
            leaf("search", "검색", Icon::Search),
        ],
    },
    Section {
        title: Some("데일리 노트"),
        entries: &[leaf("today", "오늘", Icon::Calendar)],
    },
    Section {
        title: Some("브라우저"),
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

pub struct Sidebar {
    selected: &'static str,
    expanded: HashSet<&'static str>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            selected: "tasks",
            expanded: HashSet::from(["tour"]),
        }
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
                rows.push(
                    div()
                        .flex_none()
                        .mt_5()
                        .mb_1()
                        .px_1p5()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(title)
                        .into_any_element(),
                );
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
