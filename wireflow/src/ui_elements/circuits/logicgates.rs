use eframe::egui::{Color32, Painter, Pos2, Rect, Stroke, StrokeKind, Vec2};
use eframe::egui::accesskit::Role::Code;
use eframe::egui::ImageData::Color;
use eframe::egui::Key::Colon;
use eframe::epaint::{CornerRadiusF32, QuadraticBezierShape};
use circuit_sim::component::Component;

const PIN_LENGTH:f32 = 12.0;
const INPUT_SPACING:f32 = 16.0;
const STROKE_SIZE:f32 = 3.0;

pub fn draw_and_gate(component: &mut Component, painter: &Painter, rect: &Rect) {
    let mut corner_radius = CornerRadiusF32::same(0.0);
    corner_radius.ne = 50.0;
    corner_radius.se = 50.0;

    let color = Color32::WHITE;
    let pos = Pos2::new(component.x, component.y);

    let mut inputs = component.inputs_mut();
    inputs.sort_by_key(|input| input.id.clone());

    let inputs_size = (STROKE_SIZE *inputs.len() as f32)+(inputs.len()-1) as f32 * INPUT_SPACING;
    for (i, input) in inputs.iter().enumerate() {
        painter.hline(rect.left()..=rect.left()+ PIN_LENGTH, rect.top() + (i as f32* INPUT_SPACING) + (rect.size().y/2.0 - inputs_size / 2.0) + STROKE_SIZE, Stroke::new(STROKE_SIZE, color));
    }

    let body_rect = Rect::from_min_size(pos+Vec2::RIGHT* PIN_LENGTH, Vec2::splat(rect.size().y));
    painter.rect_stroke(body_rect, corner_radius, Stroke::new(STROKE_SIZE, color), StrokeKind::Inside);

    painter.hline(rect.right()-PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(STROKE_SIZE, color));
}

pub fn draw_or_gate(component: &mut Component, painter: &Painter, rect: &Rect) {
    let mut corner_radius = CornerRadiusF32::same(0.0);
    corner_radius.ne = 50.0;
    corner_radius.se = 50.0;

    let color = Color32::WHITE;
    let pos = Pos2::new(component.x, component.y);

    let mut inputs = component.inputs_mut();
    inputs.sort_by_key(|input| input.id.clone());

    let inputs_size = (STROKE_SIZE *inputs.len() as f32)+(inputs.len()-1) as f32 * INPUT_SPACING;
    for (i, input) in inputs.iter().enumerate() {
        painter.hline(rect.left()..=rect.left()+ PIN_LENGTH, rect.top() + (i as f32* INPUT_SPACING) + (rect.size().y/2.0 - inputs_size / 2.0) + STROKE_SIZE, Stroke::new(STROKE_SIZE, color));
    }

    let body_rect = Rect::from_min_size(pos+Vec2::RIGHT*PIN_LENGTH, Vec2::splat(rect.size().y));
    let quad_bezier = QuadraticBezierShape::from_points_stroke([body_rect.left_top(), body_rect.left_center()+Vec2::RIGHT*5.0, body_rect.left_bottom()], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE, color));
    let body_bezier = QuadraticBezierShape::from_points_stroke([body_rect.left_top(), rect.right_center()+Vec2::RIGHT*PIN_LENGTH*1.5, body_rect.left_bottom()], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE, color));

    painter.add(body_bezier);
    painter.add(quad_bezier);

    painter.hline(rect.right()-PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(STROKE_SIZE, color));
}

pub fn draw_not_gate(component: &mut Component, painter: &Painter, rect: &Rect) {
    let color = Color32::WHITE;

    let mut inputs = component.inputs_mut();
    inputs.sort_by_key(|input| input.id.clone());

    let inputs_size = (STROKE_SIZE *inputs.len() as f32)+(inputs.len()-1) as f32 * INPUT_SPACING;
    for (i, input) in inputs.iter().enumerate() {
        painter.hline(rect.left()..=rect.left()+ PIN_LENGTH, rect.top() + (i as f32* INPUT_SPACING) + (rect.size().y/2.0 - inputs_size / 2.0) + STROKE_SIZE, Stroke::new(STROKE_SIZE, color));
    }

    painter.vline(rect.left()+PIN_LENGTH, rect.top()..=rect.bottom(), Stroke::new(STROKE_SIZE, color));
    painter.line_segment([rect.left_top()+Vec2::RIGHT*PIN_LENGTH, rect.right_center()-Vec2::RIGHT*PIN_LENGTH], Stroke::new(STROKE_SIZE, color));
    painter.line_segment([rect.left_bottom()+Vec2::RIGHT*PIN_LENGTH, rect.right_center()-Vec2::RIGHT*PIN_LENGTH], Stroke::new(STROKE_SIZE, color));

    painter.circle_filled(rect.right_center()-Vec2::RIGHT*PIN_LENGTH, STROKE_SIZE+1.5, color);

    painter.hline(rect.right()-PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(STROKE_SIZE, color));
}

pub fn draw_xor_gate(component: &mut Component, painter: &Painter, rect: &Rect) {
    let spacing = 8.0;
    draw_or_gate(component, painter, rect);
    let quad_bezier = QuadraticBezierShape::from_points_stroke([rect.left_top()+Vec2::RIGHT*spacing, rect.left_center()+Vec2::RIGHT*(5.0+spacing), rect.left_bottom()+Vec2::RIGHT*spacing], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE/2.0, Color32::WHITE));

    painter.add(quad_bezier);
}

pub fn draw_io(component: &mut Component, painter: &Painter, rect: &Rect) {
    let color = if component.id.contains("input") {
        Color32::GREEN
    } else {
        Color32::RED
    };

    painter.hline(rect.left()+PIN_LENGTH..=rect.left(), rect.center().y, Stroke::new(STROKE_SIZE, Color32::WHITE));

    painter.circle_filled(rect.center(), rect.width()/2.0 - PIN_LENGTH, color);

    painter.hline(rect.right()-PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(STROKE_SIZE, Color32::WHITE));
}