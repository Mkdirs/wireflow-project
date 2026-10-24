mod region;
mod selection;
pub mod nodes_panel;
pub mod node_picker;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Sub;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use eframe::egui::{Align, Align2, CentralPanel, Color32, DragPanButtons, Frame, Id, Key, Layout, PointerButton, Popup, PopupAnchor, Pos2, Rect, RectAlign, Response, Scene, Stroke, Ui, Vec2, Widget};
use eframe::epaint::{CubicBezierShape};
use wireflowapi::{Circuit, PlayState};
use wireflowapi::component::Component;
use crate::AppState;
use crate::ui_elements::{circuits, UiElement};
use crate::ui_elements::circuits::{ComponentPinEvent, PIN_RADIUS};
use crate::ui_elements::circuits_scene::node_picker::NodePicker;
use crate::ui_elements::circuits_scene::nodes_panel::NodePanel;
use crate::ui_elements::circuits_scene::selection::Selection;

pub struct CircuitsScene {
    circuit: Arc<Mutex<Circuit>>,
    component_wiring: ComponentWiringProcess,
    selection: Selection,
    node_picker: NodePicker,
    last_mouse_pos: Pos2,
    components_info: HashMap<String, ComponentInfos>,
    component_counter: u32,
}

struct ComponentWiringProcess {
    start: String,
    end: String,
    just_finished: bool,
    delete: Option<(String, String)>,
    on_delete: Rc<dyn Fn(&mut Circuit, &str, &str)>
}

#[derive(Debug)]
struct ComponentInfos {
    rect: Rect,
    position: Pos2,
    pins_position: HashMap<String, Pos2>
}

impl Default for CircuitsScene {
    fn default() -> Self {
        Self {
            circuit: Arc::new(Mutex::new(Circuit::default())),
            component_wiring: ComponentWiringProcess{
                start: String::new(),
                end: String::new(),
                just_finished: false,
                delete: None,
                on_delete: Rc::new(|circuit, comp, pin| {
                    circuit.unwire_all((comp, pin));
                }),
            },
            selection: Selection::default(),
            last_mouse_pos: Pos2::ZERO,
            node_picker: NodePicker::default()
                .id("node_picker_popup"),
            components_info: HashMap::default(),
            component_counter: 0,
        }
    }
}

impl UiElement for CircuitsScene {
    type State = (AppState, Rect);

    fn show(&mut self, ui: &mut Ui, args: Self::State) -> Self::State {
        let (mut state, mut rect) = args;
        
        NodePanel::default()
            .id("scene_left_panel")
            .title("Nodes")
            .show(ui, self);

        CentralPanel::default()
            .show_inside(ui, |ui| {
                let scene_res = self.render_scene(ui, &mut rect);

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
                    self.node_picker.toggle(ui.ctx().screen_rect().center());
                }

                if let Some(entry) = self.node_picker.show(ui) {
                    let mut circuit = self.circuit.lock().unwrap();
                    let node_id = format!("comp#{}", self.component_counter);

                    let mut component = Component::new(&node_id, entry.name.clone(), (entry.factory)());
                    component.set_pins(HashMap::from_iter(
                        entry.pins.iter()
                            .map(|pin| (pin.name.clone(), pin.clone()))
                    ));

                    let pos = self.last_mouse_pos;
                    self.components_info.insert(
                        node_id.clone(),
                        ComponentInfos {
                            rect: Rect::ZERO,
                            position: pos,
                            pins_position: HashMap::default()
                        }
                    );

                    circuit.add(&node_id, component);
                    self.component_counter += 1;
                };

                Popup::new(Id::new("scene_float_bottom"), ui.ctx().clone(), PopupAnchor::from(scene_res.rect), ui.layer_id())
                    .align(RectAlign::from_align2(Align2::CENTER_BOTTOM))
                    .layout(Layout::left_to_right(Align::TOP))
                    .show(|ui| {
                        ui.button("Play");
                        ui.button("Zozo");
                    });
            });

        if self.is_running() {
            ui.ctx().request_repaint();
        }
        
        return (state, rect);
    }
}

impl CircuitsScene {
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

                        self.selection.show(ui, self.last_mouse_pos, &mut |selection, delta| {
                            for (_, infos) in self.components_info
                                .iter_mut()
                                .filter(|(id, _)| selection.contains(*id)) {

                                infos.position += delta;
                            }
                        });

                        self.handle_wiring(ui);
                        self.selection.handle(
                            ui,
                            self.last_mouse_pos,
                            self.components_info
                                .iter()
                                .map(|(id, info)| (id.clone(), info.rect))
                        );

