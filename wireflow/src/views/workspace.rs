use std::sync::LazyLock;
use eframe::egui::{Key, KeyboardShortcut, Modifiers, Rect, Ui};
use crate::actions::{ActionHandler, Actions};
use crate::AppState;
use crate::ui_elements::{CircuitsScene, UiElement};
use crate::ui_elements::tabview::{TabView};
use crate::views::View;

pub static WORKSPACE_ACTIONS: LazyLock<Actions<WorkspaceAction>> = LazyLock::new(||
    Actions::default()
        .register(WorkspaceAction::AddComponent, KeyboardShortcut::new(
            Modifiers::COMMAND,
            Key::N
        ))
        .register(WorkspaceAction::DeleteComponent, KeyboardShortcut::new(
            Modifiers::NONE,
            Key::Delete
        ))
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkspaceAction {
    AddComponent,
    DeleteComponent
}

pub struct Workspace {
    rect: Rect,
    tabview: TabView<CircuitsScene>,
}

impl Default for Workspace {
    fn default() -> Self {
        let mut tabview = TabView::<CircuitsScene>::default();
        tabview.id = String::from("workspace");

        Workspace {
            tabview,
            rect: Rect::ZERO,
        }
    }
}

impl View for Workspace {
    fn show(&mut self, state: &mut AppState, ui: &mut Ui) {
        (*state, self.rect) = self.tabview.show(ui, (state.clone(), self.rect));

        if let Some(action) = ui.input_mut(|input| WORKSPACE_ACTIONS.listen(input))
        && let Some(tab) = self.tabview.get_current_tab_mut(){
            tab.state.handle_action(*action, ui);
        }
    }
}