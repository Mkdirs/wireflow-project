use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::{Circuit, Simulation};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PinMode {
    Input,
    Output
}

#[derive(Debug, Clone)]
pub enum ComponentType {
    AND,
    OR,
    XOR,
    NOT,
    CLOCK(Instant, Duration)
}

/// A pin is where data enter/exit a component
/// 
/// It can be an [Input](PinMode::Input) or an [Output](PinMode::Output)
#[derive(Debug, PartialEq, Clone)]
pub struct Pin {
    pub id: String,
    pub label: String,
    pub mode: PinMode,
    pub value: bool,
}

/// A Component receives inputs and gives outputs
/// 
/// e.g: [And](logic::And)
#[derive(Debug, Clone)]
pub struct Component {
    pins: HashMap<String, Pin>,
    pub id: String,
    pub label: String,
    pub node_type: ComponentType,
    pub is_dirty: bool
}

#[derive(Debug, PartialEq, Clone)]
pub struct PinDescriptor{
    pub id: String,
    pub label: String,
    pub mode: PinMode,
}

/// See [PinDescriptor]
pub type PinsDescriptor = Vec<PinDescriptor>;

/// Describes the pin layout of a [component](Component)
pub trait ComponentDescriptor {
    
    fn label(&self) -> String;

    /// Describe your inputs and outputs
    fn describe(&self) -> PinsDescriptor;
}

/// Utilitary function to construct a [PinDescriptor]
pub fn describe_pin(id: &str, label: &str, mode:PinMode) -> PinDescriptor {
    PinDescriptor{
        id: id.to_string(),
        label: label.to_string(),
        mode
    }
}

impl Pin {
    pub fn new(id: &str, label: &str, mode: PinMode, value: bool) -> Pin {
        Pin{
            id: id.to_string(),
            label: label.to_string(),
            mode,
            value
        }
    }
}


impl Component {
    /// Constructs a new [Component] from the given descriptor [T]
    pub fn new(id: &str, node_type: ComponentType) -> Self {
        let pins_descriptor = node_type.describe();
        let pins = HashMap::from_iter(
            pins_descriptor.into_iter()
                .map(|pin_desc| (
                    pin_desc.id.clone(),
                    Pin{
                        id: pin_desc.id,
                        label: pin_desc.label,
                        mode: pin_desc.mode,
                        value: false
                    }
                )));

        Self {
            id: id.to_string(),
            pins,
            label: node_type.label(),
            node_type,
            is_dirty: true
        }
    }
    
    pub fn compute(&mut self) {
        if !self.is_dirty && !matches!(self.node_type, ComponentType::CLOCK(_, _)) {
            return;
        }

        match &mut self.node_type {
            ComponentType::AND => {
                let a = self.get("input0").expect("input0");
                let b = self.get("input1").expect("input1");

                self.set("output0", a.value && b.value);
                self.is_dirty = false;
            },
            ComponentType::OR => {
                let a = self.get("input0").expect("input0");
                let b = self.get("input1").expect("input1");

                self.set("output0", a.value || b.value);
                self.is_dirty = false;
            },
            ComponentType::XOR => {
                let a = self.get("input0").expect("input0");
                let b = self.get("input1").expect("input1");

                self.set("output0", a.value ^ b.value);
                self.is_dirty = false;
            },
            ComponentType::NOT => {
                let input = self.get("input0").expect("input0");

                self.set("output0", !input.value);
                self.is_dirty = false;
            },
            
            ComponentType::CLOCK(instant, duration) => {
                let output = self.pins.get_mut("output0").unwrap();
                if Instant::now().duration_since(*instant) >= *duration {
                    output.value = !output.value;
                    self.is_dirty = true;
                    *instant = Instant::now();
                }else{
                    self.is_dirty = false;
                }
            },
        };
    }

    /// Sets a pin's value
    pub fn set(&mut self, pin_id: &str, value: bool) {
        if let Some(pin) = self.pins.get_mut(pin_id) {
            if pin.value != value {
                pin.value = value;
                self.is_dirty = true;
            }
            
        }
    }

    /// Gets a pin
    pub fn get(&self, pin_id: &str) -> Option<&Pin> {
        self.pins.get(pin_id)
        
    }
    
    pub fn get_mut(&mut self, pin_id: &str) -> Option<&mut Pin> {
        self.pins.get_mut(pin_id)
    }
    
    /// gets all pins
    pub fn pins(&self) -> Vec<&Pin> {
        self.pins.values().collect()
    }
    
    /// Gets all input pins
    pub fn inputs(&self) -> Vec<&Pin> {
        self.pins.values()
            .filter(|pin| pin.mode == PinMode::Input)
            .collect()
    }

    /// Gets all output pins
    pub fn outputs(&self) -> Vec<&Pin> {
        self.pins.values()
            .filter(|pin| pin.mode == PinMode::Output)
            .collect()
    }
}

impl ComponentDescriptor for ComponentType {
    fn label(&self) -> String {
        match self { 
            ComponentType::AND => "AND".to_string(),
            ComponentType::OR => "OR".to_string(),
            ComponentType::XOR => "XOR".to_string(),
            ComponentType::NOT => "NOT".to_string(),
            ComponentType::CLOCK(_, _) => "CLOCK".to_string()
        }
    }

    fn describe(&self) -> PinsDescriptor {
        match self {
            ComponentType::AND | ComponentType::OR | ComponentType::XOR  => vec![
                describe_pin("input0", "A", PinMode::Input),
                describe_pin("input1", "B", PinMode::Input),
                describe_pin("output0", "OUT", PinMode::Output),
            ],
            ComponentType::NOT => vec![
                describe_pin("input0", "IN", PinMode::Input),
                describe_pin("output0", "OUT", PinMode::Output),
            ],
            
            ComponentType::CLOCK(_, _) => vec![describe_pin("output0", "OUT", PinMode::Output)]
        }
    }
}