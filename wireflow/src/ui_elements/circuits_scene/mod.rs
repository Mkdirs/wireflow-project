use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use eframe::egui::{Align, Align2, Button, CentralPanel, Color32, Frame, Grid, Id, Key, LayerId, Layout, Modifiers, Popup, PopupAnchor, Pos2, Rect, RectAlign, Response, Scene, ScrollArea, Sense, SidePanel, Stroke, Style, TextEdit, TopBottomPanel, Ui, Vec2};
use circuit_sim::{Circuit, Simulation};
use circuit_sim::component::{describe_pin, Component, ComponentDescriptor, ComponentType, PinMode};
use crate::AppState;
use crate::ui_elements::{circuits};

#[derive(Debug)]
pub struct CircuitsScene {
    pub simulation: Arc<Mutex<Simulation>>,
    component_wiring: ComponentWiringProcess,
    components_prompt: ComponentsPrompt,
    last_mouse_pos: Pos2,
}

#[derive(Debug, Default)]
struct ComponentWiringProcess {
    start: String,
    end: String,
    just_finished: bool
}

#[derive(Debug, Default)]
struct ComponentsPrompt {
    search: String,
    show: bool,
    position: Pos2,
    just_requested_focus: bool,
}

impl Default for CircuitsScene {
    fn default() -> Self {
        Self {
            simulation: Arc::new(Mutex::new(Simulation::default())),
            component_wiring: ComponentWiringProcess::default(),
            last_mouse_pos: Pos2::ZERO,
            components_prompt: ComponentsPrompt::default(),
        }
    }
}

