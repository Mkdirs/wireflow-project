use std::collections::HashMap;
use std::hash::Hash;
use eframe::egui::{InputState, KeyboardShortcut, Ui};

pub struct Actions<T> {
    action_shortcuts: HashMap<T, KeyboardShortcut>,
}

impl <T> Default for Actions<T> {
    fn default() -> Self {
        Self {
            action_shortcuts: Default::default(),
        }
    }
}

pub trait ActionHandler {
    type Action;
    
    fn handle_action(&mut self, action: Self::Action, ui: &mut Ui);
}

impl<T> Actions<T>
where T: Eq + Hash,
{
    pub fn register(mut self, action: T, shortcut: KeyboardShortcut) -> Self {
        self.action_shortcuts.insert(action, shortcut.into());
        self
    }
    

    pub fn get_shortcut(&self, action: T) -> Option<&KeyboardShortcut> {
        self.action_shortcuts.get(&action)
    }
    
    pub fn listen(&self, input: &mut InputState) -> Option<&T> {
        self.action_shortcuts.iter()
            .find(|(_, shortcut)| input.consume_shortcut(shortcut))
            .map(|(action, _)| action)
    }
    
    pub fn is_action_called(&self, action: T, input: &mut InputState) -> bool {
        self.action_shortcuts.get(&action)
            .map_or(false, |shortcut| input.consume_shortcut(shortcut))
    }
}