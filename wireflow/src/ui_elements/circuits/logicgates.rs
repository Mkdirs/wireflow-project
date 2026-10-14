use crate::ui_elements::circuits::{get_pin_interaction, with_body, ComponentPinResponse, ComponentWidget, ComponentWidgetResponse, INPUT_SPACING, STROKE_SIZE, PIN_COLOR, PIN_RADIUS, with_inputs, with_pin};
use wireflowapi::component::Component;
use eframe::egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2};
use eframe::epaint::{CornerRadiusF32, QuadraticBezierShape};
use wireflowapi::component::primitives::{AndGate, NandGate, NorGate, NotGate, OrGate, XnorGate, XorGate};

pub(crate) const LOGIC_GATE_SIZE: Vec2 = Vec2::new(44.0, 44.0);

pub(crate) struct LogicGatesWidget<'a>{
    pub component: &'a Component,
    pub pos: Pos2,
}

impl<'a> ComponentWidget for LogicGatesWidget<'a> {
    fn component(&self) -> &Component {
        self.component
    }

    fn position(&self) -> Pos2 {
        self.pos
    }

    fn size(&self) -> Vec2 {
        LOGIC_GATE_SIZE
    }

    fn draw(&self, ui: &mut Ui, interactive: bool) -> ComponentWidgetResponse {
        let rect = self.rect();
        let component_type = self.component.component_type.lock().unwrap();

        let response = if component_type.as_any().is::<AndGate>() {
            draw_and_gate(ui, self.component, self.pos, &rect, interactive)
        } else if component_type.as_any().is::<OrGate>() {
            draw_or_gate(ui, self.component, self.pos, &rect, interactive)
        } else if component_type.as_any().is::<XorGate>() {
            draw_xor_gate(ui, self.component, self.pos, &rect, interactive)
        } else if component_type.as_any().is::<NotGate>() {
            draw_not_gate(ui, self.component, &rect, interactive)
        } else if component_type.as_any().is::<NandGate>() {
            draw_nand_gate(ui, self.component, self.pos, &rect, interactive)
        } else if component_type.as_any().is::<NorGate>() {
            draw_nor_gate(ui, self.component, self.pos, &rect, interactive)
        } else if component_type.as_any().is::<XnorGate>() {
            draw_xnor_gate(ui, self.component, self.pos, &rect, interactive)
        } else {
            panic!("Unknown component type: {:?}", component_type);
        };

        response
    }
}

fn draw_and_gate(ui: &mut Ui, component: &Component, pos: Pos2, rect: &Rect, interactive: bool) -> ComponentWidgetResponse {
    let mut corner_radius = CornerRadiusF32::same(0.0);
    corner_radius.ne = 50.0;
    corner_radius.se = 50.0;

    let color = Color32::WHITE;
    let mut pin_response = ComponentPinResponse::default();

    let response = with_body(ui, interactive, *rect, |res, painter| {
        let body_rect = Rect::from_min_size(pos, Vec2::splat(rect.size().y));
        painter.rect_stroke(body_rect, corner_radius, Stroke::new(STROKE_SIZE, color), StrokeKind::Inside);
    });

    with_inputs(ui, *rect, component, interactive, &mut pin_response);

    let output = component.get("OUT").unwrap();
    let pin_pos = Pos2::new(rect.right() - STROKE_SIZE / 2.0, rect.center().y);
    with_pin(ui, component, output, pin_pos, interactive, &mut pin_response);

    (pin_response, response)
}

fn draw_or_gate(ui: &mut Ui, component: &Component, pos: Pos2, rect: &Rect, interactive: bool) -> ComponentWidgetResponse {
    let color = Color32::WHITE;
    let mut pin_response = ComponentPinResponse::default();

    let response = with_body(ui, interactive, *rect, |res, painter| {
        let line_bezier = QuadraticBezierShape::from_points_stroke([rect.left_top(), rect.left_center()+Vec2::RIGHT*5.0, rect.left_bottom()], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE, color));
        let body_bezier = QuadraticBezierShape::from_points_stroke([rect.left_top(), rect.right_center()+Vec2::RIGHT*rect.width(), rect.left_bottom()], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE, color));

        painter.add(body_bezier);
        painter.add(line_bezier);
    });

    with_inputs(ui, *rect, component, interactive, &mut pin_response);

    let output = component.get("OUT").unwrap();
    let pin_pos = Pos2::new(rect.right() - STROKE_SIZE / 2.0, rect.center().y);
    with_pin(ui, component, output, pin_pos, interactive, &mut pin_response);

    (pin_response, response)
}

fn draw_not_gate(ui: &mut Ui, component: &Component, rect: &Rect, interactive: bool) -> ComponentWidgetResponse {
    let color = Color32::WHITE;
    
    let mut pin_response = ComponentPinResponse::default();

    let response = with_body(ui, interactive, *rect, |res, painter| {
        painter.vline(rect.left(), rect.top()..=rect.bottom(), Stroke::new(STROKE_SIZE, color));
        painter.line_segment([rect.left_top(), rect.right_center()], Stroke::new(STROKE_SIZE, color));
        painter.line_segment([rect.left_bottom(), rect.right_center()], Stroke::new(STROKE_SIZE, color));
    });

    with_inputs(ui, *rect, component, interactive, &mut pin_response);

    let output = component.get("OUT").unwrap();
    let pin_pos = Pos2::new(rect.right() - STROKE_SIZE / 2.0, rect.center().y);

    ui.painter().circle_filled(
        pin_pos,
        STROKE_SIZE + 1.5,
        color
    );

    pin_response.pins_position.insert(output.name.clone(), pin_pos);
    if pin_response.interaction.is_none() {
        pin_response.interaction = get_pin_interaction(ui, component, output, pin_pos, interactive);
    }

    (pin_response, response)
}

fn draw_xor_gate(ui: &mut Ui, component: &Component, pos: Pos2, rect: &Rect, interactive: bool) -> ComponentWidgetResponse {
    let spacing = 3.0;
    let quad_bezier = QuadraticBezierShape::from_points_stroke([rect.left_top() - Vec2::RIGHT*spacing, rect.left_center(), rect.left_bottom() - Vec2::RIGHT*spacing], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE / 2.0, Color32::WHITE));
    ui.painter().add(quad_bezier);
    let response = draw_or_gate(ui, component, pos, rect, interactive);

    response
}

fn draw_nand_gate(ui: &mut Ui, component: &Component, pos: Pos2, rect: &Rect, interactive: bool) -> ComponentWidgetResponse {
    let response = draw_and_gate(ui, component, pos, rect, interactive);
    ui.painter().circle_filled(rect.right_center() - Vec2::RIGHT * (STROKE_SIZE / 2.0), STROKE_SIZE+1.5, Color32::WHITE);

    response
}

fn draw_nor_gate(ui: &mut Ui, component: &Component, pos: Pos2, rect: &Rect, interactive: bool) -> ComponentWidgetResponse {
    let response = draw_or_gate(ui, component, pos, rect, interactive);
    ui.painter().circle_filled(rect.right_center() - Vec2::RIGHT * (STROKE_SIZE / 2.0), STROKE_SIZE+1.5, Color32::WHITE);

    response
}

fn draw_xnor_gate(ui: &mut Ui, component: &Component, pos: Pos2, rect: &Rect, interactive: bool) -> ComponentWidgetResponse {
    let response = draw_xor_gate(ui, component, pos, rect, interactive);
    ui.painter().circle_filled(rect.right_center() - Vec2::RIGHT * (STROKE_SIZE / 2.0), STROKE_SIZE+1.5, Color32::WHITE);

    response
}