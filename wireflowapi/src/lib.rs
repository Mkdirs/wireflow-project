use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::prelude::*;
use crate::component::{Component, PinMode};

pub mod component;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum PlayState {
    Running,
    Paused,
    #[default]
    Stopped,
}

#[derive(Debug, Default, Clone)]
pub struct SimulationState {
    to_visit: VecDeque<NodeIndex>,
    visited: HashSet<NodeIndex>,
    pub play_state: PlayState,

}

#[derive(Debug, Default, Clone)]
pub struct Circuit {
    pub graph: StableDiGraph<Component, (String, String)>,
    components_id: HashMap<String, NodeIndex>,
    edges_id: HashMap<String, EdgeIndex>,
    pub simulation: SimulationState
}

impl Circuit {
    pub fn add(&mut self, id:&str, component: Component) {
        let node_id = self.graph.add_node(component);
        self.components_id.insert(id.to_string(), node_id);
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
        let res = self.graph.remove_node(node_id);

        // handle invalidated node indices and node edges
        if self.graph.node_count() > 0 {
            let last_index = self.graph.node_indices().last().unwrap();
            self.components_id.insert(self.graph.node_weight(last_index).unwrap().id.clone(), last_index);
            
        }

        res
    }

    pub fn play_state(&self) -> &PlayState {
        &self.simulation.play_state
    }

    /// Steps until stopped
    ///
    /// This will block the thread until the circuit is stopped
    pub fn run(&mut self) {
        if self.simulation.play_state != PlayState::Stopped {
            return;
        }
        self.simulation.play_state = PlayState::Running;

        while self.simulation.play_state != PlayState::Stopped {
            if let PlayState::Paused = self.simulation.play_state {
                continue;
            }

            self.step();
        }
    }

    /// Computes all remaining components once
    pub fn leap(&mut self) {
        if self.simulation.play_state != PlayState::Stopped {
            return;
        }

        self.simulation.play_state = PlayState::Running;

        if self.simulation.to_visit.is_empty() {
            self.simulation.to_visit.extend(self.graph.externals(Incoming));
            self.simulation.visited.clear();
        }

        while !self.simulation.to_visit.is_empty() {
            self.step();
        }

        self.stop();
    }

    pub fn pause(&mut self) {
        if let PlayState::Running = self.simulation.play_state {
            self.simulation.play_state = PlayState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if let PlayState::Paused = self.simulation.play_state {
            self.simulation.play_state = PlayState::Running;
        }
    }

    pub fn stop(&mut self) {
        if self.simulation.play_state == PlayState::Running
            || self.simulation.play_state == PlayState::Paused
        {
            self.simulation.to_visit.clear();
            self.simulation.visited.clear();
            self.simulation.play_state = PlayState::Stopped;
        }
    }

    /// Compute the current component and set the inputs of the children
    pub fn step(&mut self) {
        if self.simulation.to_visit.is_empty() {
            self.simulation.to_visit.extend(self.graph.externals(Incoming));
            self.simulation.visited.clear();
        }

        if self.simulation.to_visit.is_empty() {
            return;
        }

        let node_index = self.simulation.to_visit.pop_front()
            .expect("to_visit should not be empty");
        let node = self.graph.node_weight_mut(node_index).unwrap();

        if node.is_dirty {
            node.compute();
        }

        let edges = {
            self.graph.edges_directed(node_index, Outgoing)
                .map(|edge| {
                    (
                        edge.target(),
                        edge.weight().1.clone(),
                        self.graph.node_weight(edge.source())
                            .unwrap()
                            .get(&edge.weight().0)
                            .unwrap().value.clone()
                    )
                })
                .collect::<Vec<_>>()
        };

        for (target_id, target_pin, inputs) in edges {
            let child = self.graph.node_weight_mut(target_id).unwrap();

            child.set(&target_pin, &inputs);

            if !self.simulation.to_visit.contains(&target_id) && !self.simulation.visited.contains(&target_id) {
                self.simulation.to_visit.push_back(target_id);
            }
        }

        self.simulation.visited.insert(node_index);
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

        if source_pin.mode == target_pin.mode {
            return false;
        }

        // if we want to wire an input to an output
        // reverse the action because only output --> input wiring is allowed
        if source_pin.mode == PinMode::Input && target_pin.mode == PinMode::Output {
            return self.wire(target, source);
        }

        if self.is_input_wired(target.0, target.1) {
            return false;
        }

        let wire_id = format!("{}.{}_{}.{}", source.0, source.1, target.0, target.1);
        let edge_id = self.graph.add_edge(*source_node_id, *target_node_id, (source.1.to_string(), target.1.to_string()));
        self.edges_id.insert(wire_id, edge_id);
        true
    }

    pub fn is_pin_wired(&self, comp_id: &str, pin_id: &str) -> bool {
        self.edges_id.keys()
            .map(|edge_id| {
                let mut split = edge_id.split('_');
                let mut source = split.next()
                    .unwrap()
                    .split('.');
                let mut target = split.next()
                    .unwrap()
                    .split('.');

                (
                    (source.next().unwrap(), source.next().unwrap()),
                    (target.next().unwrap(), target.next().unwrap()),
                )
            })
            .any(|(source, target)|
                (source.0 == comp_id && source.1 == pin_id) ||
                    (target.0 == comp_id && target.1 == pin_id)
            )
    }

    #[inline]
    pub fn is_input_wired(&self, comp_id: &str, pin_id: &str) -> bool {
        self.edges_id.keys()
            .map(|edge_id| {
                let mut split = edge_id.split('_');
                let mut source = split.next()
                    .unwrap()
                    .split('.');
                let mut target = split.next()
                    .unwrap()
                    .split('.');

                (
                    (source.next().unwrap(), source.next().unwrap()),
                    (target.next().unwrap(), target.next().unwrap()),
                )
            })
            .any(|(_, target)| target.0 == comp_id && target.1 == pin_id)
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
    /// comp_pin: (comp_id, pin_id)
    pub fn unwire_all(&mut self, comp_pin: (&str, &str)) {
        let node_id = self.components_id.get(comp_pin.0).unwrap();
        let pin = self.graph.node_weight(*node_id).unwrap().get(comp_pin.1).unwrap();
        let wires = if pin.mode == PinMode::Input{
            self.graph.edges_directed(*node_id, Incoming)
                .map(|edge| {
                    let (source_pin, _) = edge.weight();
                    let source_comp = &self.graph[edge.source()].id;
                    
                    format!("{}.{}_{}.{}", source_comp, source_pin, comp_pin.0, comp_pin.1)
                }).collect::<Vec<_>>()
        }else{
            self.graph.edges_directed(*node_id, Outgoing)
                .map(|edge| {
                    let (_, target_pin) = edge.weight();
                    let target_comp = &self.graph[edge.target()].id;

                    format!("{}.{}_{}.{}", comp_pin.0, comp_pin.1, target_comp, target_pin)
                }).collect::<Vec<_>>()
        };
        
        for wire in wires {
            let edge_id = self.edges_id.remove(&wire).unwrap();
            self.graph.remove_edge(edge_id);
        }
    }
}


#[cfg(test)]
mod tests;
pub mod registry;