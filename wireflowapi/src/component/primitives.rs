use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::time::{Duration, Instant};
use crate::component::{ComponentType, Pin, PinMode};

#[inline]
pub fn binary_logicgate_pins() -> [Pin;3] {
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
    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        binary_logicgate_compute(pins, |a, b| a && b)
    }
}

#[derive(Debug, Default)]
pub struct OrGate;
impl ComponentType for OrGate {
    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        binary_logicgate_compute(pins, |a, b| a || b)
    }
}

#[derive(Debug, Default)]
pub struct XorGate;
impl ComponentType for XorGate {
    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        binary_logicgate_compute(pins, |a, b| a ^ b)
    }
}

#[derive(Debug, Default)]
pub struct NotGate;
impl ComponentType for NotGate {
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
    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool{
        binary_logicgate_compute(pins, |a, b| !(a && b))
    }
}

#[derive(Debug, Default)]
pub struct NorGate;
impl ComponentType for NorGate {
    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {
        binary_logicgate_compute(pins, |a, b| !(a || b))
    }
}

#[derive(Debug, Default)]
pub struct XnorGate;
impl ComponentType for XnorGate {
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
        Self::new(1.0)
    }
}

impl ComponentType for Clock {
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
    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {false}
}

#[derive(Debug, Default)]
pub struct Out;
impl ComponentType for Out {
    fn compute(&mut self, pins: &mut HashMap<String, Pin>) -> bool {false}
}