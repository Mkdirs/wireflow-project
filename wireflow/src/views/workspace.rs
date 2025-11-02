use eframe::egui::{CentralPanel, Label, Rect, ScrollArea, SidePanel, TextWrapMode, Ui, WidgetText};
use eframe::egui::AtomKind::Text;
use crate::AppState;
use crate::ui_elements::CircuitsScene;
use crate::views::{View, ViewId};

#[derive(Debug)]
pub struct Workspace {
    scene: CircuitsScene,
    rect: Rect
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            scene: CircuitsScene::default(),
            rect: Rect::ZERO
        }
    }
}

impl View for Workspace {
    fn show(&mut self, state: &mut AppState, ui: &mut Ui) {
        self.scene.show(state, &mut self.rect, ui);
    }
}