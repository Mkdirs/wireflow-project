use std::collections::HashMap;
use eframe::egui::{Align, Align2, Color32, FontId, Frame, Label, Painter, Pos2, Rect, Response, RichText, Sense, Stroke, Ui, UiBuilder, Vec2, Widget};
use eframe::epaint::CubicBezierShape;
use circuit_sim::component::Component;
use plugin::components::{AndGate, Clock, Compound, NandGate, NorGate, NotGate, OrGate, XnorGate, XorGate};
use crate::ui_elements::circuits::logicgates::LogicGatesWidget;

mod logicgates;

const PIN_LENGTH:f32 = 12.0;
const INPUT_SPACING:f32 = 16.0;
pub const STROKE_SIZE:f32 = 3.0;

pub type ComponentWidgetResponse = (ComponentPinResponse, Response);

#[derive(Default, Debug)]
pub struct ComponentPinResponse {
    pub pins_position: HashMap<String, Pos2>,
    pub interaction: Option<ComponentPinInteraction>
}

#[derive(Debug)]
pub struct ComponentPinInteraction {
    pub pin_name: String,
    pub event: ComponentPinEvent
}

#[derive(Debug)]
pub enum ComponentPinEvent {
    HOVERED,
    CLICKED
}

trait ComponentWidget {
    fn component(&self) -> &Component;
    fn position(&self) -> Pos2;
    fn size(&self) -> Vec2;
    fn rect(&self) -> Rect {
        Rect::from_min_size(self.position(), self.size())
    }
    fn draw(&self, ui: &mut Ui) -> ComponentWidgetResponse;
}

struct CompoundWidget<'a>{
    component: &'a Component,
    pos: Pos2,
}
impl<'a> ComponentWidget for CompoundWidget<'a> {
    fn component(&self) -> &Component {
        todo!()
    }

    fn position(&self) -> Pos2 {
        todo!()
    }

    fn size(&self) -> Vec2 {
        todo!()
    }

    fn draw(&self, ui: &mut Ui) -> ComponentWidgetResponse {
        todo!()
    }
}

struct ClockWidget<'a>{
    component: &'a Component,
    pos: Pos2,
}
impl<'a> ComponentWidget for ClockWidget<'a> {
    fn component(&self) -> &Component {
        todo!()
    }

    fn position(&self) -> Pos2 {
        todo!()
    }

    fn size(&self) -> Vec2 {
        todo!()
    }

    fn draw(&self, ui: &mut Ui) -> ComponentWidgetResponse {
        todo!()
    }
}

pub fn draw(component: &Component, pos: Pos2, ui: &mut Ui) -> ComponentWidgetResponse {
    let component_type = component.component_type.lock().unwrap();

    let res = if component_type.as_any().is::<OrGate>()
        || component_type.as_any().is::<AndGate>()
        || component_type.as_any().is::<XorGate>()
        || component_type.as_any().is::<NotGate>()
        || component_type.as_any().is::<NandGate>()
        || component_type.as_any().is::<NorGate>()
        || component_type.as_any().is::<XnorGate>()
    {
        drop(component_type);
        let widget = LogicGatesWidget{ component, pos };
        widget.draw(ui)
    } else if component_type.as_any().is::<Clock>() {
        drop(component_type);
        let widget = ClockWidget{ component, pos };
        widget.draw(ui)
    } else {
        drop(component_type);
        let widget = CompoundWidget{ component, pos };
        widget.draw(ui)
    };

    res
}

// fn draw_logic_gate(component: &mut Component, painter: &Painter, rect: &Rect) {
//     let comp_clone = component.component_type.clone();
//     let component_type = comp_clone.lock().unwrap();
//
//     if component_type.as_any().is::<AndGate>() {
//         logicgates::draw_and_gate(component, &painter, &rect);
//     } else if component_type.as_any().is::<OrGate>() {
//         logicgates::draw_or_gate(component, &painter, &rect);
//     } else if component_type.as_any().is::<NotGate>() {
//         logicgates::draw_not_gate(component, &painter, &rect);
//     } else if component_type.as_any().is::<XorGate>() {
//         logicgates::draw_xor_gate(component, &painter, &rect);
//     } else if component_type.as_any().is::<NandGate>() {
//         logicgates::draw_nand_gate(component, &painter, &rect);
//     } else if component_type.as_any().is::<NorGate>() {
//         logicgates::draw_nor_gate(component, &painter, &rect);
//     } else if component_type.as_any().is::<XnorGate>() {
//         logicgates::draw_xnor_gate(component, &painter, &rect);
//     }
// }

