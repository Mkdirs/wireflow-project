use eframe::egui::{Rect, Ui};
use crate::AppState;
use crate::ui_elements::{CircuitsScene, UiElement};
use crate::ui_elements::tabview::{Tab, TabView};
use crate::views::View;

pub struct Workspace {
    // scene: CircuitsScene,
    rect: Rect,
    tabview: TabView<CircuitsScene>
}

impl Default for Workspace {
    fn default() -> Self {
        let mut tabview = TabView::<CircuitsScene>::default();
        tabview.id = String::from("workspace");
        // tabview.add("Main", CircuitsScene::default());
        Workspace {
            tabview,
            rect: Rect::ZERO
        }
    }
}

impl View for Workspace {
    fn show(&mut self, state: &mut AppState, ui: &mut Ui) {
        (*state, self.rect) = self.tabview.show(ui, (state.clone(), self.rect));
    }
}