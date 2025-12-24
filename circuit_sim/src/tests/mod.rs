use crate::component::{AndGate, Component, NandGate, NorGate, NotGate, OrGate, XnorGate, XorGate};

#[test]
fn and_gate_should_work() {
    let mut component = Component::new::<AndGate>("comp0");
    component.set("A", &[false]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[false]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );
}

#[test]
fn or_gate_should_work() {
    let mut component = Component::new::<OrGate>("comp0");
    component.set("A", &[false]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[false]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );
}

#[test]
fn xor_gate_should_work() {
    let mut component = Component::new::<XorGate>("comp0");
    component.set("A", &[false]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[false]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );
}

#[test]
fn not_gate_should_work() {
    let mut component = Component::new::<NotGate>("comp0");
    
    component.set("IN", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("IN", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );
}

#[test]
fn nand_gate_should_work() {
    let mut component = Component::new::<NandGate>("comp0");
    component.set("A", &[false]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[false]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );
}

#[test]
fn nor_gate_should_work() {
    let mut component = Component::new::<NorGate>("comp0");
    component.set("A", &[false]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[false]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );
}

#[test]
fn xnor_gate_should_work() {
    let mut component = Component::new::<XnorGate>("comp0");
    component.set("A", &[false]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[false]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[false]);

    component.compute();

    assert_eq!(
        Some(&vec![false]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );

    component.set("A", &[true]);
    component.set("B", &[true]);

    component.compute();

    assert_eq!(
        Some(&vec![true]),
        component.get("OUT").and_then(|pin| Some(&pin.value))
    );
}