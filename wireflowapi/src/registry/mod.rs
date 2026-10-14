use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, LazyLock, Mutex};
use crate::component::{ComponentType, Pin, PinMode};
use crate::component::primitives::{binary_logicgate_pins, AndGate, Clock, NandGate, NorGate, NotGate, OrGate, XnorGate, XorGate, In, Out};

pub type ComponentsRegistry = HashMap<String, ComponentRegistryItem>;

pub static REGISTRIES: LazyLock<Registries> = LazyLock::new(||
    Registries {
        components: ComponentsRegistry::new()
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("and")
                    .with_description("Logic gate and")
                    .with_pins(binary_logicgate_pins().to_vec())
                    .with_factory(|| Arc::new(Mutex::new(AndGate)))
                    .build()
            )
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("or")
                    .with_description("Logic gate or")
                    .with_pins(binary_logicgate_pins().to_vec())
                    .with_factory(|| Arc::new(Mutex::new(OrGate)))
                    .build()
            )
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("xor")
                    .with_description("Logic gate xor")
                    .with_pins(binary_logicgate_pins().to_vec())
                    .with_factory(|| Arc::new(Mutex::new(XorGate)))
                    .build()
            )
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("not")
                    .with_description("Logic gate not")
                    .with_pins(vec![
                        Pin::new("IN", PinMode::Input, NonZeroUsize::new(1).unwrap()),
                        Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap()),
                    ])
                    .with_factory(|| Arc::new(Mutex::new(NotGate)))
                    .build()
            )
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("nand")
                    .with_description("Logic gate nand")
                    .with_pins(binary_logicgate_pins().to_vec())
                    .with_factory(|| Arc::new(Mutex::new(NandGate)))
                    .build()
            )
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("nor")
                    .with_description("Logic gate nor")
                    .with_pins(binary_logicgate_pins().to_vec())
                    .with_factory(|| Arc::new(Mutex::new(NorGate)))
                    .build()
            )
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("xnor")
                    .with_description("Logic gate xnor")
                    .with_pins(binary_logicgate_pins().to_vec())
                    .with_factory(|| Arc::new(Mutex::new(XnorGate)))
                    .build()
            )
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("clock")
                    .with_description("Clock")
                    .with_pin(
                        Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap())
                    )
                    .with_factory(|| Arc::new(Mutex::new(Clock::default())))
                    .build()
            )
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("input")
                    .with_description("Special component marking an input")
                    .with_pin(
                        Pin::new("OUT", PinMode::Output, NonZeroUsize::new(1).unwrap())
                    )
                    .with_factory(|| Arc::new(Mutex::new(In)))
                    .build()
            )
            .with_component(
                ComponentRegistryItemBuilder::default()
                    .with_name("output")
                    .with_description("Special component marking an output")
                    .with_pin(
                        Pin::new("IN", PinMode::Input, NonZeroUsize::new(1).unwrap())
                    )
                    .with_factory(|| Arc::new(Mutex::new(Out)))
                    .build()
            )
    }
);

#[derive(Debug, Default)]
pub struct Registries {
    pub components: ComponentsRegistry
}

trait IComponentRegistry {
    fn with_component(self, item: ComponentRegistryItem) -> Self;
}

impl IComponentRegistry for ComponentsRegistry {
    fn with_component(mut self, item: ComponentRegistryItem) -> Self {
        self.insert(item.name.clone(), item);
        self
    }
}

#[derive(Debug)]
pub struct ComponentRegistryItem {
    pub name: String,
    pub description: String,
    pub pins: Vec<Pin>,
    pub factory: fn() -> Arc<Mutex<dyn ComponentType>>
}

impl Default for ComponentRegistryItem {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            pins: vec![],
            factory: || panic!("No factory function were set")
        }
    }
}


#[derive(Debug, Default)]
pub struct ComponentRegistryItemBuilder {
    item: ComponentRegistryItem
}
impl ComponentRegistryItemBuilder {
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.item.name = name.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.item.description = description.into();
        self
    }

    pub fn with_pin(mut self, pin: Pin) -> Self {
        self.item.pins.push(pin);
        self
    }

    pub fn with_pins(mut self, pins: Vec<Pin>) -> Self {
        self.item.pins = pins;
        self
    }

    pub fn with_factory(mut self, factory: fn() -> Arc<Mutex<dyn ComponentType>>) -> Self {
        self.item.factory = factory;
        self
    }

    pub fn build(self) -> ComponentRegistryItem {
        ComponentRegistryItem {
            name: self.item.name,
            description: self.item.description,
            pins: self.item.pins,
            factory: self.item.factory
        }
    }
}