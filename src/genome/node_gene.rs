#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum NodeGeneType {
    INPUT,
    HIDDEN,
    OUTPUT,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NodeGene {
    node_type: NodeGeneType,
    id: i32,
}

impl NodeGene {
    pub fn new(node_type: NodeGeneType, id: i32) -> Self {
        Self {
            node_type: node_type,
            id,
        }
    }

    pub fn get_type(&self) -> NodeGeneType {
        self.node_type.to_owned()
    }

    pub fn get_id(&self) -> i32 {
        self.id.to_owned()
    }
}
