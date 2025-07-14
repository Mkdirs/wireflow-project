// use eframe::egui::{Align, Color32, CornerRadius, Frame, Layout, Pos2, Response, Sense, Shape, Spacing, Stroke, StrokeKind, Ui, UiBuilder, Vec2};
// use eframe::egui::Shape::CubicBezier;
// use eframe::emath::Rect;
// use eframe::epaint::{CornerRadiusF32, CubicBezierShape, PathShape, PathStroke, QuadraticBezierShape};
// use circuit_sim::component::{Component, ComponentType, Pin};
// use crate::simscene::SimScene;
// 
// const PIN_LENGTH:f32 = 12.0;
// const INPUT_SPACING:f32 = 16.0;
// const STROKE_SIZE:f32 = 3.0;
// const LOGIC_GATE_SIZE:Vec2 = Vec2::new(56.0, 33.0);
// 
// fn draw_pin(pin: &mut Pin, ui: &mut Ui) -> Response {
//     let (response, painter) = ui.allocate_painter(
//         Vec2::new(24.0, 3.0),
//         Sense::click()
//     );
// 
//     painter.rect_filled(response.rect, 0.0, Color32::WHITE);
// 
//     response
// }
// 
// fn draw_curved_logical_gate_body(rect:Rect, color:Color32, curved_front:bool, ui: &mut Ui) {
//     let painter = ui.painter();
//     let mut corner = CornerRadiusF32::same(0.0);
//     corner.ne = 50.0;
//     corner.se = 50.0;
//     
//     painter.rect_stroke(rect, corner, Stroke::new(3.0, color), StrokeKind::Inside);
// }
// 
// impl SimScene {
//     pub fn draw_and_gate(&mut self, component: &mut Component, ui: &mut Ui) -> Response {
//         let pos = Pos2::new(component.x, component.y);
//         let rect = Rect::from_min_size(pos, LOGIC_GATE_SIZE);
// 
//         let color = Color32::WHITE;
// 
//         let mut corner_radius = CornerRadiusF32::same(0.0);
//         corner_radius.ne = 50.0;
//         corner_radius.se = 50.0;
// 
//         ui.put(rect, |ui: &mut Ui| {
//             let (res, painter) = ui.allocate_painter(
//                 LOGIC_GATE_SIZE,
//                 Sense::drag()
//             );
// 
//             let mut inputs = component.inputs_mut();
//             inputs.sort_by_key(|input| input.id.clone());
//             let inputs_size = (STROKE_SIZE*inputs.len() as f32)+(inputs.len()-1) as f32 * INPUT_SPACING;
//             for (i, input) in inputs.iter().enumerate() {
//                 painter.hline(rect.left()..=rect.left()+PIN_LENGTH, rect.top() + (i as f32*INPUT_SPACING) + (LOGIC_GATE_SIZE.y/2.0 - inputs_size / 2.0) + STROKE_SIZE, Stroke::new(STROKE_SIZE, color));
//             }
// 
//             let body_rect = Rect::from_min_size(pos+Vec2::RIGHT*PIN_LENGTH, Vec2::splat(LOGIC_GATE_SIZE.y));
//             painter.rect_stroke(body_rect, corner_radius, Stroke::new(STROKE_SIZE, color), StrokeKind::Inside);
// 
//             painter.hline(rect.right()-PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(STROKE_SIZE, color));
// 
//             //painter.debug_rect(rect, Color32::RED, format!("{}-{}", component.label, component.id));
// 
//             res
//         })
//     }
// 
//     fn draw_or_gate(&mut self, component: &mut Component, ui: &mut Ui) -> Response {
//         let pos = Pos2::new(component.x, component.y);
//         let rect = Rect::from_min_size(pos, LOGIC_GATE_SIZE);
// 
//         let color = Color32::WHITE;
// 
//         ui.put(rect, |ui: &mut Ui| {
//             let (res, painter) = ui.allocate_painter(
//                 LOGIC_GATE_SIZE,
//                 Sense::drag()
//             );
// 
//             let mut inputs = component.inputs_mut();
//             inputs.sort_by_key(|input| input.id.clone());
//             let inputs_size = (STROKE_SIZE*inputs.len() as f32)+(inputs.len()-1) as f32 * INPUT_SPACING;
//             for (i, input) in inputs.iter().enumerate() {
//                 painter.hline(rect.left()..=rect.left()+PIN_LENGTH, rect.top() + (i as f32*INPUT_SPACING) + (LOGIC_GATE_SIZE.y/2.0 - inputs_size / 2.0) + STROKE_SIZE, Stroke::new(STROKE_SIZE, color));
//             }
// 
//             let body_rect = Rect::from_min_size(pos+Vec2::RIGHT*PIN_LENGTH, Vec2::splat(LOGIC_GATE_SIZE.y));
//             let quad_bezier = QuadraticBezierShape::from_points_stroke([body_rect.left_top(), body_rect.left_center()+Vec2::RIGHT*5.0, body_rect.left_bottom()], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE, color));
//             let body_bezier = QuadraticBezierShape::from_points_stroke([body_rect.left_top(), rect.right_center()+Vec2::RIGHT*PIN_LENGTH*1.5, body_rect.left_bottom()], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE, color));
// 
//             painter.add(body_bezier);
//             painter.add(quad_bezier);
// 
//             painter.hline(rect.right()-PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(STROKE_SIZE, color));
// 
//             //painter.debug_rect(rect, Color32::RED, format!("{}-{}", component.label, component.id));
// 
//             res
//         })
//     }
//     pub fn draw_comp(&mut self, component: &mut Component, ui: &mut Ui) -> Response {
//         match component.node_type {
//             ComponentType::AND => self.draw_and_gate(component, ui),
//             ComponentType::OR => self.draw_or_gate(component, ui),
//             _ => todo!()
//         }
//     }
// }