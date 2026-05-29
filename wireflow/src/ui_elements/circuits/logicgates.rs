use eframe::egui::{Color32, Id, Painter, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2};
use eframe::epaint::{CornerRadiusF32, QuadraticBezierShape};
use circuit_sim::component::{Component, Pin};
use plugin::components::{AndGate, NandGate, NorGate, NotGate, OrGate, XnorGate, XorGate};
use crate::ui_elements::circuits::{default_senses, get_pin_interaction, ComponentPinEvent, ComponentPinInteraction, ComponentPinResponse, ComponentWidget, ComponentWidgetResponse, INPUT_SPACING, PIN_LENGTH, STROKE_SIZE};

pub(crate) const LOGIC_GATE_SIZE:Vec2 = Vec2::new(56.0, 33.0);

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

    fn draw(&self, ui: &mut Ui) -> ComponentWidgetResponse {
        let rect = self.rect();

        let mut pin_res = ComponentPinResponse::default();
        let response = ui.put(rect, |ui: &mut Ui| {
            let (res, painter) = ui.allocate_painter(
                self.size(),
                default_senses()
            );

            let component_type = self.component.component_type.lock().unwrap();

            pin_res = if component_type.as_any().is::<AndGate>() {
                draw_and_gate(&ui, self.component, &painter, self.pos, &rect)
            } else if component_type.as_any().is::<OrGate>() {
                draw_or_gate(&ui, self.component, &painter, self.pos, &rect)
            } else if component_type.as_any().is::<XorGate>() {
                draw_xor_gate(&ui, self.component, &painter, self.pos, &rect)
            } else if component_type.as_any().is::<NotGate>() {
                draw_not_gate(&ui, self.component, &painter, &rect)
            } else if component_type.as_any().is::<NandGate>() {
                draw_nand_gate(&ui, self.component, &painter, self.pos, &rect)
            } else if component_type.as_any().is::<NorGate>() {
                draw_nor_gate(&ui, self.component, &painter, self.pos, &rect)
            } else if component_type.as_any().is::<XnorGate>() {
                draw_xnor_gate(&ui, self.component, &painter, self.pos, &rect)
            } else {
                panic!("Unknown component type: {:?}", component_type);
            };

            res
        });

        (pin_res, response)
    }
}

fn draw_and_gate(ui: &Ui, component: &Component, painter: &Painter, pos: Pos2, rect: &Rect) -> ComponentPinResponse {
    let mut corner_radius = CornerRadiusF32::same(0.0);
    corner_radius.ne = 50.0;
    corner_radius.se = 50.0;

    let color = Color32::WHITE;
    let mut response = ComponentPinResponse::default();
    let mut inputs = component.inputs();
    inputs.sort_by_key(|input| input.name.clone());

    let inputs_size = (STROKE_SIZE *inputs.len() as f32)+(inputs.len()-1) as f32 * INPUT_SPACING;
    for (i, input) in inputs.into_iter().enumerate() {
        let y = rect.top() + (i as f32* INPUT_SPACING) + (rect.size().y/2.0 - inputs_size / 2.0) + STROKE_SIZE;
        painter.hline(rect.left()..=rect.left()+ PIN_LENGTH, y, Stroke::new(
            STROKE_SIZE,
            if input.value.iter().fold(false, |acc, e| acc || *e) {
                Color32::GREEN
            } else {
                color
            }
        ));
        let pin_pos = Pos2::new(rect.left(), y);
        response.pins_position.insert(input.name.clone(), pin_pos);

        if response.interaction.is_none() {
            response.interaction = get_pin_interaction(ui, component, input, pin_pos)
        }
    }

    let body_rect = Rect::from_min_size(pos+Vec2::RIGHT* PIN_LENGTH, Vec2::splat(rect.size().y));
    painter.rect_stroke(body_rect, corner_radius, Stroke::new(STROKE_SIZE, color), StrokeKind::Inside);

    let output = component.get("OUT").unwrap();
    painter.hline(rect.right()-PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(
        STROKE_SIZE,
        if output.value.iter().fold(false, |acc, e| acc || *e) {
            Color32::GREEN
        } else {
            color
        }
    ));

    let pin_pos = Pos2::new(rect.right(), rect.center().y);
    response.pins_position.insert(output.name.clone(), pin_pos);
    if response.interaction.is_none() {
        response.interaction = get_pin_interaction(ui, component, output, pin_pos);
    }

    response
}

