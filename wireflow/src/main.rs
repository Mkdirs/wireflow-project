
mod views;
mod ui_elements;
mod registry;

use std::time::{Duration, Instant};
use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use eframe::egui::{CentralPanel, Context, IconData, TopBottomPanel, Widget};
use circuit_sim::component::{Component, ComponentType};
use crate::registry::{Registries, Registry};
use crate::views::{View, ViewId, Workspace};

#[derive(Debug, Default)]
struct AppState {
    current_view: ViewId,
    registries: Registries,
    workspace_name: String
}
#[derive(Default)]
struct Wireflow {
    state: AppState,
    workspace: Workspace,
}
impl Wireflow {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let mut wireflow = Wireflow::default();
        wireflow.state.registries.components_registry = Registry::from([
            (String::from("wireflow:and"), ComponentType::AND),
            (String::from("wireflow:or"), ComponentType::OR),
            (String::from("wireflow:xor"), ComponentType::XOR),
            (String::from("wireflow:not"), ComponentType::NOT),
            (String::from("wireflow:nand"), ComponentType::NAND),
            (String::from("wireflow:nor"), ComponentType::NOR),
            (String::from("wireflow:xnor"), ComponentType::XNOR),
            (String::from("wireflow:input"), ComponentType::IO),
            (String::from("wireflow:output"), ComponentType::IO),
            (String::from("wireflow:clock"), ComponentType::CLOCK(Instant::now(), Duration::from_secs(1))),
        ]);

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