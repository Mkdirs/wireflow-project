use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;
use std::iter::zip;
use std::num::NonZeroUsize;
use std::ops::{Add, Deref};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use petgraph::graph::DiGraph;
// use circuit_sim::component::ComponentType;
// fn make_nand_component(id: &str) -> Component {
//     let mut circuit = Circuit::default();
//
//     circuit.add(Component::new("input0", ComponentType::IO));
//     circuit.add(Component::new("input1", ComponentType::IO));
//
//     circuit.add(Component::new("and", ComponentType::AND));
//     circuit.add(Component::new("not", ComponentType::NOT));
//
//     circuit.add(Component::new("output0", ComponentType::IO));
//
//     circuit.wire(("input0", "output0"), ("and", "input0"));
//     circuit.wire(("input1", "output0"), ("and", "input1"));
//     circuit.wire(("and", "output0"), ("not", "input0"));
//     circuit.wire(("not", "output0"), ("output0", "input0"));
//
//     let pins = vec![
//         describe_pin("input0", "A", PinMode::Input),
//         describe_pin("input1", "B", PinMode::Input),
//         describe_pin("output0", "OUT", PinMode::Output),
//     ];
//
//     Component::new(id, ComponentType::COMPOUND(circuit, pins))
// }

#[derive(Debug, Clone)]
struct MyTruthTable{}

#[derive(Debug)]
struct MyComp{
    id:String,
    comp_type: Box<dyn MyCompType>,
    table: Option<MyTruthTable>,
    pins: HashMap<String, MyPin>

}
#[derive(Debug, Clone)]
struct MyPin {
    id:String,
    channels: NonZeroUsize,
    value: Vec<bool>
}

impl MyPin {
    fn new(id:&str, channels:NonZeroUsize) -> MyPin {
        MyPin {
            id: id.to_string(),
            channels,
            value: (0..channels.get()).map(|_| false).collect()
        }
    }

    fn high(&mut self) {
        self.value = (0..self.channels.get()).map(|_| true).collect()
    }

    fn low(&mut self) {
        self.value = (0..self.channels.get()).map(|_| false).collect()
    }
}
impl MyComp {
    fn new<T: MyCompType+Default+'static>(id: &str) -> Self
    {
        let comp = Box::new(T::default());
        let pins:HashMap<String, MyPin> = HashMap::from_iter(
            comp.default_pins().into_iter()
                .map(|pin| (pin.id.clone(), pin))
                .collect::<Vec<_>>()
        );
        Self {
            id: id.to_string(),
            comp_type: comp,
            table: None,
            pins
        }
    }

    fn compute(&mut self) {
        self.comp_type.compute(&mut self.pins);
    }
}

trait MyCompType : Debug {
    fn name(&self) -> &str;
    fn default_pins(&self) -> Vec<MyPin>;
    fn compute(&mut self, pins: &mut HashMap<String, MyPin>);
}
#[derive(Default, Debug)]
struct MyAnd {}
impl MyCompType for MyAnd {

    fn name(&self) -> &str {
        "and"
    }

    fn default_pins(&self) -> Vec<MyPin> {
        vec![
            MyPin::new("a", NonZeroUsize::new(1).unwrap()),
            MyPin::new("b", NonZeroUsize::new(1).unwrap()),
            MyPin::new("out", NonZeroUsize::new(1).unwrap()),
        ]
    }


    fn compute(&mut self, pins: &mut HashMap<String, MyPin>) {
        let value = zip(
            &pins.get("a").unwrap().value,
            &pins.get("b").unwrap().value
        )
            .map(|(a,b)| a & b)
            .collect::<Vec<bool>>();

        let out = pins.get_mut("out").unwrap();
        out.value = value;
    }
}

struct MyS {
    graph: DiGraph::<MyComp, String>
}

fn main(){
    let mut g = DiGraph::<Box<MyComp>, String>::new();
    let ind = g.add_node(Box::new(MyComp::new::<MyAnd>("")));

    let node = g.node_weight_mut(ind).unwrap();
    // let a = node.pins.get_mut("a").unwrap();
    // let b = node.pins.get_mut("b").unwrap();

    println!("{:#?}", node.pins);

    node.compute();

    println!("{:#?}", node.pins);

    {
        node.pins.get_mut("a").unwrap().high();
    }

    println!("{:#?}", node.pins);
    node.compute();

    println!("{:#?}", node.pins);

    {
        node.pins.get_mut("b").unwrap().high();
    }

    println!("{:#?}", node.pins);
    node.compute();

    println!("{:#?}", node.pins);
    
}