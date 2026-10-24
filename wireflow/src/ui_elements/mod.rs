mod circuits;
mod circuits_scene;
pub mod tabview;

use eframe::egui::Ui;
pub use circuits_scene::CircuitsScene;

pub trait UiElement {
    type State;
    fn show(&mut self, ui: &mut Ui, args: Self::State) -> Self::State;
}