fn draw_or_gate(ui: &Ui, component: &Component, painter: &Painter, pos: Pos2, rect: &Rect) -> ComponentPinResponse {
    let color = Color32::WHITE;

    let mut inputs = component.inputs();
    let mut response = ComponentPinResponse::default();
    inputs.sort_by_key(|input| input.name.clone());

    let inputs_size = (STROKE_SIZE *inputs.len() as f32)+(inputs.len()-1) as f32 * INPUT_SPACING;
    for (i, input) in inputs.into_iter().enumerate() {
        let y = rect.top() + (i as f32* INPUT_SPACING) + (rect.size().y/2.0 - inputs_size / 2.0) + STROKE_SIZE;
        painter.hline(rect.left()..=rect.left()+ PIN_LENGTH, y, Stroke::new(
            STROKE_SIZE,
            if input.value.iter().fold(false, |acc, e| acc || *e) {
                Color32::GREEN
            } else {
                color
            }
        ));

        let pin_pos = Pos2::new(rect.left(), y);
        response.pins_position.insert(input.name.clone(), pin_pos);
        if response.interaction.is_none() {
            response.interaction = get_pin_interaction(ui, component, input, pin_pos);
        }
    }

    let body_rect = Rect::from_min_size(pos+Vec2::RIGHT*PIN_LENGTH, Vec2::splat(rect.size().y));
    let quad_bezier = QuadraticBezierShape::from_points_stroke([body_rect.left_top(), body_rect.left_center()+Vec2::RIGHT*5.0, body_rect.left_bottom()], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE, color));
    let body_bezier = QuadraticBezierShape::from_points_stroke([body_rect.left_top(), rect.right_center()+Vec2::RIGHT*PIN_LENGTH*1.5, body_rect.left_bottom()], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE, color));

    painter.add(body_bezier);
    painter.add(quad_bezier);

    let output = component.get("OUT").unwrap();
    painter.hline(rect.right()-PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(
        STROKE_SIZE,
        if output.value.iter().fold(false, |acc, e| acc || *e) {
            Color32::GREEN
        } else {
            color
        }
    ));

    let pin_pos = Pos2::new(rect.right(), rect.center().y);
    response.pins_position.insert(output.name.clone(), pin_pos);
    if response.interaction.is_none() {
        response.interaction = get_pin_interaction(ui, component, output, pin_pos);
    }

    response
}

fn draw_not_gate(ui: &Ui, component: &Component, painter: &Painter, rect: &Rect) -> ComponentPinResponse {
    let color = Color32::WHITE;

    let mut inputs = component.inputs();
    let mut response = ComponentPinResponse::default();
    inputs.sort_by_key(|input| input.name.clone());

    let inputs_size = (STROKE_SIZE *inputs.len() as f32)+(inputs.len()-1) as f32 * INPUT_SPACING;
    for (i, input) in inputs.into_iter().enumerate() {
        let y = rect.top() + (i as f32* INPUT_SPACING) + (rect.size().y/2.0 - inputs_size / 2.0) + STROKE_SIZE;
        painter.hline(rect.left()..=rect.left()+ PIN_LENGTH, y, Stroke::new(
            STROKE_SIZE,
            if input.value.iter().fold(false, |acc, e| acc || *e) {
                Color32::GREEN
            } else {
                color
            }
        ));

        let pin_pos = Pos2::new(rect.left(), y);
        response.pins_position.insert(input.name.clone(), pin_pos);
        if response.interaction.is_none() {
            response.interaction = get_pin_interaction(ui, component, input, pin_pos);
        }
    }

    painter.vline(rect.left()+PIN_LENGTH, rect.top()..=rect.bottom(), Stroke::new(STROKE_SIZE, color));
    painter.line_segment([rect.left_top()+Vec2::RIGHT*PIN_LENGTH, rect.right_center()-Vec2::RIGHT*PIN_LENGTH], Stroke::new(STROKE_SIZE, color));
    painter.line_segment([rect.left_bottom()+Vec2::RIGHT*PIN_LENGTH, rect.right_center()-Vec2::RIGHT*PIN_LENGTH], Stroke::new(STROKE_SIZE, color));

    let output = component.get("OUT").unwrap();
    painter.hline(rect.right()-PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(
        STROKE_SIZE,
        if output.value.iter().fold(false, |acc, e| acc || *e) {
            Color32::GREEN
        } else {
            color
        }
    ));

    painter.circle_filled(rect.right_center()-Vec2::RIGHT*PIN_LENGTH, STROKE_SIZE+1.5, color);

    let pin_pos = Pos2::new(rect.right(), rect.center().y);
    response.pins_position.insert(output.name.clone(), pin_pos);
    if response.interaction.is_none() {
        response.interaction = get_pin_interaction(ui, component, output, pin_pos);
    }

    response
}

fn draw_xor_gate(ui: &Ui, component: &Component, painter: &Painter, pos: Pos2, rect: &Rect) -> ComponentPinResponse {
    let spacing = 7.0;
    let response = draw_or_gate(ui, component, painter, pos, rect);
    let quad_bezier = QuadraticBezierShape::from_points_stroke([rect.left_top()+Vec2::RIGHT*spacing, rect.left_center()+Vec2::RIGHT*(5.0+spacing), rect.left_bottom()+Vec2::RIGHT*spacing], false, Color32::TRANSPARENT, Stroke::new(STROKE_SIZE, Color32::WHITE));

    painter.add(quad_bezier);

    response
}

fn draw_nand_gate(ui: &Ui, component: &Component, painter: &Painter, pos: Pos2, rect: &Rect) -> ComponentPinResponse {
    let response = draw_and_gate(ui, component, painter, pos, rect);
    painter.circle_filled(rect.right_center()-Vec2::RIGHT*PIN_LENGTH, STROKE_SIZE+1.5, Color32::WHITE);

    response
}

fn draw_nor_gate(ui: &Ui, component: &Component, painter: &Painter, pos: Pos2, rect: &Rect) -> ComponentPinResponse {
    let response = draw_or_gate(ui, component, painter, pos, rect);
    painter.circle_filled(rect.right_center()-Vec2::RIGHT*PIN_LENGTH, STROKE_SIZE+1.5, Color32::WHITE);

    response
}

fn draw_xnor_gate(ui: &Ui, component: &Component, painter: &Painter, pos: Pos2, rect: &Rect) -> ComponentPinResponse {
    let response = draw_xor_gate(ui, component, painter, pos, rect);
    painter.circle_filled(rect.right_center()-Vec2::RIGHT*PIN_LENGTH, STROKE_SIZE+1.5, Color32::WHITE);

    response
}