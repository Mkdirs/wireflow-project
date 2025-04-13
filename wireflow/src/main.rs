use circuit_sim::component::{Component, ComponentType};

fn main() {
    let mut comp = Component::new("Component", ComponentType::AND);
    for a in [false, true] {
        comp.set("input0", a);
        for b in [false, true] {
            comp.set("input1", b);
            comp.compute();
            
            println!("{a} AND {b} = {}", comp.get("output0").unwrap().value);
        }
    }
}