impl CircuitsScene {
    pub fn show(&mut self, state: &mut AppState, rect: &mut Rect, ui: &mut Ui) {

        let scene_res = self.render_scene(ui, rect);

        Popup::new(Id::new("scene_float_top"), ui.ctx().clone(), PopupAnchor::from(scene_res.rect), ui.layer_id())
            .align(RectAlign::from_align2(Align2::CENTER_TOP))
            .layout(Layout::left_to_right(Align::TOP))
            .show(|ui| {
                let running = self.is_running();
                let label = if running { "⏹" } else { "▶" };
                if ui.button(label).clicked() {
                    if running {
                        self.stop();
                    } else {
                        self.run();
                    }
                }
            });

        if ui.input(|input| input.key_pressed(Key::N) && input.modifiers.command_only()) {
            self.components_prompt.show = !self.components_prompt.show;
            if self.components_prompt.show {
                self.components_prompt.position = ui.ctx().pointer_hover_pos()
                    .unwrap_or(ui.ctx().screen_rect().center());
            }
        }
        Popup::new(Id::new("scene_components_popup"), ui.ctx().clone(), PopupAnchor::from(self.components_prompt.position), ui.layer_id())
            .align(RectAlign::from_align2(Align2::CENTER_CENTER))
            .width(300.0)
            .frame(Frame::popup(ui.style())
                .fill(Color32::TRANSPARENT)
            )
            .open(self.components_prompt.show)
            .show(|ui| {

                let res = ui.text_edit_singleline(&mut self.components_prompt.search);
                let search = self.components_prompt.search.clone();

                if !self.components_prompt.just_requested_focus {
                    res.request_focus();
                    self.components_prompt.just_requested_focus = true;
                }

                if ui.input(|input| input.key_pressed(Key::Escape)) {
                    self.components_prompt = ComponentsPrompt::default();
                }

                ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.set_min_height(300.0);

                        let components = state.registries.components_registry
                            .iter()
                            .filter(|(id, _)| id.contains(&search))
                            .collect::<Vec<_>>();

                        if !components.is_empty() {
                            for (id, comp_type) in components {
                                if ui.add(
                                    Button::new(format!("{id}"))
                                        .fill(Color32::TRANSPARENT)
                                ).clicked() {
                                    {
                                        let mut sim = self.simulation.lock().unwrap();
                                        let node_id = format!("{}{}", id, sim.circuit.graph.node_count());
                                        let mut comp = Component::new(&node_id, comp_type.clone());
                                        let pos = self.last_mouse_pos;
                                        comp.x = pos.x;
                                        comp.y = pos.y;

                                        sim.circuit.add(comp);

                                        self.components_prompt = ComponentsPrompt::default();
                                    }
                                }
                                ui.separator();
                            }
                        } else {
                            ui.centered_and_justified(|ui| ui.label(":( no components found"));
                        }
                    });
            });

        Popup::new(Id::new("scene_float_bottom"), ui.ctx().clone(), PopupAnchor::from(scene_res.rect), ui.layer_id())
            .align(RectAlign::from_align2(Align2::CENTER_BOTTOM))
            .layout(Layout::left_to_right(Align::TOP))
            .show(|ui| {
                ui.button("Play");
                ui.button("Zozo");
            });

        if self.is_running() {
            ui.ctx().request_repaint();
        }
    }

    fn render_scene(&mut self, ui: &mut Ui, rect: &mut Rect) -> Response {
        Frame::new()
            .show(ui, |ui| {
                Scene::default()
                    .show(ui, rect, |ui: &mut Ui| {
                        if let Some(mouse_pos) = ui.response().hover_pos() {
                            self.last_mouse_pos = mouse_pos;
                        }

                        self.render_components(ui);
                        self.render_connexions(ui);

                        let sim_clone = self.simulation.clone();
                        let mut simulation = sim_clone.lock().unwrap();

                        if self.component_wiring.begining() {
                            if ui.response().clicked() {
                                self.component_wiring.reset();
                                return;
                            }
                            let (start_comp_id, start_pin_id) = self.component_wiring.start().unwrap();
                            let (start_x, start_y) = {
                                let pin = simulation.circuit.get(&start_comp_id)
                                    .unwrap()
                                    .get(&start_pin_id).unwrap();

                                (pin.x, pin.y)
                            };
                            let painter = ui.painter();
                            let start = Pos2::new(start_x, start_y);

                            painter.line_segment(
                                [start, self.last_mouse_pos],
                                Stroke::new(circuits::STROKE_SIZE, Color32::WHITE)
                            );

                        } else if self.component_wiring.just_finished() {
                            let (start_comp_id, start_pin_id) = self.component_wiring.start().unwrap();
                            let (end_comp_id, end_pin_id) = self.component_wiring.end().unwrap();

                            simulation.circuit.wire(
                                (&start_comp_id, &start_pin_id),
                                (&end_comp_id, &end_pin_id)
                            );

                            self.component_wiring.reset();
                        }

                    })
            }).response
    }
    fn render_components(&mut self, ui: &mut Ui) {
        let sim_clone = self.simulation.clone();
        let mut simulation = sim_clone.lock().unwrap();

        for node_ind in simulation.circuit.graph.node_indices() {
            let comp = simulation.circuit.graph
                .node_weight_mut(node_ind)
                .unwrap();

            let component_res = circuits::draw(comp, ui);

            if component_res.dragged() {
                comp.x += component_res.drag_delta().x;
                comp.y += component_res.drag_delta().y;
            }

            if let Some(mouse_pos) = component_res.hover_pos() {
                self.last_mouse_pos = mouse_pos;
            }

            for pin in comp.pins() {
                let pos = Pos2::new(pin.x, pin.y);
                let pin_rect = Rect::from_center_size(pos, Vec2::splat(8.0));
                let pin_response = ui.interact(
                    pin_rect,
                    Id::from(format!("{cmp}.{pin_id}", cmp=comp.id, pin_id=pin.id)),
                    Sense::hover() | Sense::click()
                );

                if pin_response.hovered() {
                    let painter = ui.painter();
                    painter.circle_filled(
                        pos,
                        4.0,
                        Color32::GOLD
                    );
                }

                if pin_response.clicked() {
                    if !self.component_wiring.just_finished() {
                        self.component_wiring.target(&comp.id, &pin.id);
                    }
                }
            }
        }
    }

    fn render_connexions(&mut self, ui: &mut Ui) {
        let sim_clone = self.simulation.clone();
        let mut simulation = sim_clone.lock().unwrap();

        for edge_ind in simulation.circuit.graph.edge_indices() {
            let painter = ui.painter();
            let (source_pin, target_pin) = simulation.circuit.graph.edge_weight(edge_ind).unwrap();
            let (source, target) = simulation.circuit.graph.edge_endpoints(edge_ind).unwrap();

            let (source_comp, target_comp) = (simulation.circuit.graph.node_weight(source).unwrap(), simulation.circuit.graph.node_weight(target).unwrap());
            let (source_pin, target_pin) = (source_comp.get(source_pin).unwrap(), target_comp.get(target_pin).unwrap());


            let color = if source_pin.value {
                Color32::GREEN
            }else {
                Color32::WHITE
            };
            painter.line_segment(
                [
                    Pos2::new(source_pin.x, source_pin.y + 0.5),
                    Pos2::new(target_pin.x, target_pin.y + 0.5)
                ],
                Stroke::new(circuits::STROKE_SIZE, color)
            );
        }
    }

    pub fn is_running(&self) -> bool {
        self.simulation.lock().unwrap().running
    }

    pub fn run(&self) {
        let simulation = self.simulation.clone();
        let _ = thread::spawn(move || {
            Simulation::run(simulation, || {});
        });
    }

    pub fn stop(&self) {
        self.simulation.lock().unwrap().running = false;
    }
}

impl ComponentWiringProcess {
    pub fn begining(&self) -> bool {
        !self.start.is_empty() && !self.just_finished
    }

    pub fn start(&self) -> Option<(String, String)> {
        if self.start.is_empty() {
            return None;
        }

        let parts = self.start.split('.')
            .collect::<Vec<_>>();

        Some((parts[0].to_string(), parts[1].to_string()))
    }

    pub fn end(&self) -> Option<(String, String)> {
        if self.end.is_empty() {
            return None;
        }

        let parts = self.end.split('.')
            .collect::<Vec<_>>();

        Some((parts[0].to_string(), parts[1].to_string()))
    }

    pub fn reset(&mut self) {
        *self = ComponentWiringProcess::default();
    }

    pub fn target(&mut self, component_id: &str, pin_id: &str) {
        let id = format!("{}.{}", component_id, pin_id);
        if !self.begining() {
            self.start = id.clone();
        } else {
            self.end = id.clone();
            self.just_finished = true;
        }
    }

    pub fn just_finished(&self) -> bool {
        self.just_finished
    }
}