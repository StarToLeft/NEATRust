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

    layer: i32,
}

impl NodeGene {
    pub fn new(node_type: NodeGeneType, id: i32, layer: i32) -> Self {
        Self {
            node_type: node_type,
            id,

            layer,
        }
    }

    pub fn get_type(&self) -> NodeGeneType {
        self.node_type.to_owned()
    }

    pub fn get_id(&self) -> i32 {
        self.id.to_owned()
    }

    pub fn get_layer(&self) -> i32 {
        self.layer
    }

    pub fn set_layer(&mut self, layer: i32) {
        self.layer = layer;
    }
}
