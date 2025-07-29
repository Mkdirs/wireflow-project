use eframe::egui::{Align, Align2, Color32, FontId, Frame, Id, Label, Layout, Margin, Painter, Pos2, Rect, Response, RichText, Sense, SidePanel, Stroke, StrokeKind, Ui, UiBuilder, Vec2};
use eframe::egui::Shape::CubicBezier;
use eframe::epaint::{CubicBezierShape, Marginf, QuadraticBezierShape, RectShape};
use circuit_sim::component::{Component, ComponentType};
use log::log;

mod logicgates;

const PIN_LENGTH:f32 = 12.0;
const INPUT_SPACING:f32 = 16.0;
const STROKE_SIZE:f32 = 3.0;
const LOGIC_GATE_SIZE:Vec2 = Vec2::new(56.0, 33.0);

pub fn draw(component: &mut Component, ui: &mut Ui) -> Response {
    let pos = Pos2::new(component.x, component.y);
    let rect = Rect::from_min_size(pos, LOGIC_GATE_SIZE);

    ui.put(rect, |ui: &mut Ui| {
        if matches!(component.node_type, ComponentType::COMPOUND(_, _)) {
            return draw_compound(component, ui);
        }

        let (res, painter) = ui.allocate_painter(
            LOGIC_GATE_SIZE,
            Sense::drag()
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
    })
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
    painter.circle_stroke(rect.center(), rect.width()/2.0 - PIN_LENGTH - STROKE_SIZE, Stroke::new(STROKE_SIZE, Color32::WHITE));
    if component.id.contains("input") {
        painter.hline(rect.right()- PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(STROKE_SIZE, Color32::WHITE));
    } else {
        painter.hline(rect.left()+ PIN_LENGTH..=rect.left(), rect.center().y, Stroke::new(STROKE_SIZE, Color32::WHITE));
    };
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
    painter.hline(rect.right()- PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(STROKE_SIZE, Color32::WHITE));
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
                                    painter.hline(start_x..=start_x-PIN_LENGTH*2.0, next_pos.y, Stroke::new(STROKE_SIZE, Color32::WHITE));
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

                            for output in outputs {
                                let pos_y = ui.next_widget_position().y;
                                ui.add(
                                    Label::new(&output.label)
                                        .halign(Align::Center)
                                        .extend()
                                );
                                {
                                    let painter = ui.painter();
                                    let start_x = ui.min_rect().right()+SPACING+STROKE_SIZE;
                                    painter.hline(start_x..=start_x+PIN_LENGTH*2.0, pos_y, Stroke::new(STROKE_SIZE, Color32::WHITE));
                                }
                            }
                        });
                    });
                });
        }
    );

    res.response
}