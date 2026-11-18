use std::rc::Rc;

use gpui::{App, KeyBinding, KeyBindingContextPredicate, Result};

pub struct Binding<'a> {
    pub keystrokes: &'a str,
    pub action: &'a str,
    pub context: Option<&'a str>,
}

pub const DEFAULT_BINDINGS: &[Binding<'static>] = &[
    Binding {
        keystrokes: "cmd-q",
        action: "app::Quit",
        context: None,
    },
    Binding {
        keystrokes: "cmd-t",
        action: "workspace::NewTab",
        context: Some("Workspace"),
    },
    Binding {
        keystrokes: "cmd-w",
        action: "workspace::CloseTab",
        context: Some("Workspace"),
    },
    Binding {
        keystrokes: "escape",
        action: "menu::Dismiss",
        context: Some("Menu"),
    },
];

pub fn load(bindings: &[Binding], cx: &App) -> Result<Vec<KeyBinding>> {
    bindings
        .iter()
        .map(|binding| {
            let action = cx.build_action(binding.action, None)?;
            let context = binding
                .context
                .map(KeyBindingContextPredicate::parse)
                .transpose()?
                .map(Rc::new);
            Ok(KeyBinding::load(
                binding.keystrokes,
                action,
                context,
                false,
                None,
                cx.keyboard_mapper().as_ref(),
            )?)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;

    use super::*;

    #[gpui::test]
    fn default_bindings_resolve_to_registered_actions(cx: &mut TestAppContext) {
        let bindings = cx.update(|cx| load(DEFAULT_BINDINGS, cx)).unwrap();
        assert_eq!(bindings.len(), DEFAULT_BINDINGS.len());
    }

    #[gpui::test]
    fn unknown_actions_are_rejected(cx: &mut TestAppContext) {
        let bindings = [Binding {
            keystrokes: "cmd-k",
            action: "workspace::Missing",
            context: None,
        }];
        assert!(cx.update(|cx| load(&bindings, cx)).is_err());
    }
}
