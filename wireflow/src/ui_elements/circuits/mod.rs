use std::collections::HashMap;
use eframe::egui::{Align2, Color32, FontId, Id, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};
use eframe::epaint::CubicBezierShape;
use circuit_sim::component::{Component, Pin};
use plugin::components::{AndGate, Clock, In, NandGate, NorGate, NotGate, OrGate, Out, XnorGate, XorGate};
use crate::ui_elements::circuits::compound::CompoundWidget;
use crate::ui_elements::circuits::logicgates::{LogicGatesWidget, LOGIC_GATE_SIZE};

mod logicgates;
pub mod compound;

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

pub trait ComponentWidget {
    fn component(&self) -> &Component;
    fn position(&self) -> Pos2;
    fn size(&self) -> Vec2;
    fn rect(&self) -> Rect {
        Rect::from_min_size(self.position(), self.size())
    }
    fn draw(&self, ui: &mut Ui) -> ComponentWidgetResponse;
}

struct ClockWidget<'a>{
    component: &'a Component,
    pos: Pos2,
}

struct IOWidget<'a> {
    component: &'a Component,
    pos: Pos2,
    is_input: bool
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
        let widget = ClockWidget { component, pos };
        widget.draw(ui)
    } else if component_type.as_any().is::<In>() {
        drop(component_type);
        let widget = IOWidget { component, pos, is_input: true };
        widget.draw(ui)
    } else if component_type.as_any().is::<Out>() {
        drop(component_type);
        let widget = IOWidget { component, pos, is_input: false };
        widget.draw(ui)
    } else {
        drop(component_type);
        let widget = CompoundWidget{ component, pos };
        widget.draw(ui)
    };

    res
}

impl<'a> ComponentWidget for IOWidget<'a> {
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
                Sense::drag() | Sense::hover()
            );

            let stroke = Stroke::new(STROKE_SIZE, Color32::WHITE);

            let (pin, pin_pos) = match self.is_input {
                true => {
                    let output = self.component.get("OUT").unwrap();
                    let output_pos = Pos2::new(rect.right(), rect.center().y);

                    (output, output_pos)
                },
                false => {
                    let input = self.component.get("IN").unwrap();
                    let input_pos = Pos2::new(rect.left(), rect.center().y);

                    (input, input_pos)
                },
            };

            let pin_color = if pin.value.iter().fold(false, |acc, e| acc || *e) {
                Color32::GREEN
            } else {
                stroke.color
            };

            let display_label = if self.is_input {
                "IN"
            } else {
                "OUT"
            };
            let mut font = FontId::default();
            font.size = 10.0;

            painter.circle_stroke(rect.center(), rect.width()/2.0 - PIN_LENGTH - STROKE_SIZE, stroke);
            painter.text(rect.center(), Align2::CENTER_CENTER, display_label, font, Color32::WHITE);

            if self.is_input {
                painter.hline(pin_pos.x - PIN_LENGTH..=pin_pos.x, pin_pos.y, Stroke{
                    color: pin_color,
                    ..stroke
                });
            } else {
                painter.hline(pin_pos.x + PIN_LENGTH..=pin_pos.x, pin_pos.y, Stroke {
                    color: pin_color,
                    ..stroke
                });
            }

            pin_res.pins_position.insert(pin.name.clone(), pin_pos);

            pin_res.interaction = get_pin_interaction(ui, self.component, pin, pin_pos);

            res
        });

        (pin_res, response)
    }
}
impl<'a> ComponentWidget for ClockWidget<'a> {
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
                Sense::drag() | Sense::hover()
            );

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
            let output = self.component.get("OUT").unwrap();
            painter.hline(rect.right()- PIN_LENGTH..=rect.right(), rect.center().y, Stroke::new(
                STROKE_SIZE,
                if output.value.iter().fold(false, |acc, e| acc || *e) {
                    Color32::GREEN
                } else {
                    Color32::WHITE
                }
            ));
            let pin_pos = Pos2::new(rect.right(), rect.center().y);
            pin_res.pins_position.insert(output.name.clone(), pin_pos);

            pin_res.interaction = get_pin_interaction(ui, self.component, output, pin_pos);
            res
        });

        (pin_res, response)
    }
}

pub(self) fn get_pin_interaction(ui: &Ui, component: &Component, pin: &Pin, pin_pos: Pos2) -> Option<ComponentPinInteraction> {
    let pin_rect = Rect::from_center_size(pin_pos, Vec2::splat(8.0));
    let pin_response = ui.interact(
        pin_rect,
        Id::from(format!("{cmp}.{pin_id}", cmp=component.id, pin_id=pin.name)),
        Sense::hover() | Sense::click()
    );

    if pin_response.clicked() || pin_response.hovered() {
        return pin_response.clicked()
            .then_some(ComponentPinInteraction {
                pin_name: pin.name.clone(),
                event: ComponentPinEvent::CLICKED
            })
            .or(pin_response.hovered().then_some(ComponentPinInteraction {
                pin_name: pin.name.clone(),
                event: ComponentPinEvent::HOVERED
            }));
    }

    None
}