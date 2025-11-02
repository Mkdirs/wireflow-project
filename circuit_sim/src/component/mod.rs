use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::time::{Duration, Instant};
use crate::{Circuit, Simulation};

/// A Component receives inputs and gives outputs
///
/// e.g: [And](logic::And)
#[derive(Debug)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub component_type: Box<dyn ComponentType>,
    pub is_dirty: bool,
    pub x: f32,
    pub y: f32,
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
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PinMode {
    Input,
    Output
}

pub trait ComponentType : Any+Debug {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn default_pins(&self) -> Vec<Pin>;
    fn compute(&mut self, pins: &mut HashMap<String, Pin>);
}
impl dyn ComponentType {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Component {
    /// Constructs a new [Component] from the given descriptor [T]
    fn new<T: ComponentType+Default+'static>(id: &str) -> Self
    {
        let comp = Box::new(T::default());
        let pins:HashMap<String, Pin> = HashMap::from_iter(
            comp.default_pins().into_iter()
                .map(|pin| (pin.name.clone(), pin))
                .collect::<Vec<_>>()
        );
        Self {
            id: id.to_string(),
            name: String::new(),
            component_type: comp,
            pins,
            is_dirty: false,
            x: 0.0,
            y: 0.0
        }
    }

    pub fn compute(&mut self) {
        self.component_type.compute(&mut self.pins);
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
            x: 0.0,
            y: 0.0
        }
    }

    pub fn high(&mut self) {
        self.value = (0..self.channels.get()).map(|_| true).collect()
    }

    pub fn low(&mut self) {
        self.value = (0..self.channels.get()).map(|_| false).collect()
    }
}

#[inline]
fn binary_logicgate_pins() -> [Pin;3] {
    [
        Pin::new("A", PinMode::Input, NonZeroUsize::new(1).unwrap()),
        Pin::new("B", PinMode::Input, NonZeroUsize::new(1).unwrap()),
        Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap()),
    ]
}

#[inline]
fn binary_logicgate_compute(pins: &mut HashMap<String,Pin>, f:impl Fn(bool, bool)->bool) {
    pins.get_mut("OUT").unwrap().value = {
        let a = pins.get("A").unwrap();
        let b = pins.get("B").unwrap();
        a.value.iter().zip(b.value.iter()).map(|(&a, &b)| f(a,b)).collect()
    };
}

#[derive(Debug, Default)]
pub struct And;
impl ComponentType for And {
    fn name(&self) -> String {
        "wireflow:and".to_string()
    }

    fn description(&self) -> String {
        "Logic gate and".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {
        binary_logicgate_compute(pins, |a, b| a && b);
    }
}

#[derive(Debug, Default)]
pub struct Or;
impl ComponentType for Or {
    fn name(&self) -> String {
        "wireflow:or".to_string()
    }

    fn description(&self) -> String {
        "Logic gate or".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {
        binary_logicgate_compute(pins, |a, b| a || b);
    }
}

#[derive(Debug, Default)]
pub struct Xor;
impl ComponentType for Xor {
    fn name(&self) -> String {
        "wireflow:xor".to_string()
    }

    fn description(&self) -> String {
        "Logic gate xor".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {
        binary_logicgate_compute(pins, |a, b| a ^ b);
    }
}

#[derive(Debug, Default)]
pub struct Not;
impl ComponentType for Not {
    fn name(&self) -> String {
        "wireflow:not".to_string()
    }

    fn description(&self) -> String {
        "Logic gate not".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        vec![
            Pin::new("IN", PinMode::Input, NonZeroUsize::new(1).unwrap()),
            Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap())
        ]
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {
        pins.get_mut("IN").unwrap().value = {
            pins.get("OUT").unwrap().value
                .iter().map(|b| !b).collect()
        };
    }
}

#[derive(Debug, Default)]
pub struct Nand;
impl ComponentType for Nand {
    fn name(&self) -> String {
        "wireflow:nand".to_string()
    }

    fn description(&self) -> String {
        "Logic gate nand".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {
        binary_logicgate_compute(pins, |a, b| !(a && b));
    }
}

#[derive(Debug, Default)]
pub struct Nor;
impl ComponentType for Nor {
    fn name(&self) -> String {
        "wireflow:nor".to_string()
    }

    fn description(&self) -> String {
        "Logic gate nor".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {
        binary_logicgate_compute(pins, |a, b| !(a || b));
    }
}

#[derive(Debug, Default)]
pub struct Xnor;
impl ComponentType for Xnor {
    fn name(&self) -> String {
        "wireflow:xnor".to_string()
    }

    fn description(&self) -> String {
        "Logic gate xnor".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {
        binary_logicgate_compute(pins, |a, b| !(a ^ b));
    }
}

#[derive(Debug, Default)]
pub struct Clock {
    pub frequency: Duration,
    pub start_time: Option<Instant>,
}

impl Clock {
    pub fn new(freq_hertz: f32) -> Self {
        Self {
            frequency: Duration::from_secs_f32(1.0 / freq_hertz),
            start_time: None
        }
    }
}

impl ComponentType for Clock {
    fn name(&self) -> String {
        format!("wireflow:clock:{0}hz", 1.0 / self.frequency.as_secs_f32())
    }

    fn description(&self) -> String {
        "Clock".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        vec![Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap())]
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {
        if let Some(start_time) = self.start_time {
            if start_time.elapsed() >= self.frequency {
                let value = {
                    pins.get("OUT").unwrap().value
                        .iter().map(|b| !b).collect()
                };
                pins.get_mut("OUT").unwrap().value = value;
                self.start_time = Some(Instant::now());
            }
            return;
        }

        self.start_time = Some(Instant::now());
    }
}

#[derive(Debug, Default)]
pub struct In;
impl ComponentType for In {
    fn name(&self) -> String {
        "wireflow:in".to_string()
    }

    fn description(&self) -> String {
        "Special component that mark an input".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        vec![Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap())]
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {}
}

#[derive(Debug, Default)]
pub struct Out;
impl ComponentType for Out {
    fn name(&self) -> String {
        "wireflow:out".to_string()
    }

    fn description(&self) -> String {
        "Special component that mark a output".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        vec![Pin::new("IN", PinMode::Input, NonZeroUsize::new(1).unwrap())]
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {}
}

#[derive(Debug, Default)]
struct Compound {
    pub circuit: Circuit,
    pub name: String,
    pub description: String,
}
impl ComponentType for Compound {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn default_pins(&self) -> Vec<Pin> {
        self.circuit.graph.node_weights()
            .filter(|comp|
                comp.component_type.as_any().is::<In>()
                || comp.component_type.as_any().is::<Out>()
            )
            .map(|comp| {
                let pin = comp.pins()[0];
                return Pin::new(&comp.name, pin.mode, pin.channels);
            }).collect::<Vec<_>>()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) {
        // Set inner component inputs as inputs
        for comp in self.circuit.graph.node_weights_mut()
            .filter(|comp| comp.component_type.as_any().is::<In>()) {
            comp.pins_mut()[0].value = pins.get(&comp.name).unwrap().value.clone();
        }

        Simulation::step_until_stable(&mut self.circuit);

        // Set outputs as inner component outputs
        for comp in self.circuit.graph.node_weights()
            .filter(|comp| comp.component_type.as_any().is::<Out>()) {
            pins.get_mut(&comp.name).unwrap().value = comp.pins()[0].value.clone();
        }
    }
}