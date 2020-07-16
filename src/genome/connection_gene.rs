use crate::NodeGene;
use crate::Genome;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConnectionGene {
    in_node: i32,
    out_node: i32,
    weight: f64,

    // Also called "enabled" or "disabled", whether the connection is active
    expressed: bool,
    innovation: i32,
}

impl ConnectionGene {
    pub fn new(in_node: i32, out_node: i32, weight: f64, expressed: bool, innovation: i32) -> Self {
        Self {
            in_node,
            out_node,
            weight,
            expressed,
            innovation,
        }
    }

    pub fn get_in_node(&self) -> i32 {
        self.in_node.to_owned()
    }

    pub fn get_out_node(&self) -> i32 {
        self.out_node.to_owned()
    }

    pub fn get_weight(&self) -> f64 {
        self.weight.to_owned()
    }

    pub fn is_expressed(&self) -> bool {
        self.expressed.to_owned()
    }

    pub fn get_released(&self) -> ConnectionGene {
        ConnectionGene::new(self.in_node, self.out_node, self.weight, self.expressed, self.innovation)
    }
    
    pub fn disable(&mut self) {
        self.expressed = false;
    }

    pub fn get_innovation(&self) -> i32 {
        self.innovation.to_owned()
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }
}
