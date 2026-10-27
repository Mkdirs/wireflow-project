use eframe::egui::Ui;
use crate::AppState;

pub mod workspace;

pub trait View: Default {
    fn show(&mut self, app_state: &mut AppState, ui: &mut Ui);
}

pub trait ViewState : Default {}

#[derive(Debug, Default, Clone)]
pub enum ViewId {
    #[default]
    Workspace,
}