use eframe::egui::Ui;
use crate::AppState;

mod workspace;
pub use workspace::*;

pub trait View: Default {
    fn show(&mut self, state: &mut AppState, ui: &mut Ui);
}

#[derive(Debug, Default)]
pub enum ViewId {
    #[default]
    Workspace,
}