// fn draw_io(component: &mut Component, painter: &Painter, rect: &Rect) {
//     let stroke = Stroke::new(STROKE_SIZE, Color32::WHITE);
//     let input_pos = Pos2::new(rect.left(), rect.center().y);
//     let output_pos = Pos2::new(rect.right(), rect.center().y);
//     let mut pins_color = [stroke.color;2];
//
//     {
//         let input = component.get_mut("input0").unwrap();
//         input.x = input_pos.x;
//         input.y = input_pos.y;
//         pins_color[0] = if input.value.iter().fold(false, |acc, e| acc || *e) {
//             Color32::GREEN
//         } else {
//             pins_color[0]
//         };
//     }
//
//     {
//         let output = component.get_mut("output0").unwrap();
//         output.x = output_pos.x;
//         output.y = output_pos.y;
//         pins_color[1] = if output.value.iter().fold(false, |acc, e| acc || *e) {
//             Color32::GREEN
//         } else {
//             pins_color[0]
//         };
//     }
//
//     let display_label = if component.id.contains("input") {
//         "IN"
//     } else {
//         "OUT"
//     };
//     let mut font = FontId::default();
//     font.size = 10.0;
//
//     painter.circle_stroke(rect.center(), rect.width()/2.0 - PIN_LENGTH - STROKE_SIZE, stroke);
//     painter.text(rect.center(), Align2::CENTER_CENTER, display_label, font, Color32::WHITE);
//     painter.hline(input_pos.x + PIN_LENGTH..=input_pos.x, input_pos.y, Stroke{
//         color: pins_color[0],
//         ..stroke
//     });
//     painter.hline(output_pos.x - PIN_LENGTH..=output_pos.x, rect.center().y, Stroke {
//         color: pins_color[1],
//         ..stroke
//     });
// }

// fn draw_clock(component: &mut Component, painter: &Painter, rect: &Rect) {
//     let size = 10.0;
//     painter.circle_stroke(rect.center(), rect.width()/2.0 - PIN_LENGTH - STROKE_SIZE, Stroke::new(STROKE_SIZE, Color32::WHITE));
//     let curve = CubicBezierShape::from_points_stroke(
//         [
//             rect.center()+Vec2::LEFT*size,
//             rect.center()+Vec2::LEFT*(size / 2.0)+Vec2::UP*size,
//             rect.center()+Vec2::RIGHT*(size / 2.0)+Vec2::DOWN*size,
//             rect.center()+Vec2::RIGHT*size
//         ],
//         false,
//         Color32::TRANSPARENT,
//         Stroke::new(STROKE_SIZE, Color32::WHITE)
//     );
//     painter.add(curve);
//     let output = component.get_mut("OUT").unwrap();
//     painter.hline(rect.right()- PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(
//         STROKE_SIZE,
//         if output.value.iter().fold(false, |acc, e| acc || *e) {
//             Color32::GREEN
//         } else {
//             Color32::WHITE
//         }
//     ));
//     output.x = rect.right();
//     output.y = rect.center().y;
//
// }

// fn draw_compound(component: &mut Component, ui: &mut Ui) -> Response {
//     const SPACING:f32 = 10.0;
//     let res = ui.allocate_new_ui(
//         UiBuilder::new()
//             .sense(Sense::drag()),
//         |ui| {
//             Frame::default()
//                 .inner_margin(SPACING)
//                 .stroke(Stroke::new(STROKE_SIZE, Color32::WHITE))
//                 .show(ui, |ui| {
//                     ui.style_mut()
//                         .spacing.item_spacing = Vec2::new(0.0, SPACING);
//
//                     ui.horizontal(|ui| {
//                         ui.vertical(|ui| {
//                             let mut inputs = component.inputs_mut();
//                             inputs.sort_by_key(|input| input.name.clone());
//
//                             for input in inputs {
//                                 {
//                                     let painter = ui.painter();
//                                     let next_pos = ui.next_widget_position();
//                                     let start_x = next_pos.x-SPACING-STROKE_SIZE;
//                                     painter.hline(start_x..=start_x-PIN_LENGTH*2.0, next_pos.y, Stroke::new(
//                                         STROKE_SIZE,
//                                         if input.value.iter().fold(false, |acc, e| acc || *e) {
//                                             Color32::GREEN
//                                         } else {
//                                             Color32::WHITE
//                                         }
//                                     ));
//                                     input.x = start_x-PIN_LENGTH*2.0;
//                                     input.y = next_pos.y;
//                                 }
//                                 ui.add(Label::new(&input.name)
//                                     .halign(Align::Center)
//                                     .extend());
//                             }
//                         });
//
//                         ui.add_space(SPACING*3.0);
//                         ui.add(
//                             Label::new(
//                                 RichText::new(&component.name)
//                                     .heading()
//                             )
//                                 .halign(Align::Center)
//                                 .extend()
//                         );
//                         ui.add_space(SPACING*3.0);
//
//                         ui.vertical(|ui| {
//                             let mut outputs = component.outputs_mut();
//                             outputs.sort_by_key(|output| output.name.clone());
//
//                             let mut positions:Vec<f32> = vec![];
//                             for output in outputs {
//                                 let pos_y = ui.next_widget_position().y;
//                                 positions.push(pos_y);
//                                 ui.add(
//                                     Label::new(&output.name)
//                                         .halign(Align::Center)
//                                         .extend()
//                                 );
//                             }
//
//                             for (ind, pos) in positions.into_iter().enumerate() {
//                                 let painter = ui.painter();
//                                 let start_x = ui.min_rect().right()+SPACING+STROKE_SIZE;
//                                 let output = component.get_mut(&format!("output{ind}")).unwrap();
//                                 painter.hline(start_x..=start_x+PIN_LENGTH*2.0, pos, Stroke::new(
//                                     STROKE_SIZE,
//                                     if output.value.iter().fold(false, |acc, e| acc || *e) {
//                                         Color32::GREEN
//                                     } else {
//                                         Color32::WHITE
//                                     }
//                                 ));
//                                 output.x = start_x+PIN_LENGTH*2.0;
//                                 output.y = pos;
//                             }
//                         });
//                     });
//                 });
//         }
//     );
//
//     res.response
// }