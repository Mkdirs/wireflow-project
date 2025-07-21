use eframe::egui::{Color32, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};
use circuit_sim::component::{Component, ComponentType};
use log::log;

mod logicgates;

const PIN_LENGTH:f32 = 12.0;
const INPUT_SPACING:f32 = 16.0;
const STROKE_SIZE:f32 = 3.0;
const LOGIC_GATE_SIZE:Vec2 = Vec2::new(56.0, 33.0);

pub fn draw(component: &mut Component, ui: &mut Ui) -> Response {
    match component.node_type {
        ComponentType::OR
        | ComponentType::AND
        | ComponentType::XOR
        | ComponentType::NOT
        | ComponentType::NAND
        | ComponentType::NOR
        | ComponentType::XNOR => draw_logic_gate(component, ui),
        ComponentType::IO => {
            let pos = Pos2::new(component.x, component.y);
            let rect = Rect::from_min_size(pos, LOGIC_GATE_SIZE);

            ui.put(rect, |ui: &mut Ui| {
                let (res, painter) = ui.allocate_painter(
                    LOGIC_GATE_SIZE,
                    Sense::drag()
                );

                draw_io(component, &painter, &rect);

                res
            })
        },

        ComponentType::CLOCK(_,_) => todo!("Clock visual not done yet !"),
        ComponentType::COMPOUND(_, _) => todo!("Compound visual not done yet !"),

    }
}

fn draw_logic_gate(component: &mut Component, ui: &mut Ui) -> Response {
    let pos = Pos2::new(component.x, component.y);
    let rect = Rect::from_min_size(pos, LOGIC_GATE_SIZE);

    let color = Color32::WHITE;

    ui.put(rect, |ui: &mut Ui| {
        let (res, painter) = ui.allocate_painter(
            LOGIC_GATE_SIZE,
            Sense::drag()
        );

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

        res
    })
}

fn draw_io(component: &mut Component, painter: &Painter, rect: &Rect) {
    let color = if component.id.contains("input") {
        Color32::GREEN
    } else {
        Color32::RED
    };

    painter.hline(rect.left()+ PIN_LENGTH..=rect.left(), rect.center().y, Stroke::new(STROKE_SIZE, Color32::WHITE));

    painter.circle_stroke(rect.center(), rect.width()/2.0 - PIN_LENGTH - STROKE_SIZE, Stroke::new(STROKE_SIZE, color));

    painter.hline(rect.right()- PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(STROKE_SIZE, Color32::WHITE));
}