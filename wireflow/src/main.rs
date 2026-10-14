
mod views;
mod ui_elements;

use crate::views::{View, ViewId, Workspace};
use eframe::egui::{CentralPanel, Context, IconData, TopBottomPanel};
use eframe::{run_native, App, CreationContext, Frame, NativeOptions};

#[derive(Default)]
struct AppState {
    current_view: ViewId,
    workspace_name: String
}

#[derive(Default)]
struct Wireflow {
    state: AppState,
    workspace: Workspace,
}

impl Wireflow {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let wireflow = Wireflow::default();

        wireflow
    }
}
impl App for Wireflow {
    
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            ui.label("🛠 In progres...");
        });

        CentralPanel::default()
            .show(ctx, |ui| {
                match self.state.current_view {
                    ViewId::Workspace => {
                        self.workspace.show(&mut self.state, ui);
                    }
                }
        });
    }
}

fn main() -> eframe::Result {
    let mut options = NativeOptions::default();
    options.centered = true;
    options.viewport = options.viewport
        .with_icon(IconData::default());
    
    run_native(
        "Wireflow",
        options,
        Box::new(|cc| Ok(Box::new(Wireflow::new(cc))))
    )
}
