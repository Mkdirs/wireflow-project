pub mod components;

use std::cell::LazyCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use circuit_sim::component::ComponentType;
use crate::components::{AndGate, Clock, Compound, In, NandGate, NorGate, NotGate, OrGate, Out, XnorGate, XorGate};

#[derive(Default, Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub components: HashMap<String, Arc<Mutex<dyn ComponentType>>>
}

impl Plugin {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            components: HashMap::new()
        }
    }
    
    pub fn register_component<T: ComponentType+Default>(&mut self, name: &str) -> &mut Self {
        let component = T::default();
        let name = format!("{prefix}:{name}", prefix=self.name);
        self.components.insert(name, Arc::new(Mutex::new(component)));
        self
    }
}

pub const WIREFLOW:LazyCell<Plugin> = LazyCell::new(|| {
    let mut plugin = Plugin::new("wireflow");
    plugin
        .register_component::<AndGate>("and")
        .register_component::<OrGate>("or")
        .register_component::<XorGate>("xor")
        .register_component::<NotGate>("not")
        .register_component::<NandGate>("nand")
        .register_component::<NorGate>("nor")
        .register_component::<XnorGate>("xnor")
        .register_component::<Clock>("clock")
        .register_component::<Compound>("compound")
        .register_component::<In>("in")
        .register_component::<Out>("out");
    
    plugin
});