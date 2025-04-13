use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use circuit_sim::{Circuit, Simulation};
use circuit_sim::component::{describe_pin, Component, ComponentType, PinMode};


fn make_nand_component(id: &str) -> Component {
    let mut circuit = Circuit::default();

    circuit.add(Component::new("input0", ComponentType::IO));
    circuit.add(Component::new("input1", ComponentType::IO));
    
    circuit.add(Component::new("and", ComponentType::AND));
    circuit.add(Component::new("not", ComponentType::NOT));

    circuit.add(Component::new("output0", ComponentType::IO));

    circuit.wire(("input0", "output0"), ("and", "input0"));
    circuit.wire(("input1", "output0"), ("and", "input1"));
    circuit.wire(("and", "output0"), ("not", "input0"));
    circuit.wire(("not", "output0"), ("output0", "input0"));

    let pins = vec![
        describe_pin("input0", "A", PinMode::Input),
        describe_pin("input1", "B", PinMode::Input),
        describe_pin("output0", "OUT", PinMode::Output),
    ];

    Component::new(id, ComponentType::COMPOUND(circuit, pins))
}


fn main(){
    let simulation = Arc::new(Mutex::new(Simulation::default()));
    {
        let mut simulation = simulation.lock().unwrap();


        simulation.circuit.add(make_nand_component("nand1"));
        

        // simulation.circuit.add(Component::new("clock", ComponentType::CLOCK(
        //     Instant::now(),
        //     Duration::from_secs(1)
        // )));
        
        //println!("{}", simulation.circuit.wire(("clock", "output0"), ("nand1", "input0")));
        //println!("{}", simulation.circuit.wire(("clock", "output0"), ("nand1", "input1")));
        

        {
            Simulation::step_until_stable(&mut simulation.circuit);
            let nand1 = simulation.circuit.get("nand1").unwrap();
            let input0 = nand1.get("input0").unwrap();
            let input1 = nand1.get("input1").unwrap();
            let output = nand1.get("output0").unwrap();
            println!("NAND({}, {}) = {}", input0.value, input1.value, output.value);
        };

        {
            let nand1 = simulation.circuit.get_mut("nand1").unwrap();
            nand1.set("input0", true);
            nand1.set("input1", false);
        };

        {
            Simulation::step_until_stable(&mut simulation.circuit);
            let nand1 = simulation.circuit.get("nand1").unwrap();
            let input0 = nand1.get("input0").unwrap();
            let input1 = nand1.get("input1").unwrap();
            let output = nand1.get("output0").unwrap();
            println!("NAND({}, {}) = {}", input0.value, input1.value, output.value);
        };

        {
            let nand1 = simulation.circuit.get_mut("nand1").unwrap();
            nand1.set("input0", false);
            nand1.set("input1", true);
        };

        {
            Simulation::step_until_stable(&mut simulation.circuit);
            let nand1 = simulation.circuit.get("nand1").unwrap();
            let input0 = nand1.get("input0").unwrap();
            let input1 = nand1.get("input1").unwrap();
            let output = nand1.get("output0").unwrap();
            println!("NAND({}, {}) = {}", input0.value, input1.value, output.value);
        };

        {
            let nand1 = simulation.circuit.get_mut("nand1").unwrap();
            nand1.set("input0", true);
            nand1.set("input1", true);
        };

        {
            Simulation::step_until_stable(&mut simulation.circuit);
            let nand1 = simulation.circuit.get("nand1").unwrap();
            let input0 = nand1.get("input0").unwrap();
            let input1 = nand1.get("input1").unwrap();
            let output = nand1.get("output0").unwrap();
            println!("NAND({}, {}) = {}", input0.value, input1.value, output.value);
        };
        
    };
    
    Simulation::run(simulation.clone());
    //thread::sleep(Duration::from_secs(1));
    
    // loop {
    //     {
    //         let simulation = simulation.lock().unwrap();
    // 
    //         let nand = simulation.circuit.get("nand1").unwrap();
    //         let input0 = nand.get("input0").unwrap();
    //         let input1 = nand.get("input1").unwrap();
    //         let output = nand.get("output0").unwrap();
    //         
    //         // println!("{:?}", input0);
    //         // println!("{:?}", input1);
    //         // println!("{:?}", output);
    // 
    //         println!("NAND({}, {}) = {}", input0.value, input1.value, output.value);
    //     }
    //     thread::sleep(Duration::from_secs(1));
    // }
    
}