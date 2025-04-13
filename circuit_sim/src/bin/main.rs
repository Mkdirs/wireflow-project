use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use circuit_sim::{Circuit, Simulation};
use circuit_sim::component::{describe_pin, Component, ComponentType, PinMode};


fn main(){
    let simulation = Arc::new(Mutex::new(Simulation::default()));
    {
        let mut simulation = simulation.lock().unwrap();

        simulation.circuit.add(Component::new("clock", ComponentType::CLOCK(
            Instant::now(),
            Duration::from_secs(1)
        )));
        
        simulation.circuit.add(Component::new("not", ComponentType::NOT));
        
        simulation.circuit.wire(("clock", "output0"), ("not", "input0"));
    };
    
    Simulation::run(simulation.clone());
    thread::sleep(Duration::from_secs(1));
    
    loop {
        {
            let simulation = simulation.lock().unwrap();
    
            let not = simulation.circuit.get("not").unwrap();
            let input = not.get("input0").unwrap();
            let output = not.get("output0").unwrap();
            
            println!("!{} = {}", input.value, output.value);
        }
        thread::sleep(Duration::from_secs(1));
    }
    
}