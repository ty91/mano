use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

macro_rules! icons {
    ($($variant:ident => $file:literal),* $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum Icon {
            $($variant),*
        }

        impl Icon {
            pub fn path(self) -> &'static str {
                match self {
                    $(Icon::$variant => concat!("icons/", $file, ".svg")),*
                }
            }
        }

        const FILES: &[(&str, &[u8])] = &[
            $((
                concat!("icons/", $file, ".svg"),
                include_bytes!(concat!("../assets/icons/", $file, ".svg")),
            )),*
        ];
    };
}

icons! {
    BookOpen => "book-open",
    Calendar => "calendar",
    Feather => "feather",
    File => "file",
    Folder => "folder",
    FolderOpen => "folder-open",
    Image => "image",
    LayoutGrid => "layout-grid",
    Notebook => "notebook",
    Plus => "plus",
    Search => "search",
    Settings => "settings",
    Smile => "smile",
    SquareCheck => "square-check-big",
    Trash => "trash-2",
}

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(FILES
            .iter()
            .find(|(file, _)| *file == path)
            .map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(FILES
            .iter()
            .filter(|(file, _)| file.starts_with(path))
            .map(|(file, _)| SharedString::from(*file))
            .collect())
    }
}
