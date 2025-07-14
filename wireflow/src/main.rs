
mod views;
mod ui_elements;

use eframe::{run_native, App, CreationContext, Frame, NativeOptions};
use eframe::egui::{CentralPanel, Context, IconData, TopBottomPanel, Widget};
use crate::views::{View, ViewId, Workspace};

#[derive(Debug, Default)]
struct AppState {
    current_view: ViewId,
}
#[derive()]
struct Wireflow {
    state: AppState,
    workspace: Workspace,
}
impl Wireflow {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self {
            state: AppState::default(),
            workspace: Workspace::default(),
        }
    }
}
impl App for Wireflow {
    
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            // let running = self.sim_scene.is_running();
            // 
            // if running {
            //     if ui.button("Stop").clicked() {
            //         self.sim_scene.stop();
            //     }
            //     ctx.request_repaint();
            // }else if ui.button("Start").clicked() {
            //     self.sim_scene.run();
            // }
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