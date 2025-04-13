use crate::component::{describe_pin, Component, ComponentType, Pin, PinDescriptor, PinMode};

#[test]
fn describe_pin_work() {
    let descriptor = describe_pin("input0", "A", PinMode::Input);
    assert_eq!(PinDescriptor{
        id: "input0".to_string(),
        label: "A".to_string(),
        mode: PinMode::Input,
    }, descriptor);
}

#[test]
fn and_gate_should_work() {
    let mut component = Component::new("comp0", ComponentType::AND);
    component.set("input0", false);
    component.set("input1", false);

    component.compute();

    assert_eq!(
        Some(false),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", false);
    component.set("input1", true);

    component.compute();

    assert_eq!(
        Some(false),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", true);
    component.set("input1", false);

    component.compute();

    assert_eq!(
        Some(false),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", true);
    component.set("input1", true);

    component.compute();

    assert_eq!(
        Some(true),
        component.get("output0").and_then(|pin| Some(pin.value))
    );
}

#[test]
fn or_gate_should_work() {
    let mut component = Component::new("comp0", ComponentType::OR);
    component.set("input0", false);
    component.set("input1", false);

    component.compute();

    assert_eq!(
        Some(false),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", false);
    component.set("input1", true);

    component.compute();

    assert_eq!(
        Some(true),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", true);
    component.set("input1", false);

    component.compute();

    assert_eq!(
        Some(true),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", true);
    component.set("input1", true);

    component.compute();

    assert_eq!(
        Some(true),
        component.get("output0").and_then(|pin| Some(pin.value))
    );
}

#[test]
fn xor_gate_should_work() {
    let mut component = Component::new("comp0", ComponentType::XOR);
    component.set("input0", false);
    component.set("input1", false);

    component.compute();

    assert_eq!(
        Some(false),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", false);
    component.set("input1", true);

    component.compute();

    assert_eq!(
        Some(true),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", true);
    component.set("input1", false);

    component.compute();

    assert_eq!(
        Some(true),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", true);
    component.set("input1", true);

    component.compute();

    assert_eq!(
        Some(false),
        component.get("output0").and_then(|pin| Some(pin.value))
    );
}

#[test]
fn not_gate_should_work() {
    let mut component = Component::new("comp0", ComponentType::NOT);
    
    assert_eq!(
        vec![&Pin::new("input0", "IN", PinMode::Input, false)],
        component.inputs()
    );
    
    component.set("input0", false);

    component.compute();

    assert_eq!(
        Some(true),
        component.get("output0").and_then(|pin| Some(pin.value))
    );

    component.set("input0", true);

    component.compute();

    assert_eq!(
        Some(false),
        component.get("output0").and_then(|pin| Some(pin.value))
    );
}