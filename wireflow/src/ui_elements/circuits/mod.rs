use eframe::egui::{output, Align, Align2, Color32, FontId, Frame, Id, Label, Layout, Margin, Painter, Pos2, Rect, Response, RichText, Sense, SidePanel, Stroke, StrokeKind, Ui, UiBuilder, Vec2};
use eframe::egui::Shape::CubicBezier;
use eframe::epaint::{CubicBezierShape, Marginf, QuadraticBezierShape, RectShape};
use circuit_sim::component::{Component, ComponentType};
use log::log;

mod logicgates;

const PIN_LENGTH:f32 = 12.0;
const INPUT_SPACING:f32 = 16.0;
pub const STROKE_SIZE:f32 = 3.0;
const LOGIC_GATE_SIZE:Vec2 = Vec2::new(56.0, 33.0);


pub fn draw(component: &mut Component, ui: &mut Ui) -> Response {
    let pos = Pos2::new(component.x, component.y);
    let rect = Rect::from_min_size(pos, LOGIC_GATE_SIZE);


    let res = ui.put(rect, |ui: &mut Ui| {
        if matches!(component.node_type, ComponentType::COMPOUND(_, _)) {
            return draw_compound(component, ui);
        }

        let (res, painter) = ui.allocate_painter(
            LOGIC_GATE_SIZE,
            Sense::drag() | Sense::hover()
        );

        match component.node_type {
            ComponentType::OR
            | ComponentType::AND
            | ComponentType::XOR
            | ComponentType::NOT
            | ComponentType::NAND
            | ComponentType::NOR
            | ComponentType::XNOR => draw_logic_gate(component, &painter, &rect),
            ComponentType::IO => draw_io(component, &painter, &rect),
            ComponentType::CLOCK(_,_) => draw_clock(component, &painter, &rect),
            ComponentType::COMPOUND(_, _) => unreachable!(),
        }

        res
    });

    res
}

fn draw_logic_gate(component: &mut Component, painter: &Painter, rect: &Rect) {
    match component.node_type {
        ComponentType::AND => logicgates::draw_and_gate(component, &painter, &rect),
        ComponentType::OR => logicgates::draw_or_gate(component, &painter, &rect),
        ComponentType::NOT => logicgates::draw_not_gate(component, &painter, &rect),
        ComponentType::XOR => logicgates::draw_xor_gate(component, &painter, &rect),
        ComponentType::NAND => logicgates::draw_nand_gate(component, &painter, &rect),
        ComponentType::NOR => logicgates::draw_nor_gate(component, &painter, &rect),
        ComponentType::XNOR => logicgates::draw_xnor_gate(component, &painter, &rect),
        _ => unreachable!()
    }
}

fn draw_io(component: &mut Component, painter: &Painter, rect: &Rect) {
    let stroke = Stroke::new(STROKE_SIZE, Color32::WHITE);
    let input_pos = Pos2::new(rect.left(), rect.center().y);
    let output_pos = Pos2::new(rect.right(), rect.center().y);
    let mut pins_color = [stroke.color;2];

    {
        let input = component.get_mut("input0").unwrap();
        input.x = input_pos.x;
        input.y = input_pos.y;
        pins_color[0] = if input.value { Color32::GREEN } else { pins_color[0] };
    }

    {
        let output = component.get_mut("output0").unwrap();
        output.x = output_pos.x;
        output.y = output_pos.y;
        pins_color[1] = if output.value { Color32::GREEN } else { pins_color[0] };
    }

    let display_label = if component.id.contains("input") {
        "IN"
    } else {
        "OUT"
    };
    let mut font = FontId::default();
    font.size = 10.0;

    painter.circle_stroke(rect.center(), rect.width()/2.0 - PIN_LENGTH - STROKE_SIZE, stroke);
    painter.text(rect.center(), Align2::CENTER_CENTER, display_label, font, Color32::WHITE);
    painter.hline(input_pos.x + PIN_LENGTH..=input_pos.x, input_pos.y, Stroke{
        color: pins_color[0],
        ..stroke
    });
    painter.hline(output_pos.x - PIN_LENGTH..=output_pos.x, rect.center().y, Stroke {
        color: pins_color[1],
        ..stroke
    });
}

