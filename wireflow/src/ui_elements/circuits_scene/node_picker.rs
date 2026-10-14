use std::collections::HashMap;
use eframe::egui::{Align2, Color32, CursorIcon, Frame, Grid, Id, InnerResponse, Key, Label, Margin, Popup, PopupAnchor, Pos2, Rect, RectAlign, Response, ScrollArea, Sense, Stroke, StrokeKind, Ui, Vec2};
use wireflowapi::component::Component;
use wireflowapi::registry::{ComponentRegistryItem, REGISTRIES};
use crate::ui_elements::{circuits};

const NUM_COLS:usize = 4;
const MARGIN:f32 = 50.0;
const SPACING:f32 = 10.0;

#[derive(Debug, Default)]
pub struct NodePicker {
    id: String,
    open: bool,
    search: String,
    position: Pos2,
    just_requested_focus: bool,
}

impl NodePicker {

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn open(&self) -> bool {
        self.open
    }

    pub fn toggle(&mut self, pos: impl Into<Pos2>) {
        self.open = !self.open;
        if self.open {
            self.position = pos.into();
            return;
        }

        self.close();
    }

    fn close(&mut self) {
        self.open = false;
        self.search = String::new();
        self.position = Pos2::default();
        self.just_requested_focus = false;
    }

    pub fn show(&mut self, ui: &mut Ui) -> Option<&ComponentRegistryItem> {
        let res = Popup::new(Id::from(self.id.clone()), ui.ctx().clone(), PopupAnchor::Position(self.position), ui.layer_id())
            .align(RectAlign::from_align2(Align2::CENTER_CENTER))
            .sense(Sense::DRAG)
            .open(self.open)
            .show(|ui| {
                let res = ui.vertical_centered(|ui|
                    ui.text_edit_singleline(&mut self.search)
                ).inner;
                let search = self.search.clone();

                if !self.just_requested_focus {
                    res.request_focus();
                    self.just_requested_focus = true;
                }

                if ui.input(|input| input.key_pressed(Key::Escape)) {
                    self.close();
                }

                let mut components = (*REGISTRIES).components
                    .iter()
                    .filter(|(id, _)| id.contains(&search))
                    .collect::<Vec<_>>();

                components.sort_by_key(|(name, _)| *name);

                ui.add_space(25.0);
                ScrollArea::vertical()
                    .min_scrolled_height(300.0)
                    .show(ui, |ui| {
                        Grid::new("Components grid")
                            .spacing(Vec2::splat(SPACING))
                            .striped(true)
                            .min_col_width(MARGIN)
                            .num_columns(NUM_COLS)
                            .show(ui, |ui| {
                                if !components.is_empty() {
                                    for cols in components.chunks(NUM_COLS) {
                                        for (name, entry) in cols {
                                            let comp_id = format!("{}:{}", self.id, name);
                                            let mut frame = Frame::default()
                                                .inner_margin(MARGIN)
                                                .begin(ui);

                                            {
                                                let mut comp = Component::new(
                                                    &comp_id,
                                                    name.to_string(),
                                                    (entry.factory)()
                                                );
                                                comp.set_pins(HashMap::from_iter(
                                                    entry.pins.iter()
                                                        .map(|pin| (pin.name.clone(), pin.clone()))
                                                ));

                                                frame.content_ui.vertical(|ui| {
                                                    let pos = ui.next_widget_position();
                                                    circuits::draw(
                                                        ui,
                                                        &comp,
                                                        pos,
                                                        false
                                                    );
                                                    ui.add(
                                                        Label::new(comp.name)
                                                            .selectable(false)
                                                    );

                                                });
                                            }

                                            let response = frame.content_ui.interact(
                                                frame.allocate_space(ui).rect,
                                                Id::new(comp_id),
                                                Sense::HOVER | Sense::CLICK
                                            );

                                            let style = ui.ctx().style().clone();

                                            if response.clicked() {
                                                self.close();
                                                return Some(*entry);
                                            }

                                            if response.hovered() {
                                                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                                                frame.frame.stroke = style.interact(&response).bg_stroke;
                                            }

                                            frame.paint(ui);

                                        }
                                        ui.end_row();
                                    }
                                } else {
                                    ui.label(":( no components found");
                                    ui.end_row();
                                }

                                None

                            }).inner
                    }).inner
            });

        if let Some(res) = &res {
            if res.response.dragged() {
                self.position += res.response.drag_delta();
            }
        }

        res?.inner
    }
}