use std::collections::HashMap;
use eframe::egui::{Pos2, Rect, StrokeKind, Ui, Vec2};

#[derive(Debug, Default)]
pub struct Region {
    start: Pos2,
    end: Pos2,
    pub elements: HashMap<String, Rect>,
}

impl Region {
    pub fn from(start: Pos2) -> Self {
        Region {
            start,
            end: start,
            ..Region::default()
        }
    }

    pub fn to(&mut self, end: Pos2) {
        self.end = end;
    }

    pub fn rect(&self) -> Rect {
        Rect::from_points(&[self.start, self.end])
    }

    pub fn translate(&mut self, delta: Vec2) {
        self.start += delta;
        self.end += delta;
    }

    pub fn fit_to_elements(&mut self) {
        let points = self.elements.values()
            .map(|rect| [rect.min, rect.max])
            .flatten()
            .collect::<Vec<_>>();

        let rect = Rect::from_points(&points);

        self.start = rect.min;
        self.end = rect.max;
    }

    pub fn render(&self, ui: &Ui) {
        let rect = self.rect();
        let painter = ui.painter();
        painter.rect_stroke(rect, 0.0, ui.style().visuals.selection.stroke, StrokeKind::Outside);

        for (_, rect) in &self.elements {
            painter.rect_stroke(*rect, 0.0, ui.style().visuals.selection.stroke, StrokeKind::Outside);
        }
    }
}