fn draw_clock(component: &mut Component, painter: &Painter, rect: &Rect) {
    let size = 10.0;
    painter.circle_stroke(rect.center(), rect.width()/2.0 - PIN_LENGTH - STROKE_SIZE, Stroke::new(STROKE_SIZE, Color32::WHITE));
    let curve = CubicBezierShape::from_points_stroke(
        [
            rect.center()+Vec2::LEFT*size,
            rect.center()+Vec2::LEFT*(size / 2.0)+Vec2::UP*size,
            rect.center()+Vec2::RIGHT*(size / 2.0)+Vec2::DOWN*size,
            rect.center()+Vec2::RIGHT*size
        ],
        false,
        Color32::TRANSPARENT,
        Stroke::new(STROKE_SIZE, Color32::WHITE)
    );
    painter.add(curve);
    let output = component.get_mut("output0").unwrap();
    painter.hline(rect.right()- PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(
        STROKE_SIZE,
        if output.value { Color32::GREEN } else { Color32::WHITE }
    ));
    output.x = rect.right();
    output.y = rect.center().y;

}

fn draw_compound(component: &mut Component, ui: &mut Ui) -> Response {
    const SPACING:f32 = 10.0;
    let res = ui.allocate_new_ui(
        UiBuilder::new()
            .sense(Sense::drag()),
        |ui| {
            Frame::default()
                .inner_margin(SPACING)
                .stroke(Stroke::new(STROKE_SIZE, Color32::WHITE))
                .show(ui, |ui| {
                    ui.style_mut()
                        .spacing.item_spacing = Vec2::new(0.0, SPACING);

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            let mut inputs = component.inputs_mut();
                            inputs.sort_by_key(|input| input.id.clone());

                            for input in inputs {
                                {
                                    let painter = ui.painter();
                                    let next_pos = ui.next_widget_position();
                                    let start_x = next_pos.x-SPACING-STROKE_SIZE;
                                    painter.hline(start_x..=start_x-PIN_LENGTH*2.0, next_pos.y, Stroke::new(
                                        STROKE_SIZE,
                                        if input.value { Color32::GREEN } else { Color32::WHITE }
                                    ));
                                    input.x = start_x-PIN_LENGTH*2.0;
                                    input.y = next_pos.y;
                                }
                                ui.add(Label::new(&input.label)
                                    .halign(Align::Center)
                                    .extend());
                            }
                        });

                        ui.add_space(SPACING*3.0);
                        ui.add(
                            Label::new(
                                RichText::new(&component.label)
                                    .heading()
                            )
                                .halign(Align::Center)
                                .extend()
                        );
                        ui.add_space(SPACING*3.0);

                        ui.vertical(|ui| {
                            let mut outputs = component.outputs_mut();
                            outputs.sort_by_key(|output| output.id.clone());

                            let mut positions:Vec<f32> = vec![];
                            for output in outputs {
                                let pos_y = ui.next_widget_position().y;
                                positions.push(pos_y);
                                ui.add(
                                    Label::new(&output.label)
                                        .halign(Align::Center)
                                        .extend()
                                );
                            }

                            for (ind, pos) in positions.into_iter().enumerate() {
                                let painter = ui.painter();
                                let start_x = ui.min_rect().right()+SPACING+STROKE_SIZE;
                                let output = component.get_mut(&format!("output{ind}")).unwrap();
                                painter.hline(start_x..=start_x+PIN_LENGTH*2.0, pos, Stroke::new(
                                    STROKE_SIZE,
                                    if output.value { Color32::GREEN } else { Color32::WHITE }
                                ));
                                output.x = start_x+PIN_LENGTH*2.0;
                                output.y = pos;
                            }
                        });
                    });
                });
        }
    );

    res.response
}