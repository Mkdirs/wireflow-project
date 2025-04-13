use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use petgraph::prelude::*;
use petgraph::visit::Topo;
use crate::component::{Component, ComponentType, PinMode};

pub mod component;

#[derive(Debug, Default, Clone)]
pub struct Circuit {
    graph: DiGraph<Component, (String, String)>,
    components_id: HashMap<String, NodeIndex>,
    edges_id: HashMap<String, EdgeIndex>,
}

#[derive(Debug, Default)]
pub struct Simulation{
    pub circuit: Circuit,
    pub running: bool
}

impl Circuit {
    pub fn add(&mut self, component: Component) {
        let component_id = component.id.clone();
        let node_id = self.graph.add_node(component);
        self.components_id.insert(component_id, node_id);
    }
    
    pub fn get(&self, id: &str) -> Option<&Component> {
        let node_id = self.components_id.get(id)?;
        self.graph.node_weight(*node_id)
    }
    
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Component> {
        let node_id = self.components_id.get(id)?;
        self.graph.node_weight_mut(*node_id)
    }
    
    pub fn remove(&mut self, id: &str) -> Option<Component> {
        let node_id = self.components_id.remove(id)?;
        self.graph.remove_node(node_id)
    }
    
    /// Connect two component
    /// 
    /// source: (source_id, source_pin_id)
    /// 
    /// target: (target_id, target_pin_id)
    pub fn wire(&mut self, source: (&str, &str), target: (&str, &str)) -> bool {
        let source_node_id = self.components_id.get(source.0).unwrap();
        let target_node_id = self.components_id.get(target.0).unwrap();

        let source_pin = &self.graph[*source_node_id].get(source.1).unwrap();
        let target_pin = &self.graph[*target_node_id].get(target.1).unwrap();

        if (source_pin.mode == target_pin.mode) || !(source_pin.mode == PinMode::Output && target_pin.mode == PinMode::Input) {
            return false;
        }

        let wire_id = format!("{}.{}_{}.{}", source.0, source.1, target.0, target.1);
        let edge_id = self.graph.update_edge(*source_node_id, *target_node_id, (source.1.to_string(), target.1.to_string()));
        self.edges_id.insert(wire_id, edge_id);
        true
    }

    /// Disconnect two component
    ///
    /// source: (source_id, source_pin_id)
    ///
    /// target: (target_id, target_pin_id)
    pub fn unwire(&mut self, source: (&str, &str), target: (&str, &str)) -> Option<(String, String)> {
        let wire_id = format!("{}.{}_{}.{}", source.0, source.1, target.0, target.1);
        let edge_id = self.edges_id.remove(&wire_id)?;
        self.graph.remove_edge(edge_id)
    }

    /// Remove all wires associated with the pin
    ///
    /// source: (source_id, source_pin_id)
    pub fn unwire_all(&mut self, source: (&str, &str)) {
        let node_id = self.components_id.get(source.0).unwrap();
        let pin = self.graph.node_weight(*node_id).unwrap().get(source.1).unwrap();
        let wires = if pin.mode == PinMode::Input{
            self.graph.edges_directed(*node_id, Incoming)
                .map(|edge| {
                    let (edge_comp, edge_pin) = edge.weight();
                    
                    format!("{}.{}_{}.{}", edge_comp, edge_pin, source.0, source.1)
                }).collect::<Vec<_>>()
        }else{
            self.graph.edges_directed(*node_id, Outgoing)
                .map(|edge| {
                    let (edge_comp, edge_pin) = edge.weight();

                    format!("{}.{}_{}.{}", source.0, source.1, edge_comp, edge_pin)
                }).collect::<Vec<_>>()
        };
        
        for wire in wires {
            let edge_id = self.edges_id.remove(&wire).unwrap();
            self.graph.remove_edge(edge_id);
        }
    }
}

impl Simulation {

    pub fn run(simulation: Arc<Mutex<Simulation>>) -> Option<JoinHandle<()>> {
        let mut sim = simulation.lock().unwrap();
        if sim.running {
            return None;
        }
        
        sim.running = true;
        drop(sim);
        
        Some(thread::spawn(move || {
            loop {
                {
                    let mut simulation = simulation.lock().unwrap();

                    if !simulation.running {
                        continue;
                    }

                    Self::step(&mut simulation.circuit);
                };
            }
        }))
    }

    /// Returns `true` if the circuit has any dirty component
    pub fn step(circuit: &mut Circuit) -> bool {
        let mut topo = Topo::new(&circuit.graph);

        while let Some(node_index) = topo.next(&circuit.graph) {
            let execute = {
                let node = &circuit.graph[node_index];
                node.is_dirty || matches!(node.node_type, ComponentType::CLOCK(_, _))
            };

            if !execute {
                continue;
            }

            let output_before = {
                circuit.graph[node_index].pins().iter()
                    .filter(|pin| matches!(pin.mode, PinMode::Output))
                    .map(|pin| (pin.id.clone(), pin.value))
                    .collect::<HashMap<_, _>>()
            };

            circuit.graph.node_weight_mut(node_index).unwrap().compute();

            let updates = circuit.graph.edges(node_index)
                .map(|edge| {
                    let target_index = edge.target();
                    let (source_pin_id, target_pin_id) = edge.weight();

                    let source_node = &circuit.graph[node_index];

                    let new_value = source_node.get(source_pin_id)
                        .map(|pin| pin.value)
                        .unwrap_or(false);

                    let old_value = output_before.get(source_pin_id).copied().unwrap_or(false);

                    (target_index, target_pin_id.clone(), new_value, old_value)
                })
                .collect::<Vec<_>>();

            for (target_index, target_pin_id, new_value, old_value) in updates {
                if new_value != old_value {
                    let target_node = circuit.graph.node_weight_mut(target_index).unwrap();
                    target_node.set(&target_pin_id, new_value);
                }
            }
        }

        circuit.graph.node_weights().any(|node| node.is_dirty)
    }

    pub fn step_until_stable(circuit: &mut Circuit) {
        loop {
            if !Simulation::step(circuit) {break;}
        }
    }

    pub async fn step_async(circuit: &mut Circuit) {
        Self::step(circuit);
    }
    
    pub async fn step_until_stable_async(circuit: &mut Circuit) {
        Self::step_until_stable(circuit);
    }

    // pub fn run_clocks(simulation: Arc<Mutex<Simulation>>) -> JoinHandle<()> {
    //     thread::spawn(move || {
    //         loop {
    //             {
    //                 let mut simulation = simulation.lock().unwrap();
    //                 Self::handle_clocks(&mut simulation.circuit);
    //             };
    //         }
    //     })
    // }
    // 
    // fn handle_clocks(circuit: &mut Circuit) {
    //     let mut topo = Topo::new(&*circuit);
    // 
    //     while let Some(node_index) = topo.next(&*circuit) {
    //         let component = circuit.node_weight_mut(node_index).unwrap();
    //         match &mut component.node_type {
    //             ComponentType::CLOCK(_, _) => {
    //                 component.compute();
    //             },
    // 
    //             ComponentType::COMPOUND(circuit, _) => {
    //                 Self::handle_clocks(circuit);
    //             }
    // 
    //             _ => continue
    //         }
    //     }
    // }
}


#[cfg(test)]
mod tests;