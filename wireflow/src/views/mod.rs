use eframe::egui::Ui;
use crate::AppState;

mod workspace;
pub use workspace::Workspace;

pub trait View: Default {
    fn show(&mut self, app_state: &mut AppState, ui: &mut Ui);
}

pub trait ViewState : Default {}

#[derive(Debug, Default)]
pub enum ViewId {
    #[default]
    Workspace,
}