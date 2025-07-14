// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::{Duration, Instant};
// use eframe::egui::{Align, Color32, Frame, Id, Layout, Pos2, Rect, Response, RichText, Scene, Sense, Stroke, Ui, UiBuilder, Vec2};
// use circuit_sim::{Circuit, Simulation};
// use circuit_sim::component::{describe_pin, Component, ComponentType, Pin, PinMode};
// 
// mod component_widgets;
// 
// const PINS_RADIUS:f32 = 5.0;
// const PINS_MARGIN:f32 = 10.0;
// const TEXT_MARGIN:f32 = 25.0;
// 
// #[derive(Debug)]
// pub struct SimScene {
//     simulation: Arc<Mutex<Simulation>>,
//     rect: Rect,
//     link_source: String,
//     ling_target: String,
// }
// 
// impl SimScene {
//     pub fn new() -> Self {
//         let mut simulation = Arc::new(Mutex::new(Simulation::default()));
//         {
//             let mut circuit_foo = Circuit::default();
//             circuit_foo.add(Component::new("input0", ComponentType::IO));
//             circuit_foo.add(Component::new("input1", ComponentType::IO));
//             circuit_foo.add(Component::new("input2", ComponentType::IO));
// 
//             circuit_foo.add(Component::new("output0", ComponentType::IO));
//             circuit_foo.add(Component::new("output1", ComponentType::IO));
//             circuit_foo.add(Component::new("output2", ComponentType::IO));
// 
//             let mut foo = Component::new("foo", ComponentType::COMPOUND(
//                 circuit_foo,
//                 vec![
//                     describe_pin("input0", "Input 0", PinMode::Input),
//                     describe_pin("input1", "Input 1", PinMode::Input),
//                     describe_pin("input2", "Input 2", PinMode::Input),
//                     describe_pin("output0", "Output 0", PinMode::Output),
//                     describe_pin("output1", "Output 1", PinMode::Output),
//                     describe_pin("output2", "Output 2", PinMode::Output),
//                 ]
//             ));
// 
//             foo.label = "Foo".to_string();
//             let clock = Component::new("clock", ComponentType::CLOCK(
//                 Instant::now(),
//                 Duration::from_secs(1)
//             ));
//             
//             let and = Component::new("and", ComponentType::AND);
//             let and2 = Component::new("and2", ComponentType::AND);
//             let or = Component::new("or", ComponentType::OR);
//             let mut simulation = simulation.lock().unwrap();
//             //simulation.circuit.add(clock);
//             //simulation.circuit.add(foo);
//             simulation.circuit.add(and);
//             simulation.circuit.add(or);
//             
//             //simulation.circuit.wire(("clock", "output0"), ("foo", "input0"));
//         };
//         Self{
//             simulation,
//             rect: Rect::ZERO,
//             link_source: String::new(),
//             ling_target: String::new(),
//         }
//     }
//     
//     pub fn is_running(&self) -> bool {
//         self.simulation.lock().unwrap().running
//     }
//     
//     pub fn run(&self) {
//         let simulation = self.simulation.clone();
//         let _ = thread::spawn(move || {
//             Simulation::run(simulation, || {});
//         });
//     }
//     
//     pub fn stop(&self) {
//         self.simulation.lock().unwrap().running = false;
//     }
//     
//     fn draw_pin(pin: &mut Pin, ui: &mut Ui) -> Response {
//         const MARGIN:f32 = 1.5;
//         let (response, painter) = ui.allocate_painter(
//             Vec2::new(PINS_RADIUS * 2.0 + MARGIN, PINS_RADIUS * 2.0 + MARGIN),
//             Sense::click()
//         );
// 
//         pin.x = response.rect.center().x;
//         pin.y = response.rect.center().y;
//         
//         let pos = Pos2::new(pin.x, pin.y);
// 
//         if pin.value {
//             painter.circle_filled(pos, PINS_RADIUS, Color32::GREEN);
//         }else{
//             painter.circle_filled(pos, PINS_RADIUS, Color32::GRAY);
//         }
//         
//         response
//     }
//     
//     fn draw_component(&mut self, component: &mut Component, ui: &mut Ui) -> Response {
//         let pos = Pos2::new(component.x, component.y);
//         let comp_id = component.id.clone();
//         ui.put(Rect::from_min_size(pos, Vec2::ZERO), |ui: &mut Ui| {
//             let res = ui.allocate_new_ui(
//                 UiBuilder::new()
//                     .sense(Sense::drag()),
//                 |ui| {
//                     Frame::default()
//                         .fill(Color32::DARK_GRAY)
//                         .corner_radius(5.0)
//                         .inner_margin(10.0)
//                         .show(ui, |ui| {
//                             ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
// 
//                                 if !component.inputs().is_empty() {
//                                     ui.with_layout(Layout::top_down(Align::Center), |ui| {
//                                         let mut inputs = component.inputs_mut();
//                                         inputs.sort_by_key(|input| input.id.clone());
//                                         
//                                         for input in inputs {
//                                             let r = Self::draw_pin(input, ui);
//                                             
//                                             if r.hovered() && !self.link_source.is_empty() {
//                                                 let pos = Pos2::new(input.x, input.y);
//                                                 ui.painter().circle_stroke(pos, PINS_RADIUS*1.5, Stroke::new(1.0, Color32::YELLOW));
//                                             }
//                                             
//                                             if r.clicked() {
//                                                 println!("{} => {}.{}", self.link_source, comp_id, input.id.clone());
//                                             }
//                                         }
//                                     });
//                                     ui.add_space(TEXT_MARGIN);
//                                 }
//                                 ui.label(
//                                     RichText::new(component.label.clone())
//                                         .color(Color32::WHITE)
//                                         .heading()
//                                 );
//                                 if !component.outputs().is_empty() {
//                                     ui.add_space(TEXT_MARGIN);
//                                     ui.with_layout(Layout::top_down(Align::Center), |ui| {
//                                         let mut outputs = component.outputs_mut();
//                                         outputs.sort_by_key(|output| output.id.clone());
//                                         
//                                         for output in component.outputs_mut() {
//                                             let r = Self::draw_pin(output, ui);
// 
//                                             if r.hovered() {
//                                                 let pos = Pos2::new(output.x, output.y);
//                                                 ui.painter().circle_stroke(pos, PINS_RADIUS*1.5, Stroke::new(1.0, Color32::YELLOW));
// 
//                                             }
//                                             
//                                             if r.clicked() {
//                                                 self.link_source = format!("{}.{}", comp_id, output.id.clone());
//                                             }
//                                         }
//                                         
//                                     });
//                                 }
//                             }).inner
//                         }).inner
//                 }
//             );
//             res.response
//         })
//     }
//     
//     
//     pub fn update(&mut self, ui: &mut Ui) {
//         let mut r = self.rect;
//         let res = Scene::default()
//             .show(ui, &mut r, |ui|{
//                 let sim_clone = self.simulation.clone();
//                 let mut simulation = sim_clone.lock().unwrap();
//                 
//                 for node_ind in simulation.circuit.graph.node_indices() {
//                     let comp = simulation.circuit.graph.node_weight_mut(node_ind).unwrap();
//                     
//                     let res = self.draw_comp(comp, ui);//self.draw_component(comp, ui);
// 
//                     if res.dragged() {
//                         println!("Eh");
//                         let delta = res.drag_delta();
//                         comp.x += delta.x;
//                         comp.y += delta.y;
//                     }
//                 }
// 
//                 let painter = ui.painter();
//                 
//                 let res = ui.interact(Rect::EVERYTHING, Id::from("YOEOLEOZM"), Sense::hover());
//                 
//                 // if res.hovered() && !self.link_source.is_empty() {
//                 //     let mut parts = self.link_source.split('.');
//                 //     let (comp_id, pin_id) = (parts.next().unwrap(), parts.next().unwrap());
//                 // 
//                 //     let comp = simulation.circuit.get(comp_id).unwrap();
//                 //     let pin = comp.get(pin_id).unwrap();
//                 //     let pos = Pos2::new(pin.x, pin.y);
//                 //     painter.circle_filled(Pos2::new(0.5, 0.5), 10.0, Color32::WHITE);
//                 //     if let Some(mouse_pos) = res.hover_pos() {
//                 //         painter.line(vec![
//                 //             pos, mouse_pos
//                 //         ], PathStroke::new(10.0, Color32::GREEN));
//                 //     }
//                 // }
//                 // 
//                 // for edge_ind in simulation.circuit.graph.edge_indices() {
//                 //     let (source_pin, target_pin) = simulation.circuit.graph.edge_weight(edge_ind).unwrap();
//                 //     let (source, target) = simulation.circuit.graph.edge_endpoints(edge_ind).unwrap();
//                 //     
//                 //     let (source_comp, target_comp) = (simulation.circuit.graph.node_weight(source).unwrap(), simulation.circuit.graph.node_weight(target).unwrap());
//                 //     let (source_pin, target_pin) = (source_comp.get(source_pin).unwrap(), target_comp.get(target_pin).unwrap());
//                 //     
//                 // 
//                 //     let color = if source_pin.value {
//                 //         Color32::GREEN
//                 //     }else {
//                 //         Color32::GRAY
//                 //     };
//                 //     painter.line_segment([Pos2::new(source_pin.x, source_pin.y), Pos2::new(target_pin.x, target_pin.y)], Stroke::new(5.0, color));
//                 // }
//             });
//         
//         self.rect = r;
//     }
// }