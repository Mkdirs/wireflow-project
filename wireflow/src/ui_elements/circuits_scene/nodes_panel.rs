use eframe::egui::{Frame, Id, Label, Response, ScrollArea, Sense, SidePanel, Ui};
use crate::ui_elements::{circuits, CircuitsScene};

#[derive(Debug, Default)]
pub struct NodePanel {
    id: String,
    title: String,
}

impl NodePanel {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn show(&self, ui: &mut Ui, scene: &mut CircuitsScene) -> Response {
        SidePanel::left(self.id.clone())
            .default_width(200.0)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| ui.label(self.title.clone()));
                ui.separator();
                ScrollArea::both()
                    .show(ui, |ui| {
                        let circuit = scene.circuit.lock().unwrap();
                        let mut components = circuit.graph.node_weights()
                            .collect::<Vec<_>>();

                        components.sort_by_key(|comp| &comp.id);

                        for comp in components {
                            let mut frame = Frame::default()
                                .inner_margin(5.0)
                                .begin(ui);

                            let component_infos = scene.components_info.get(&comp.id)
                                .unwrap();

                            {
                                frame.content_ui.horizontal(|ui| {
                                    let pos = ui.next_widget_position();

                                    circuits::draw(
                                        ui,
                                        &comp,
                                        pos,
                                        false
                                    );

                                    ui.add(
                                        Label::new(format!("{} ({})", comp.id, comp.name))
                                            .selectable(false)
                                    );
                                });
                            }

                            let response = frame.content_ui.interact(
                                frame.allocate_space(ui).rect,
                                Id::new(format!("{}:{} ({})", self.id, comp.id, comp.name)),
                                Sense::HOVER | Sense::CLICK
                            );

                            let style = ui.ctx().style().clone();
                            if response.clicked() {
                                if ui.input(|i| i.modifiers.shift) {
                                    scene.selection.select(comp.id.clone(), component_infos.rect);
                                } else {
                                    scene.selection.select_single(comp.id.clone(), component_infos.rect);
                                }
                            }
                            
                            if scene.selection.contains(&comp.id) || response.hovered() {
                                frame.frame.fill = style.interact(&response).bg_fill;
                            }

                            frame.paint(ui);
                        }
                    });
            }).response
    }
}