use std::collections::HashMap;
use circuit_sim::component::{Component, ComponentDescriptor, ComponentType};

#[derive(Default, Debug)]
pub struct Registries {
    pub components_registry: Registry<String, ComponentType>,
}

pub type Registry<Id, Item> = HashMap<Id, Item>;
