mod region;
mod selection;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread;
use eframe::egui::{Align, Align2, Button, CentralPanel, Color32, DragPanButtons, Frame, Grid, Id, Key, LayerId, Layout, Modifiers, PointerButton, Popup, PopupAnchor, Pos2, Rect, RectAlign, Response, Scene, ScrollArea, Sense, SidePanel, Stroke, StrokeKind, Style, TextEdit, TopBottomPanel, Ui, Vec2};
use circuit_sim::{Circuit, PlayState};
use circuit_sim::component::{Component, ComponentType, PinMode};
use crate::AppState;
use crate::ui_elements::{circuits};
use crate::ui_elements::circuits::ComponentPinEvent;
use crate::ui_elements::circuits_scene::region::Region;
use crate::ui_elements::circuits_scene::selection::Selection;

#[derive(Debug)]
pub struct CircuitsScene {
    circuit: Arc<Mutex<Circuit>>,
    component_wiring: ComponentWiringProcess,
    selection: Selection,
    components_prompt: ComponentsPrompt,
    last_mouse_pos: Pos2,
    components_position: HashMap<String, Pos2>,
    pins_position: HashMap<String, Pos2>,
}

#[derive(Debug, Default)]
struct ComponentWiringProcess {
    start: String,
    end: String,
    just_finished: bool
}

