use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use eframe::egui::{Key, Pos2, Rect, Scene, Ui, Vec2};
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
            simulation.circuit.add(clock);
            simulation.circuit.add(foo);
            simulation.circuit.add(and);
            simulation.circuit.add(or);
            simulation.circuit.add(Component::new("not", ComponentType::NOT));
            simulation.circuit.add(Component::new("xor", ComponentType::XOR));
            simulation.circuit.add(Component::new("input0", ComponentType::IO));
            simulation.circuit.add(Component::new("output0", ComponentType::IO));
            simulation.circuit.add(Component::new("nand", ComponentType::NAND));
            simulation.circuit.add(Component::new("nor", ComponentType::NOR));
            simulation.circuit.add(Component::new("xnor", ComponentType::XNOR));
        }
        Self {
            simulation,
            rect: Rect::ZERO,
            show_command: false,
            command_prompt: String::new(),
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
                    let comp = simulation.circuit.graph.node_weight_mut(node_ind).unwrap();
                    let res = circuits::draw(comp, ui);

                    if res.dragged() {
                        let delta = res.drag_delta();
                        comp.x += delta.x;
                        comp.y += delta.y;
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