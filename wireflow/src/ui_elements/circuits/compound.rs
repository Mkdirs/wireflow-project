use eframe::egui::{Align, Color32, Frame, Layout, Pos2, Rangef, RichText, Sense, Stroke, Ui, UiBuilder, Vec2};
use circuit_sim::component::{Component, Pin};
use crate::ui_elements::circuits::{get_pin_interaction, ComponentPinResponse, ComponentWidget, ComponentWidgetResponse, PIN_LENGTH, STROKE_SIZE};
use crate::ui_elements::circuits::logicgates::LOGIC_GATE_SIZE;

pub(super) struct CompoundWidget<'a>{
    pub component: &'a Component,
    pub pos: Pos2,
}

impl<'a> ComponentWidget for CompoundWidget<'a> {
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
        
        let margin = 5.0;

        let response = ui.put(rect, |ui: &mut Ui| {
            ui.scope_builder(
                UiBuilder::new()
                    .sense(Sense::hover() | Sense::drag())
                    .layout(Layout::left_to_right(Align::Center)),
                |ui| {
                    Frame::default()
                        .inner_margin(margin)
                        .corner_radius(5)
                        .stroke(Stroke::new(STROKE_SIZE, Color32::WHITE))
                        .show(ui, |ui| {
                            let mut inputs = self.component.inputs();
                            let mut outputs = self.component.outputs();
                            inputs.sort_by_key(|input| input.name.clone());
                            outputs.sort_by_key(|output| output.name.clone());


                            ui.vertical(|ui| {
                                for input in inputs {
                                    ui.horizontal(|ui| {
                                        let next_widget_pos = ui.next_widget_position();
                                        let pos = Pos2::new(next_widget_pos.x-margin-STROKE_SIZE-PIN_LENGTH, next_widget_pos.y);
                                        self.draw_pin(ui, input, pos.x..=next_widget_pos.x-margin-STROKE_SIZE, pos, &mut pin_res);
                                    });
                                }
                            });

                            ui.label(
                                RichText::new(self.component.name.clone())
                                    .size(20.0)
                            );

                            ui.vertical(|ui| {
                                for output in outputs {
                                    ui.horizontal(|ui| {
                                        let next_widget_pos = ui.next_widget_position();
                                        let pos = Pos2::new(next_widget_pos.x+PIN_LENGTH, next_widget_pos.y);
                                        self.draw_pin(ui, output, next_widget_pos.x..=pos.x, pos, &mut pin_res);
                                    });
                                }
                            });

                        })
                }).response
        });

        (pin_res, response)
    }
}

impl<'a> CompoundWidget<'a> {
    fn draw_pin(&self, ui: &mut Ui, pin: &Pin, x: impl Into<Rangef>, pin_pos:Pos2, pin_res: &mut ComponentPinResponse) {
        let painter = ui.painter();
        painter.hline(x, pin_pos.y, Stroke::new(STROKE_SIZE, Color32::WHITE));
        ui.label(&pin.name);
        pin_res.pins_position.insert(pin.name.clone(), pin_pos);

        if pin_res.interaction.is_none() {
            pin_res.interaction = get_pin_interaction(ui, self.component, pin, pin_pos);
        }
    }
}