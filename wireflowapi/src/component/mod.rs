pub mod primitives;
pub mod embedded;

use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

/// A Component receives inputs and gives outputs
///
/// e.g: [And](logic::And)
#[derive(Debug, Clone)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub component_type: Arc<Mutex<dyn ComponentType>>,
    pub is_dirty: bool,
    pins: HashMap<String, Pin>
}

/// A pin is where data enter/exit a component
///
/// It can be an [Input](PinMode::Input) or an [Output](PinMode::Output)
#[derive(Debug, PartialEq, Clone)]
pub struct Pin {
    pub name: String,
    pub mode: PinMode,
    pub channels: NonZeroUsize,
    pub value: Vec<bool>,
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PinMode {
    Input,
    Output
}



pub trait ComponentType : Any+Debug+Send+Sync {
    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool;
}
impl dyn ComponentType {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Component {
    /// Constructs a new [Component] from the given descriptor [T]
    pub fn new(id: &str, name: String, component: Arc<Mutex<dyn ComponentType>>) -> Self
    {
        Self {
            id: id.to_string(),
            name,
            component_type: component,
            pins: HashMap::new(),
            is_dirty: true,
        }
    }

    pub fn compute(&mut self) {
        let mut comp = self.component_type.lock().unwrap();
        self.is_dirty = comp.compute(&mut self.pins);
    }
    
    pub fn set_pins(&mut self, pins: HashMap<String, Pin>) {
        self.pins = pins;
    }

    /// Sets a pin's value
    pub fn set(&mut self, pin_name: &str, value: &[bool]) {
        if let Some(pin) = self.pins.get_mut(pin_name) {
            if pin.value != value {
                pin.value = value.to_vec();
                self.is_dirty = true;
            }

        }
    }

    /// Gets a pin
    pub fn get(&self, pin_name: &str) -> Option<&Pin> {
        self.pins.get(pin_name)

    }

    pub fn get_mut(&mut self, pin_name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(pin_name)
    }

    /// gets all pins
    pub fn pins(&self) -> Vec<&Pin> {
        self.pins.values().collect()
    }

    pub fn pins_mut(&mut self) -> Vec<&mut Pin> {
        self.pins.values_mut().collect()
    }

    /// Gets all input pins
    pub fn inputs(&self) -> Vec<&Pin> {
        self.pins.values()
            .filter(|pin| pin.mode == PinMode::Input)
            .collect()
    }

    pub fn inputs_mut(&mut self) -> Vec<&mut Pin> {
        self.pins.values_mut()
            .filter(|pin| pin.mode == PinMode::Input)
            .collect()
    }

    /// Gets all output pins
    pub fn outputs(&self) -> Vec<&Pin> {
        self.pins.values()
            .filter(|pin| pin.mode == PinMode::Output)
            .collect()
    }

    pub fn outputs_mut(&mut self) -> Vec<&mut Pin> {
        self.pins.values_mut()
            .filter(|pin| pin.mode == PinMode::Output)
            .collect()
    }
}

impl Pin {
    pub fn new(name: &str, mode: PinMode, channels:NonZeroUsize) -> Pin {
        Pin{
            name: name.to_string(),
            mode,
            channels,
            value: (0..channels.get()).map(|_| false).collect(),
        }
    }

    pub fn high(&mut self) {
        self.value = (0..self.channels.get()).map(|_| true).collect()
    }

    pub fn low(&mut self) {
        self.value = (0..self.channels.get()).map(|_| false).collect()
    }
}
