use std::collections::HashMap;
use crate::Circuit;
use crate::component::{ComponentType, Pin};
use crate::component::primitives::{In, Out};

#[derive(Debug, Default)]
pub struct EmbeddedCircuit {
    pub circuit: Circuit,
    pub name: String,
    pub description: String,
}

impl EmbeddedCircuit {
    pub fn pins(&self) -> Vec<Pin> {
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
}
impl ComponentType for EmbeddedCircuit {
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