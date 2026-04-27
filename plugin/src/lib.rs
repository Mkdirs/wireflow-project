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
    
    pub fn register_component<T: ComponentType+Default>(&mut self) -> &mut Self {
        let component = T::default();
        let name = format!("{prefix}:{name}", prefix=self.name, name=component.name());
        self.components.insert(name, Arc::new(Mutex::new(component)));
        self
    }
}

pub const WIREFLOW:LazyCell<Plugin> = LazyCell::new(|| {
    let mut plugin = Plugin::new("wireflow");
    plugin
        .register_component::<AndGate>()
        .register_component::<OrGate>()
        .register_component::<XorGate>()
        .register_component::<NotGate>()
        .register_component::<NandGate>()
        .register_component::<NorGate>()
        .register_component::<XnorGate>()
        .register_component::<Clock>()
        .register_component::<Compound>()
        .register_component::<In>()
        .register_component::<Out>();
    
    plugin
});