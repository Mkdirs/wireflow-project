use eframe::egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};
use circuit_sim::component::{Component, ComponentType};
use log::log;

mod logicgates;

const LOGIC_GATE_SIZE:Vec2 = Vec2::new(56.0, 33.0);

pub fn draw(component: &mut Component, ui: &mut Ui) -> Response {
    match component.node_type {
        ComponentType::OR
        | ComponentType::AND
        | ComponentType::XOR
        | ComponentType::NOT => draw_logic_gate(component, ui),
        ComponentType::IO => {
            let pos = Pos2::new(component.x, component.y);
            let rect = Rect::from_min_size(pos, LOGIC_GATE_SIZE);

            ui.put(rect, |ui: &mut Ui| {
                let (res, painter) = ui.allocate_painter(
                    LOGIC_GATE_SIZE,
                    Sense::drag()
                );

                logicgates::draw_io(component, &painter, &rect);

                res
            })
        }
        _ => todo!()
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
            _ => {}
        }

        res
    })
}