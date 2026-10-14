use eframe::egui::{Pos2, Rect, Ui, Vec2};
use crate::ui_elements::circuits_scene::region::Region;

#[derive(Debug, Default)]
pub struct Selection {
    pub region: Region,
    state: SelectionState,
    // is_complete: bool,
    // selecting: bool,
}

#[derive(Debug, Default, PartialEq)]
enum SelectionState {
    #[default]
    None,
    Started,
    Completed
}

impl Selection {

    pub fn handle(&mut self, ui: &mut Ui, mouse_pos: Pos2, elements: impl IntoIterator<Item=(String, Rect)>) {

        if ui.response().drag_started() && self.state == SelectionState::None {
            self.region = Region::from(mouse_pos);
            self.state = SelectionState::Started;
            return;
        }

        if ui.response().dragged() && self.state == SelectionState::Started {
            self.region.to(mouse_pos);
            return;
        }

        if ui.response().drag_stopped() && self.state == SelectionState::Started {
            for (id, rect) in elements {
                if self.intersects(rect) {
                    self.add(id, rect);
                } else if self.contains(&id) {
                    // This element is no longer inside the selection region
                    // so it needs to be removed
                    self.remove(&id);
                }
            }

            self.complete();
        }
    }

    pub fn show(&mut self, ui: &mut Ui, mouse_pos: Pos2, on_drag: &mut impl FnMut(&Self, Vec2)) {
        if self.state == SelectionState::Started || self.state == SelectionState::Completed {
            self.region.render(ui);
        }

        if self.region.rect().contains(mouse_pos) && self.state == SelectionState::Completed {
            if ui.response().dragged() {
                let delta = ui.response().drag_delta();
                self.region.translate(delta);
                on_drag(self, delta);
                // for (_, infos) in self.components_info
                //     .iter_mut()
                //     .filter(|(id, _)| self.selection.region.elements.contains_key(*id)) {
                //
                //     infos.position += delta;
                // }
            }
        } else if ui.response().is_pointer_button_down_on() && self.state == SelectionState::Completed {
            self.clear();
        }
    }

    pub fn intersects(&self, rect: Rect) -> bool {
        self.region.rect().intersects(rect)
    }

    pub fn add(&mut self, id: String, rect: Rect) {
        self.region.elements.insert(id, rect);
    }

    pub fn select_single(&mut self, id: String, rect: Rect) {
        self.clear();
        self.region = Region::from(rect.min);
        self.region.to(rect.max);
        self.add(id, rect);
        self.complete();
    }

    pub fn select(&mut self, id: String, rect: Rect) {
        if self.contains(&id) {
            self.remove(&id);
            self.complete();
            return;
        }
        self.add(id, rect);
        self.complete();
    }

    pub fn complete(&mut self) {
        self.region.fit_to_elements();

        self.state = if self.is_empty() {
            SelectionState::None
        } else {
            SelectionState::Completed
        };
    }

    pub fn is_complete(&self) -> bool {
        self.state == SelectionState::Completed
    }

    pub fn remove(&mut self, id: &str) {
        self.region.elements.remove(id);
    }

    pub fn clear(&mut self) {
        self.region = Region::default();
        self.state = SelectionState::default();
    }

    pub fn contains(&self, id: &str) -> bool {
        self.region.elements.contains_key(id)
    }

    pub fn is_empty(&self) -> bool {
        self.region.elements.is_empty()
    }

    pub fn is_selecting(&self) -> bool {
        self.state == SelectionState::Started
    }

    pub fn is_single(&self) -> bool {
        self.region.elements.len() == 1
    }
}