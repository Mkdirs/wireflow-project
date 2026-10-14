use crate::ui_elements::circuits::compound::CompoundWidget;
use crate::ui_elements::circuits::logicgates::{LOGIC_GATE_SIZE, LogicGatesWidget};
use wireflowapi::component::{Component, Pin};
use eframe::egui::{Align2, Color32, FontId, Id, IntoAtoms, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};
use eframe::epaint::CubicBezierShape;
use std::collections::HashMap;
use wireflowapi::component::primitives::{AndGate, Clock, In, NandGate, NorGate, NotGate, OrGate, Out, XnorGate, XorGate};

mod logicgates;
pub mod compound;

pub const PIN_LENGTH:f32 = 6.0;
pub const PIN_RADIUS:f32 = 4.0;
pub const PIN_COLOR:Color32 = Color32::WHITE;
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
    fn draw(&self, ui: &mut Ui, interactive: bool) -> ComponentWidgetResponse;
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

pub(super) fn default_senses(interactive: bool) -> Sense {
    if interactive {
        Sense::click_and_drag() | Sense::hover()
    } else {
        Sense::empty()
    }
}

pub fn draw(ui: &mut Ui, component: &Component, pos: Pos2, interactive: bool) -> ComponentWidgetResponse {
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
        widget.draw(ui, interactive)
    } else if component_type.as_any().is::<Clock>() {
        drop(component_type);
        let widget = ClockWidget { component, pos };
        widget.draw(ui, interactive)
    } else if component_type.as_any().is::<In>() {
        drop(component_type);
        let widget = IOWidget { component, pos, is_input: true };
        widget.draw(ui, interactive)
    } else if component_type.as_any().is::<Out>() {
        drop(component_type);
        let widget = IOWidget { component, pos, is_input: false };
        widget.draw(ui, interactive)
    } else {
        drop(component_type);
        let widget = CompoundWidget{ component, pos };
        widget.draw(ui, interactive)
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

    fn draw(&self, ui: &mut Ui, interactive: bool) -> ComponentWidgetResponse {
        let rect = self.rect();
        let stroke = Stroke::new(STROKE_SIZE, Color32::WHITE);

        let response = with_body(ui, interactive, rect, |res, painter| {
            let display_label = if self.is_input {
                "IN"
            } else {
                "OUT"
            };
            let mut font = FontId::default();
            font.size = 10.0;

            painter.circle_stroke(rect.center(), rect.height()/2.0 - stroke.width, stroke);
            painter.text(rect.center(), Align2::CENTER_CENTER, display_label, font, Color32::WHITE);
        });

        // Pins
        let mut pin_res = ComponentPinResponse::default();
        let (pin, pin_pos) = match self.is_input {
            true => {
                let output = self.component.get("OUT").unwrap();
                let output_pos = Pos2::new(rect.right() - stroke.width / 2.0, rect.center().y);

                (output, output_pos)
            },
            false => {
                let input = self.component.get("IN").unwrap();
                let input_pos = Pos2::new(rect.left() + stroke.width / 2.0, rect.center().y);

                (input, input_pos)
            },
        };

        with_pin(ui, self.component, pin, pin_pos, interactive, &mut pin_res);

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

    fn draw(&self, ui: &mut Ui, interactive: bool) -> ComponentWidgetResponse {
        let rect = self.rect();

        let response = with_body(ui, interactive, rect, |res, painter| {
            let size = 10.0;
            painter.circle_stroke(rect.center(), rect.height()/2.0 - STROKE_SIZE, Stroke::new(STROKE_SIZE, Color32::WHITE));
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
        });

        // Pins
        let mut pin_res = ComponentPinResponse::default();
        let pin_pos = Pos2::new(rect.right() - STROKE_SIZE / 2.0, rect.center().y);
        let output = self.component.get("OUT").unwrap();
        with_pin(ui, self.component, output, pin_pos, interactive, &mut pin_res);

        (pin_res, response)
    }
}

pub(self) fn with_body(ui: &mut Ui, interactive: bool, rect: Rect, f: impl Fn(Response, Painter)) -> Response{
    ui.put(rect, |ui: &mut Ui| {
        let (res, painter) = ui.allocate_painter(
            rect.size(),
            default_senses(interactive)
        );

        (f)(res.clone(), painter);
        res
    })
}
pub(self) fn with_pin(ui: &Ui, component: &Component, pin: &Pin, pos: Pos2, interactive: bool, pin_response: &mut ComponentPinResponse) {
    let painter = ui.painter();

    painter.circle_stroke(
        pos,
        PIN_RADIUS,
        Stroke::new(STROKE_SIZE / 2.0, PIN_COLOR),
    );
    pin_response.pins_position.insert(pin.name.clone(), pos);
    if pin_response.interaction.is_none() {
        pin_response.interaction = get_pin_interaction(ui, component, pin, pos, interactive);
    }
}

pub(self) fn with_inputs(ui: &Ui, rect: Rect, component: &Component, interactive: bool, pin_response: &mut ComponentPinResponse) {
    let mut inputs = component.inputs();
    inputs.sort_by_key(|input| input.name.clone());

    let inputs_size = (STROKE_SIZE *inputs.len() as f32)+(inputs.len()-1) as f32 * INPUT_SPACING;
    for (i, input) in inputs.into_iter().enumerate() {
        let y = rect.top() + (i as f32* INPUT_SPACING) + (rect.size().y/2.0 - inputs_size / 2.0) + STROKE_SIZE;

        let pin_pos = Pos2::new(rect.left() + STROKE_SIZE / 2.0, y);
        with_pin(ui, component, input, pin_pos, interactive, pin_response);
    }
}

pub(self) fn get_pin_interaction(ui: &Ui, component: &Component, pin: &Pin, pin_pos: Pos2, interactive: bool) -> Option<ComponentPinInteraction> {
    if !interactive {
        return None;
    }
    let pin_rect = Rect::from_center_size(pin_pos, Vec2::splat(PIN_RADIUS + STROKE_SIZE / 2.0));
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