// #[derive(Debug, Default)]
// struct Selection {
//     start: Option<Pos2>,
//     end: Pos2,
//     is_complete: bool,
//     elements: HashMap<String, Rect>
// }

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
            circuit: Arc::new(Mutex::new(Circuit::default())),
            component_wiring: ComponentWiringProcess::default(),
            selection: Selection::default(),
            last_mouse_pos: Pos2::ZERO,
            components_prompt: ComponentsPrompt::default(),
            components_position: HashMap::new(),
            pins_position: HashMap::new(),
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

                        let components = state.plugins
                            .iter()
                            .map(|plugin| &plugin.components)
                            .flat_map(|comps| comps.iter())
                            .filter(|(name, _)| name.contains(&search))
                            .collect::<Vec<_>>();

                        if !components.is_empty() {
                            for (name, comp_type) in components {
                                if ui.add(
                                    Button::new(format!("{name}"))
                                        .fill(Color32::TRANSPARENT)
                                ).clicked() {
                                    {
                                        let mut circuit = self.circuit.lock().unwrap();
                                        let node_id = format!("comp#{}", circuit.graph.node_count());
                                        let comp = Component::new(&node_id, name.clone(), comp_type.clone());
                                        let pos = self.last_mouse_pos;
                                        self.components_position.insert(node_id.clone(), pos);

                                        circuit.add(&node_id, comp);

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
                    .drag_pan_buttons(DragPanButtons::MIDDLE)
                    .show(ui, rect, |ui: &mut Ui| {
                        if let Some(mouse_pos) = ui.response().hover_pos() {
                            self.last_mouse_pos = mouse_pos;
                        }

                        self.render_components(ui);
                        self.render_connexions(ui);
                        if !self.selection.is_empty() {
                            self.selection.region.render(ui);
                        }

                        let circuit_clone = self.circuit.clone();
                        let mut circuit = circuit_clone.lock().unwrap();

                        if self.component_wiring.begining() {
                            if ui.response().clicked() {
                                self.component_wiring.reset();
                                return;
                            }
                            let (start_comp_id, start_pin_id) = self.component_wiring.start().unwrap();
                            let start = self.pins_position.get(&format!("{start_comp_id}.{start_pin_id}")).unwrap();
                            let painter = ui.painter();

                            painter.line_segment(
                                [*start, self.last_mouse_pos],
                                Stroke::new(circuits::STROKE_SIZE, Color32::WHITE)
                            );

                        } else if self.component_wiring.just_finished() {
                            let (start_comp_id, start_pin_id) = self.component_wiring.start().unwrap();
                            let (end_comp_id, end_pin_id) = self.component_wiring.end().unwrap();

                            circuit.wire(
                                (&start_comp_id, &start_pin_id),
                                (&end_comp_id, &end_pin_id)
                            );

                            self.component_wiring.reset();
                        }

                        if ui.response().drag_started() {
                            self.selection.region = Region::from(self.last_mouse_pos);
                        } else if ui.response().dragged() {
                            self.selection.region.to(self.last_mouse_pos);
                        } else if ui.response().drag_stopped() {
                            self.selection.complete();
                            if !self.selection.is_complete() {
                                self.selection.clear();
                            }
                        }

                        if self.selection.is_complete() {
                            self.selection.region.fit_to_elements();
                            let rect = self.selection.region.rect();
                            let res = ui.interact(rect, Id::from("SelectionRect"), Sense::click_and_drag());

                            if res.dragged() {
                                self.selection.region.translate(res.drag_delta());
                                for (id, pos) in self.components_position
                                    .iter_mut()
                                    .filter(|(id, _)| self.selection.region.elements.contains_key(*id)) {

                                    *pos = *pos+res.drag_delta();
                                }
                            }

                            if res.clicked_elsewhere() {
                                self.selection.clear();
                            }
                        }

                    })
            }).response
    }
    fn render_components(&mut self, ui: &mut Ui) {
        let circuit_clone = self.circuit.clone();
        let mut circuit = circuit_clone.lock().unwrap();

        for node_ind in circuit.graph.node_indices() {
            let comp = circuit.graph
                .node_weight_mut(node_ind)
                .unwrap();

            let component_pos = self.components_position.get_mut(&comp.id)
                .unwrap();
            let (pin_response, response) = circuits::draw(comp, *component_pos, ui);

            for (pin_name, pin_pos) in pin_response.pins_position {
                self.pins_position.insert(format!("{}.{pin_name}", comp.id), pin_pos);
            }

            if response.dragged() {
                self.selection.clear();
                let drag_delta = response.drag_delta();
                *component_pos = *component_pos+drag_delta;
            }

            if let Some(mouse_pos) = response.hover_pos() {
                self.last_mouse_pos = mouse_pos;
            }

            if response.clicked_by(PointerButton::Primary) {
                self.selection.select_single(comp.id.clone(), response.rect);
            }

            if self.selection.intersects(response.rect) {
                self.selection.add(comp.id.clone(), response.rect);
            } else if self.selection.contains(&comp.id) {
                self.selection.remove(&comp.id);
            }


            if let Some(interaction) = pin_response.interaction {
                let pin = comp.get(&interaction.pin_name).unwrap();
                let pos = self.pins_position.get(&format!("{}.{}", comp.id, interaction.pin_name)).unwrap();


                match interaction.event {
                    ComponentPinEvent::HOVERED => {
                        let painter = ui.painter();
                        painter.circle_filled(
                            *pos,
                            4.0,
                            Color32::GOLD
                        );
                    },
                    ComponentPinEvent::CLICKED if !self.component_wiring.just_finished() => {
                        self.component_wiring.target(&comp.id, &pin.name);
                    },

                    _ => {}
                }
            }

        }
    }

    fn render_connexions(&mut self, ui: &mut Ui) {
        let circuit_clone = self.circuit.clone();
        let circuit = circuit_clone.lock().unwrap();

        for edge_ind in circuit.graph.edge_indices() {
            let painter = ui.painter();
            let (source_pin, target_pin) = circuit.graph.edge_weight(edge_ind).unwrap();
            let (source, target) = circuit.graph.edge_endpoints(edge_ind).unwrap();

            let (source_comp, target_comp) = (circuit.graph.node_weight(source).unwrap(), circuit.graph.node_weight(target).unwrap());
            let (source_pin, target_pin) = (source_comp.get(source_pin).unwrap(), target_comp.get(target_pin).unwrap());

            let source_pos = self.pins_position.get(&format!("{}.{}", source_comp.id, source_pin.name)).unwrap();
            let target_pos = self.pins_position.get(&format!("{}.{}", target_comp.id, target_pin.name)).unwrap();

            let color = if source_pin.value.iter().fold(false, |acc, e| acc || *e) {
                Color32::GREEN
            } else {
                Color32::WHITE
            };
            painter.line_segment(
                [
                    Pos2::new(source_pos.x, source_pos.y + 0.5),
                    Pos2::new(target_pos.x, target_pos.y + 0.5)
                ],
                Stroke::new(circuits::STROKE_SIZE, color)
            );
        }
    }

    pub fn is_running(&self) -> bool {
        self.circuit.lock().unwrap().play_state() == &PlayState::Running
    }

    pub fn run(&self) {
        let circuit_clone = self.circuit.clone();
        let _ = thread::spawn(move || {
            let mut circuit_ref = circuit_clone.lock().unwrap();
            if circuit_ref.simulation.play_state != PlayState::Stopped {
                return;
            }

            circuit_ref.simulation.play_state = PlayState::Running;
            drop(circuit_ref);

            loop {
                {
                    let mut circuit_ref = circuit_clone.lock().unwrap();
                    match circuit_ref.simulation.play_state {
                        PlayState::Stopped => break,
                        PlayState::Paused => continue,
                        _ => circuit_ref.step()
                    }
                }
            }
            // Circuit::run(circuit);
            // circuit.lock().unwrap().run();
            // Simulation::run(simulation, || {});
        });
    }

    pub fn stop(&self) {
        self.circuit.lock().unwrap().stop();
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
//
// impl Selection {
//     pub fn region(&self) -> Option<Rect> {
//         if self.is_complete {
//             self.compact_region()
//         } else {
//             self.start.and_then(|start| Some(Rect::from_points(&[start, self.end])))
//         }
//     }
//
//     fn compact_region(&self) -> Option<Rect> {
//         let points = self.elements.values()
//             .map(|rect| [rect.min, rect.max])
//             .flatten()
//             .collect::<Vec<_>>();
//
//         Some(Rect::from_points(&points))
//     }
//
//     pub fn start(&mut self, pos: Pos2) {
//         self.start = Some(pos);
//         self.end = pos;
//         self.is_complete = false;
//     }
//
//     pub fn complete(&mut self) {
//         if self.elements.is_empty() {
//             self.start = None;
//             return;
//         }
//
//         self.is_complete = true;
//     }
//
//     pub fn translate(&mut self, delta: Vec2) {
//         let start = self.start.unwrap();
//         self.start = Some(start + delta);
//         self.end += delta;
//     }
//
// }