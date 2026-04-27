use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use circuit_sim::Circuit;
use circuit_sim::component::{ComponentType, Pin, PinMode};

#[inline]
fn binary_logicgate_pins() -> [Pin;3] {
    [
        Pin::new("A", PinMode::Input, NonZeroUsize::new(1).unwrap()),
        Pin::new("B", PinMode::Input, NonZeroUsize::new(1).unwrap()),
        Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap()),
    ]
}

#[inline]
fn binary_logicgate_compute(pins: &mut HashMap<String,Pin>, f:impl Fn(bool, bool)->bool) -> bool {
    pins.get_mut("OUT").unwrap().value = {
        let a = pins.get("A").unwrap();
        let b = pins.get("B").unwrap();
        a.value.iter().zip(b.value.iter()).map(|(&a, &b)| f(a,b)).collect()
    };
    
    false
}

#[derive(Debug, Default)]
pub struct AndGate;
impl ComponentType for AndGate {
    fn name(&self) -> String {
        "and".to_string()
    }

    fn description(&self) -> String {
        "Logic gate and".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        binary_logicgate_compute(pins, |a, b| a && b)
    }
}

#[derive(Debug, Default)]
pub struct OrGate;
impl ComponentType for OrGate {
    fn name(&self) -> String {
        "or".to_string()
    }

    fn description(&self) -> String {
        "Logic gate or".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        binary_logicgate_compute(pins, |a, b| a || b)
    }
}

#[derive(Debug, Default)]
pub struct XorGate;
impl ComponentType for XorGate {
    fn name(&self) -> String {
        "xor".to_string()
    }

    fn description(&self) -> String {
        "Logic gate xor".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        binary_logicgate_compute(pins, |a, b| a ^ b)
    }
}

#[derive(Debug, Default)]
pub struct NotGate;
impl ComponentType for NotGate {
    fn name(&self) -> String {
        "not".to_string()
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

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool{
        pins.get_mut("OUT").unwrap().value = {
            pins.get("IN").unwrap().value
                .iter().map(|b| !b).collect()
        };
        false
    }
}

#[derive(Debug, Default)]
pub struct NandGate;
impl ComponentType for NandGate {
    fn name(&self) -> String {
        "nand".to_string()
    }

    fn description(&self) -> String {
        "Logic gate nand".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool{
        binary_logicgate_compute(pins, |a, b| !(a && b))
    }
}

#[derive(Debug, Default)]
pub struct NorGate;
impl ComponentType for NorGate {
    fn name(&self) -> String {
        "nor".to_string()
    }

    fn description(&self) -> String {
        "Logic gate nor".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        binary_logicgate_compute(pins, |a, b| !(a || b))
    }
}

#[derive(Debug, Default)]
pub struct XnorGate;
impl ComponentType for XnorGate {
    fn name(&self) -> String {
        "xnor".to_string()
    }

    fn description(&self) -> String {
        "Logic gate xnor".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        binary_logicgate_pins().to_vec()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        binary_logicgate_compute(pins, |a, b| !(a ^ b))
    }
}

#[derive(Debug)]
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

impl Default for Clock {
    fn default() -> Self {
        Self::new(30.0)
    }
}

impl ComponentType for Clock {
    fn name(&self) -> String {
        format!("clock:{0:#.2}hz", 1.0 / self.frequency.as_secs_f32())
    }

    fn description(&self) -> String {
        "Clock".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        vec![Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap())]
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        if let Some(start_time) = self.start_time {
            if start_time.elapsed() >= self.frequency {
                let value = {
                    pins.get("OUT").unwrap().value
                        .iter().map(|b| !b).collect()
                };
                pins.get_mut("OUT").unwrap().value = value;
                self.start_time = Some(Instant::now());
            }
            return true;
        }

        self.start_time = Some(Instant::now());
        true
    }
}

#[derive(Debug, Default)]
pub struct In;
impl ComponentType for In {
    fn name(&self) -> String {
        "in".to_string()
    }

    fn description(&self) -> String {
        "Special component that mark an input".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        vec![Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap())]
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {false}
}

#[derive(Debug, Default)]
pub struct Out;
impl ComponentType for Out {
    fn name(&self) -> String {
        "out".to_string()
    }

    fn description(&self) -> String {
        "Special component that mark a output".to_string()
    }

    fn default_pins(&self) -> Vec<Pin> {
        vec![Pin::new("IN", PinMode::Input, NonZeroUsize::new(1).unwrap())]
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {false}
}

#[derive(Debug, Default)]
pub struct Compound {
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
            .filter(|comp| {
                let component_type = comp.component_type.lock().unwrap();
                component_type.as_any().is::<In>()
                    || component_type.as_any().is::<Out>()
            }
            )
            .map(|comp| {
                let pin = comp.pins()[0];
                return Pin::new(&comp.name, pin.mode, pin.channels);
            }).collect::<Vec<_>>()
    }

    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        // Set inner component inputs as inputs
        for comp in self.circuit.graph.node_weights_mut()
            .filter(|comp| {
                let component_type = comp.component_type.lock().unwrap();
                component_type.as_any().is::<In>()
            }) {
            comp.pins_mut()[0].value = pins.get(&comp.name).unwrap().value.clone();
        }

        self.circuit.leap();
        // Simulation::step_until_stable(&mut self.circuit);

        // Set outputs as inner component outputs
        for comp in self.circuit.graph.node_weights()
            .filter(|comp| {
                let component_type = comp.component_type.lock().unwrap();
                component_type.as_any().is::<Out>()
            }) {
            pins.get_mut(&comp.name).unwrap().value = comp.pins()[0].value.clone();
        }
        
        false
    }
}