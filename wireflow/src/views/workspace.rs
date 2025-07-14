use eframe::egui::Ui;
use crate::AppState;
use crate::ui_elements::CircuitsScene;
use crate::views::{View, ViewId};

#[derive(Default)]
pub struct Workspace {
    scene: CircuitsScene
}

impl View for Workspace {
    fn show(&mut self, state: &mut AppState, ui: &mut Ui) {
        self.scene.show(state, ui);
    }
}