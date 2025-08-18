use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use eframe::egui::{Color32, Id, Key, Pos2, Rect, Scene, Sense, Stroke, Ui, Vec2};
use circuit_sim::{Circuit, Simulation};
use circuit_sim::component::{describe_pin, Component, ComponentType, PinMode};
use crate::AppState;
use crate::ui_elements::{circuits};

#[derive(Debug)]
pub struct CircuitsScene {
    pub simulation: Arc<Mutex<Simulation>>,
    rect: Rect,
    show_command: bool,
    command_prompt: String,
    component_wiring: ComponentWiring
}

#[derive(Debug, Default)]
struct ComponentWiring {
    pub start: String,
    pub end: String,
}

impl Default for CircuitsScene {
    fn default() -> Self {
        let mut simulation = Arc::new(Mutex::new(Simulation::default()));
        {
            let mut circuit_foo = Circuit::default();
            circuit_foo.add(Component::new("input0", ComponentType::IO));
            circuit_foo.add(Component::new("input1", ComponentType::IO));
            circuit_foo.add(Component::new("input2", ComponentType::IO));

            circuit_foo.add(Component::new("output0", ComponentType::IO));
            circuit_foo.add(Component::new("output1", ComponentType::IO));
            circuit_foo.add(Component::new("output2", ComponentType::IO));

            let mut foo = Component::new("foo", ComponentType::COMPOUND(
                circuit_foo,
                vec![
                    describe_pin("input0", "Input 0", PinMode::Input),
                    describe_pin("input1", "Input 1", PinMode::Input),
                    describe_pin("input2", "Input 2", PinMode::Input),
                    describe_pin("output0", "Output 0", PinMode::Output),
                    describe_pin("output1", "Output 1", PinMode::Output),
                    describe_pin("output2", "Output 2", PinMode::Output),
                ]
            ));

            foo.label = "Foo".to_string();
            let clock = Component::new("clock", ComponentType::CLOCK(
                Instant::now(),
                Duration::from_secs(1)
            ));

            let and = Component::new("and", ComponentType::AND);
            let or = Component::new("or", ComponentType::OR);
            let mut simulation = simulation.lock().unwrap();
            // simulation.circuit.add(clock);
            // simulation.circuit.add(foo);
            // simulation.circuit.add(and);
            // simulation.circuit.add(or);
            // simulation.circuit.add(Component::new("not", ComponentType::NOT));
            // simulation.circuit.add(Component::new("xor", ComponentType::XOR));
            simulation.circuit.add(Component::new("input0", ComponentType::IO));
            simulation.circuit.add(Component::new("output0", ComponentType::IO));
            // simulation.circuit.add(Component::new("nand", ComponentType::NAND));
            // simulation.circuit.add(Component::new("nor", ComponentType::NOR));
            // simulation.circuit.add(Component::new("xnor", ComponentType::XNOR));
        }
        Self {
            simulation,
            rect: Rect::ZERO,
            show_command: false,
            command_prompt: String::new(),
            component_wiring: ComponentWiring::default(),
        }
    }
}

impl CircuitsScene {
    pub fn show(&mut self, state: &mut AppState, ui: &mut Ui) {
        let top = self.rect.center_top();
        let res = Scene::default()
            .show(ui, &mut self.rect, |ui: &mut Ui| {
                let sim_clone = self.simulation.clone();
                let mut simulation = sim_clone.lock().unwrap();

                if ui.input(|i| i.key_released(Key::P) && i.modifiers.command & i.modifiers.shift) {
                    self.show_command = !self.show_command;
                }


                if self.show_command {
                    ui.put(Rect::from_center_size(top, Vec2::new(250.0, 0.0)), |ui: &mut Ui| {
                        let res = ui.text_edit_singleline(&mut self.command_prompt);
                        if !res.has_focus() && !res.lost_focus() {
                            res.request_focus();
                        }

                        if res.lost_focus() {
                            self.show_command = false;
                        }

                        res
                    });

                }

                for node_ind in simulation.circuit.graph.node_indices() {
                    let comp = simulation.circuit.graph
                        .node_weight_mut(node_ind)
                        .unwrap();

                    let component_res = circuits::draw(comp, ui);

                    if component_res.dragged() {
                        comp.x += component_res.drag_delta().x;
                        comp.y += component_res.drag_delta().y;
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
                            if !self.component_wiring.start.is_empty() {
                                self.component_wiring.end = format!("{}.{}", comp.id, pin.id);

                            } else {
                                self.component_wiring.start = format!("{}.{}", comp.id, pin.id);
                            }
                        }
                    }
                }

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

                if !self.component_wiring.start.is_empty() {
                    let (start_comp, start_pin, start_x, start_y) = {
                        let parts = self.component_wiring.start.split('.')
                            .collect::<Vec<_>>();
                        let start_comp = simulation.circuit.get(parts[0])
                            .unwrap();
                        let start_pin = start_comp.get(parts[1])
                            .unwrap();
                        (start_comp.id.clone(), start_pin.id.clone(), start_pin.x, start_pin.y)
                    };


                    if !self.component_wiring.end.is_empty() {
                        let (end_comp, end_pin) = {
                            let parts = self.component_wiring.end.split('.')
                                .collect::<Vec<_>>();
                            let end_comp = simulation.circuit.get(parts[0])
                                .unwrap();
                            let end_pin = end_comp.get(parts[1])
                                .unwrap();

                            (end_comp.id.clone(), end_pin.id.clone())
                        };

                        simulation.circuit.wire(
                            (&start_comp, &start_pin),
                            (&end_comp, &end_pin)
                        );
                        self.component_wiring = ComponentWiring::default();

                    } else {
                        let painter = ui.painter();
                        let start = Pos2::new(start_x, start_y + 0.5);
                        let end = ui.response().hover_pos()
                            .unwrap_or_default();
                        painter.line_segment(
                            [start, end],
                            Stroke::new(circuits::STROKE_SIZE, Color32::WHITE)
                        );
                    }
                }
            });
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