use eframe::egui::{Atom, Atoms, Button, CentralPanel, Color32, Frame, Margin, Rect, Response, ScrollArea, Sense, Stroke, StrokeKind, TopBottomPanel, Ui, Vec2, Widget};
use eframe::egui::scroll_area::State;
use crate::ui_elements::UiElement;

#[derive(Default, Debug)]
pub struct Tab<State> {
    pub state: State,
    pub title: String,
}


#[derive(Debug, Default)]
pub struct TabView<T> {
    pub id: String,
    tabs: Vec<Tab<T>>,
    current_tab: usize,
    to_remove: Option<usize>
}


impl <T> TabView<T> {
    pub fn add(&mut self, title: impl Into<String>, state: T) {
        self.tabs.push(Tab{ state, title: title.into() });
    }
}

impl <T,V> UiElement for TabView<T>
where
    T: UiElement<State=V>+Default,
{
    type State = V;

    fn show(&mut self, ui: &mut Ui, args: Self::State) -> Self::State {
        if let Some(to_remove) = self.to_remove {
            self.tabs.remove(to_remove);
            if to_remove <= self.current_tab && self.current_tab > 0 {
                self.current_tab -= 1;
            }

            self.to_remove = None;
        }

        TopBottomPanel::top(format!("TabView {}", self.id))
            .frame(
                Frame::default()
                    .inner_margin(5)
                    .fill(ui.ctx().style().visuals.extreme_bg_color)
            )
            .show_inside(ui, |ui| {
                ScrollArea::horizontal()
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for (indx, tab) in self.tabs.iter().enumerate() {
                                let button = Button::new(&tab.title)
                                    .min_size(Vec2::new(200.0, 50.0))
                                    .selected(indx == self.current_tab);

                                if ui.add(button).clicked() {
                                    self.current_tab = indx;
                                }

                                let delete_button = Button::new("x")
                                    .small()
                                    .corner_radius(5);

                                let size = Vec2::splat(10.0);

                                let mut next_pos = ui.next_widget_position();
                                next_pos.x = next_pos.x - ui.spacing().item_spacing.x - size.x;
                                let rect = Rect::from_center_size(next_pos, size);


                                if ui.place(rect, delete_button).clicked() {
                                    self.to_remove = Some(indx);
                                }
                            }

                            if ui.button("+").clicked() {
                                self.add("Untitled", T::default());
                            }
                        })
                    })
            });

        CentralPanel::default()
            .show_inside(ui, |ui| {
                for (indx, tab) in self.tabs.iter_mut().enumerate() {
                    if indx == self.current_tab {
                        return tab.state.show(ui, args);
                    }
                }

                args
            }).inner
    }
}