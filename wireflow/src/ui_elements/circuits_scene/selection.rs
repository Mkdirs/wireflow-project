use eframe::egui::Rect;
use crate::ui_elements::circuits_scene::region::Region;

#[derive(Debug, Default)]
pub struct Selection {
    pub region: Region,
    is_complete: bool,
    selecting: bool,
}

impl Selection {
    pub fn intersects(&self, rect: Rect) -> bool {
        self.region.rect().intersects(rect)
    }

    pub fn add(&mut self, id: String, rect: Rect) {
        if !self.is_complete || self.region.elements.contains_key(&id) {
            self.region.elements.insert(id, rect);
        }
    }

    pub fn select_single(&mut self, id: String, rect: Rect) {
        self.clear();
        self.region = Region::from(rect.min);
        self.region.to(rect.max);
        self.add(id, rect);
        self.complete();
    }

    pub fn complete(&mut self) {
        if self.region.elements.is_empty() {
            return;
        }

        self.region.fit_to_elements();
        self.is_complete = true;
        self.selecting = false;
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn remove(&mut self, id: &str) {
        self.region.elements.remove(id);
    }

    pub fn begin(&mut self) {
        self.selecting = true;
        self.is_complete = false;
    }

    pub fn clear(&mut self) {
        self.region = Region::default();
        self.is_complete = false;
        self.selecting = false;
    }

    pub fn contains(&self, id: &str) -> bool {
        self.region.elements.contains_key(id)
    }

    pub fn is_empty(&self) -> bool {
        self.region.elements.is_empty()
    }

    pub fn is_selecting(&self) -> bool {
        self.selecting
    }

    pub fn is_single(&self) -> bool {
        self.region.elements.len() == 1
    }
}