                        if ui.input(|i| i.key_pressed(Key::D) || i.key_pressed(Key::Delete))
                            && !self.node_picker.open() && self.selection.is_complete() {
                            self.remove_selected_components();
                        }
                    })
            }).response
    }

    fn remove_selected_components(&mut self) {
        for (id, _) in &self.selection.region.elements {
            self.components_info.remove(id);
            self.circuit.lock().unwrap().remove(id);
        }

        self.selection.clear();
    }

    fn handle_wiring(&mut self, ui: &mut Ui) {
        let circuit_clone = self.circuit.clone();
        let mut circuit = circuit_clone.lock().unwrap();

        if let Some((comp, pin)) = &self.component_wiring.delete {
            (self.component_wiring.on_delete)(&mut circuit, &comp, &pin);
            self.component_wiring.reset();
            return;
        }

        if self.component_wiring.beginning() {
            if ui.response().is_pointer_button_down_on() {
                self.component_wiring.reset();
                return;
            }
            let (start_comp_id, start_pin_id) = self.component_wiring.start().unwrap();
            let pins_position = &self.components_info.get(&start_comp_id)
                .unwrap()
                .pins_position;
            let start = pins_position.get(&start_pin_id).unwrap();
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
    }

    fn render_components(&mut self, ui: &mut Ui) {
        let circuit_clone = self.circuit.clone();
        let circuit = circuit_clone.lock().unwrap();

        for comp in circuit.graph.node_weights() {
            let component_infos = self.components_info.get_mut(&comp.id).unwrap();
            let (pin_response, response) = circuits::draw(
                ui,
                comp,
                component_infos.position,
                true
            );

            for (pin_name, pin_pos) in pin_response.pins_position {
                component_infos.pins_position.insert(pin_name, pin_pos);
            }


            component_infos.rect = response.rect;

            if response.dragged() {
                let drag_delta = response.drag_delta();
                if self.selection.is_single() {
                    self.selection.region.translate(drag_delta);
                } else {
                    self.selection.clear();
                }
                component_infos.position += drag_delta;
                component_infos.rect = component_infos.rect.translate(drag_delta);
            }

            if let Some(mouse_pos) = response.hover_pos() {
                self.last_mouse_pos = mouse_pos;
            }

            if response.clicked_by(PointerButton::Primary) {
                if ui.input(|i| i.modifiers.shift) {
                    self.selection.select(comp.id.clone(), component_infos.rect);
                } else {
                    self.selection.select_single(comp.id.clone(), component_infos.rect);
                }
            }

            if let Some(interaction) = pin_response.interaction {
                let pin = comp.get(&interaction.pin_name).unwrap();
                let pos = component_infos.pins_position.get(&interaction.pin_name).unwrap();

                match interaction.event {
                    ComponentPinEvent::HOVERED => {
                        let color = if ui.input(|i| i.modifiers.shift) &&
                            circuit.is_pin_wired(&comp.id, &interaction.pin_name) &&
                            !self.component_wiring.beginning()
                        {
                            Color32::RED
                        } else {
                            Color32::GOLD
                        };
                        let painter = ui.painter();
                        painter.circle_filled(
                            *pos,
                            PIN_RADIUS,
                            color
                        );
                    },

                    ComponentPinEvent::CLICKED if ui.input(|i| i.modifiers.shift) && circuit.is_pin_wired(&comp.id, &interaction.pin_name) => {
                        if !self.component_wiring.beginning() {
                            self.component_wiring.delete = Some((comp.id.clone(), interaction.pin_name));
                        }
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

            let source_comp_infos = self.components_info.get(&source_comp.id)
                .unwrap();
            let target_comp_infos = self.components_info.get(&target_comp.id)
                .unwrap();

            let source_pos = source_comp_infos.pins_position.get(&source_pin.name).unwrap();
            let target_pos = target_comp_infos.pins_position.get(&target_pin.name).unwrap();

            let color = if source_pin.value.iter().fold(false, |acc, e| acc || *e) {
                Color32::GREEN
            } else {
                Color32::WHITE
            };

            let delta = *target_pos - (*source_pos);
            let offset = 0.3;
            let amplitude = (10.0 + delta.y) - (1.0 / (delta.x + 0.1));
            let control1 = (*source_pos + (delta * offset)) + Vec2::Y * amplitude;
            let control2 = (*target_pos - (delta * offset)) - Vec2::Y * amplitude;
            let bezier = CubicBezierShape::from_points_stroke(
                [*source_pos, control1, control2, *target_pos],
                false,
                Color32::TRANSPARENT,
                Stroke::new(circuits::STROKE_SIZE, color)
            );

            painter.add(bezier);
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
        });
    }

    pub fn stop(&self) {
        self.circuit.lock().unwrap().stop();
    }
}

impl ComponentWiringProcess {
    pub fn beginning(&self) -> bool {
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
        self.start.clear();
        self.end.clear();
        self.just_finished = false;
        self.delete = None;
    }

    pub fn target(&mut self, component_id: &str, pin_id: &str) {
        let id = format!("{}.{}", component_id, pin_id);
        if !self.beginning() {
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