// use crate::Circuit;
// use crate::component::{AndGate, Component, In, NandGate, NorGate, NotGate, OrGate, XnorGate, XorGate};
//
// #[test]
// fn and_gate_should_work() {
//     let mut component = Component::new::<AndGate>("and");
//     component.set("A", &[false]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[false]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
// }
//
// #[test]
// fn or_gate_should_work() {
//     let mut component = Component::new::<OrGate>("or");
//     component.set("A", &[false]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[false]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
// }
//
// #[test]
// fn xor_gate_should_work() {
//     let mut component = Component::new::<XorGate>("xor");
//     component.set("A", &[false]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[false]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
// }
//
// #[test]
// fn not_gate_should_work() {
//     let mut component = Component::new::<NotGate>("not");
//
//     component.set("IN", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("IN", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
// }
//
// #[test]
// fn nand_gate_should_work() {
//     let mut component = Component::new::<NandGate>("nand");
//     component.set("A", &[false]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[false]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
// }
//
// #[test]
// fn nor_gate_should_work() {
//     let mut component = Component::new::<NorGate>("nor");
//     component.set("A", &[false]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[false]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
// }
//
// #[test]
// fn xnor_gate_should_work() {
//     let mut component = Component::new::<XnorGate>("xnor");
//     component.set("A", &[false]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[false]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[false]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![false]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
//
//     component.set("A", &[true]);
//     component.set("B", &[true]);
//
//     component.compute();
//
//     assert_eq!(
//         Some(&vec![true]),
//         component.get("OUT").and_then(|pin| Some(&pin.value))
//     );
// }
//
// #[test]
// fn step_should_preserve_state() {
//     let mut circuit = Circuit::default();
//     circuit.add("in", Component::new::<In>("in"));
//     circuit.add("or", Component::new::<OrGate>("or"));
//     circuit.add("not", Component::new::<NotGate>("not"));
//
//     circuit.wire(("in", "OUT"), ("or", "A"));
//     circuit.wire(("or", "OUT"), ("not", "IN"));
//     circuit.wire(("not", "OUT"), ("or", "B"));
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("in")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//         .and_then(|comp| comp.get("A"))
//         .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("B"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("IN"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert!(circuit.simulation.to_visit.is_empty());
//     assert!(circuit.simulation.visited.is_empty());
//
//     circuit.step();
//
//     assert_eq!(
//         circuit.components_id.get("or"),
//         circuit.simulation.to_visit.front()
//     );
//
//     assert_eq!(
//         Some(false),
//         circuit.get("in")
//             .and_then(|comp| Some(comp.is_dirty))
//     );
//
//     assert!(circuit.simulation.visited.contains(circuit.components_id.get("in").unwrap()));
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("in")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("A"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("B"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("IN"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     circuit.step();
//
//     assert_eq!(
//         circuit.components_id.get("not"),
//         circuit.simulation.to_visit.front()
//     );
//
//     assert_eq!(
//         Some(false),
//         circuit.get("or")
//             .and_then(|comp| Some(comp.is_dirty))
//     );
//
//     assert!(circuit.simulation.visited.contains(circuit.components_id.get("in").unwrap()));
//     assert!(circuit.simulation.visited.contains(circuit.components_id.get("or").unwrap()));
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("in")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("A"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("B"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("IN"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//
//     circuit.step();
//
//     assert_eq!(
//         None,
//         circuit.simulation.to_visit.front()
//     );
//
//     assert_eq!(
//         Some(false),
//         circuit.get("not")
//             .and_then(|comp| Some(comp.is_dirty))
//     );
//
//     assert_eq!(
//         Some(true),
//         circuit.get("or")
//             .and_then(|comp| Some(comp.is_dirty))
//     );
//
//     assert!(circuit.simulation.visited.contains(circuit.components_id.get("in").unwrap()));
//     assert!(circuit.simulation.visited.contains(circuit.components_id.get("or").unwrap()));
//     assert!(circuit.simulation.visited.contains(circuit.components_id.get("not").unwrap()));
//     assert_eq!(3, circuit.simulation.visited.len());
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("in")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("A"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![true]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("B"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("IN"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![true]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     circuit.step();
//     circuit.step();
//
//     assert_eq!(
//         circuit.components_id.get("not"),
//         circuit.simulation.to_visit.front()
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("or")
//         .and_then(|comp| comp.get("A"))
//         .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![true]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("B"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![true]),
//         circuit.get("or")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![true]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("IN"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![true]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     circuit.step();
//
//     assert_eq!(
//         None,
//         circuit.simulation.to_visit.front()
//     );
//
//     assert_eq!(
//         Some(&vec![true]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("IN"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
//     assert_eq!(
//         Some(&vec![false]),
//         circuit.get("not")
//             .and_then(|comp| comp.get("OUT"))
//             .and_then(|pin| Some(&pin.value))
//     );
//
// }