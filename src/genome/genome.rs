use std::cmp;
use std::collections::hash_map::Keys;
use std::collections::HashMap;

use rand::Rng;
use rand_distr::{Distribution, Normal};

pub mod connection_gene;

pub mod node_gene;

use connection_gene::ConnectionGene;
use node_gene::NodeGene;
use node_gene::NodeGeneType;

use crate::Counter;

#[derive(Debug, Clone, PartialEq)]
pub struct Genome {
    pub connections: HashMap<i32, ConnectionGene>,
    pub nodes: HashMap<i32, NodeGene>,
}

impl Genome {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    /// # get_connection_genes
    /// Get Genome's connection genes
    pub fn get_connection_genes(&self) -> HashMap<i32, ConnectionGene> {
        self.connections.to_owned()
    }

    /// # get_node_genes
    /// Get Genome's node genes
    pub fn get_node_genes(&self) -> HashMap<i32, NodeGene> {
        self.nodes.to_owned()
    }

    /// # mutation
    /// Mutates the weights
    pub fn mutation(&mut self, probability_perturbing: f32) {
        let mut rng = rand::thread_rng();

        for con in self.connections.values_mut() {
            let rnd_float: f32 = rng.gen_range(0.0, 1.0);

            // Uniformly perturbing weights
            if rnd_float < probability_perturbing {
                let normal = Normal::new(0.0, 1.0).unwrap();
                let v = normal.sample(&mut rand::thread_rng());

                // Nudge the weight a random amount of a normal distribution with peak=0.0 and deviation=1
                con.set_weight(con.get_weight() * v);
            } else {
                // Assign a weight between -2.0 and 2.0
                let rnd_float: f64 = rng.gen_range(-2.0, 2.0);
                con.set_weight(rnd_float);
            }
        }
    }

    /// # add_node_gene
    /// Adds a new node gene to the Genome
    pub fn add_node_gene(&mut self, gene: NodeGene) {
        self.nodes.insert(gene.get_id(), gene);
    }

    /// # add_connection_gene
    /// Adds a new connection gene to the Genome
    pub fn add_connection_gene(&mut self, gene: ConnectionGene) {
        self.connections.insert(gene.get_innovation(), gene);
    }

    /// # add_connection_mutation
    /// Mutates connections for genome to the Genome
    pub fn add_connection_mutation(&mut self, innovation: &mut Counter, max_attempts: i32) {
        let mut tries: i32 = 0;
        let mut success: bool = false;

        let mut rng = rand::thread_rng();

        while tries < max_attempts && !success {
            tries += 1;

            let node_inovation_numbers: Vec<i32> = Genome::keys_to_vec(self.nodes.keys());
            let key_node1 = node_inovation_numbers[rng.gen_range(0, node_inovation_numbers.len())];
            let key_node2 = node_inovation_numbers[rng.gen_range(0, node_inovation_numbers.len())];

            let mut node1 = self.nodes.get(&key_node1).unwrap();
            let mut node2 = self.nodes.get(&key_node2).unwrap();
            let weight: f64 = rng.gen_range(-1.0, 1.0);

            // Check if the nodes should be reversed, in case, do it
            let reversed = if node1.get_type() == NodeGeneType::HIDDEN
                && node2.get_type() == NodeGeneType::INPUT
            {
                true
            } else if node1.get_type() == NodeGeneType::OUTPUT
                && node2.get_type() == NodeGeneType::HIDDEN
            {
                true
            } else if node1.get_type() == NodeGeneType::OUTPUT
                && node2.get_type() == NodeGeneType::INPUT
            {
                true
            } else {
                false
            };
            let tmp: NodeGene;
            if reversed {
                tmp = *node1;
                node1 = node2;
                node2 = &tmp;
            }

            // Check if the connection is impossible
            let mut connection_impossible = if node1.get_type() == NodeGeneType::INPUT
                && node2.get_type() == NodeGeneType::INPUT
            {
                true
            } else if node1.get_type() == NodeGeneType::OUTPUT
                && node2.get_type() == NodeGeneType::OUTPUT
            {
                true
            } else if node1 == node2 {
                true
            } else {
                false
            };

            // Check for circular structures
            // List of nodes that should have their connections checked
            let mut needs_checking: Vec<i32> = Vec::new();
            // List of nodes that requires output from node2
            let mut node_ids: Vec<i32> = Vec::new();
            // Add nodes that need checking and nodes that requires output from node2
            for conn_id in self.connections.keys() {
                let gene = self.connections.get(conn_id).unwrap();

                if gene.get_in_node() == node2.get_id() {
                    // connection comes from node2
                    node_ids.push(gene.get_out_node());
                    needs_checking.push(gene.get_out_node());
                }
            }

            while !needs_checking.is_empty() {
                let node_id = needs_checking.get(0).unwrap().to_owned();
                for conn_id in self.connections.keys() {
                    let gene = self.connections.get(conn_id).unwrap();

                    // connection comes from the needs_checking node
                    if gene.get_in_node() == node_id {
                        node_ids.push(gene.get_out_node());
                        needs_checking.push(gene.get_out_node());
                    }
                }
                needs_checking.remove(0);
            }

            // loop through dependent nodes
            for i in node_ids {
                // if we make it here, then node1 calculation is dependent on node2 calculation, creating a deadlock
                if i == node1.get_id() {
                    connection_impossible = true;
                }
            }

            let mut connection_exists: bool = false;
            for con in self.connections.values() {
                // Existing connection
                if con.get_in_node() == node1.get_id() && con.get_out_node() == node2.get_id() {
                    connection_exists = true;
                    break;
                } else if con.get_in_node() == node2.get_id()
                    && con.get_out_node() == node1.get_id()
                {
                    // Existing reverse connection
                    connection_exists = true;
                    break;
                }
            }

            if connection_exists || connection_impossible {
                continue;
            }

            let new_con = ConnectionGene::new(
                node1.get_id(),
                node2.get_id(),
                weight,
                true,
                innovation.get_innovation(),
            );
            self.connections.insert(new_con.get_innovation(), new_con);

            success = true;
        }
        if !success {
            println!("A mutation connection could not be established");
        }
    }

    // ! FUNCTION IS NOT RELIABLE, MAY SOMETIMES LOSE INPUTS AND OUTPUTS
    /// # add_node_mutation
    /// Adds a new mutation between two other nodes
    pub fn add_node_mutation(
        &mut self,
        connection_innovation: &mut Counter,
        node_innovation: &mut Counter,
    ) {
        let mut rng = rand::thread_rng();

        // Find some suitable connections in the genome
        let mut suitable_connections: Vec<i32> = Vec::new();
        for connection in self.connections.values() {
            if connection.is_expressed() {
                suitable_connections.push(connection.get_innovation());
            }
        }

        // Check if there are any suitable connections
        if suitable_connections.is_empty() {
            println!("A suitable connection was not found in the genome");
            return;
        }

        let length = rng.gen_range(0, suitable_connections.len());
        let con = suitable_connections.get(length).unwrap();
        let con = self.connections.get_mut(con).unwrap();

        // Get the connections in and out nodes, then disable the connection and create a new one
        let in_node = self.nodes.get(&(con.get_in_node() as i32)).unwrap().clone();
        let out_node = self
            .nodes
            .get(&(con.get_out_node() as i32))
            .unwrap()
            .clone();

        // Disable the connection
        con.disable();

        // Add the new node and the new connections
        let new_node = NodeGene::new(NodeGeneType::HIDDEN, node_innovation.get_innovation());
        let in_to_new = ConnectionGene::new(
            in_node.get_id(),
            new_node.get_id(),
            1.0,
            true,
            connection_innovation.get_innovation(),
        );
        let new_to_out = ConnectionGene::new(
            new_node.get_id(),
            out_node.get_id(),
            con.get_weight(),
            true,
            connection_innovation.get_innovation(),
        );

        self.nodes.insert(new_node.get_id(), new_node);

        self.connections
            .insert(in_to_new.get_innovation(), in_to_new);
        self.connections
            .insert(new_to_out.get_innovation(), new_to_out);
    }

    /// # crossover
    /// ### Takes two parents and outputs a child.
    ///
    /// **parent_1** is always the more "fit" parent
    ///
    /// **parent_2** is the less "fit" parent
    pub fn crossover(
        parent_1: &Genome,
        parent_2: &Genome,
        disabled_gene_inheriting_chance: f32,
    ) -> Genome {
        let mut child = Genome::new();

        // Add nodes to the child from the most fit parent, in this case it's parent 1
        for parent_1_node in parent_1.get_node_genes().values() {
            child.add_node_gene(parent_1_node.clone());
        }

        // Add connection genes to the child
        for parent_1_con in parent_1.get_connection_genes().values() {
            let mut rng = rand::thread_rng();

            // Find a matching gene or add the most fittest one
            if parent_2
                .get_connection_genes()
                .contains_key(&parent_1_con.get_innovation())
            {
                // Get parent 2 connection
                let parent_2_con = parent_2
                    .get_connection_genes()
                    .get(&parent_1_con.get_innovation())
                    .unwrap()
                    .clone();

                // Check if both are disabled
                let disabled: bool = !parent_1_con.is_expressed() || !parent_2_con.is_expressed();

                // Child connection gene
                let mut child_con_gene: ConnectionGene = if rng.gen() {
                    parent_1_con.clone()
                } else {
                    parent_2_con.clone()
                };

                // Give it a random chance to disable
                let rnd_float: f32 = rng.gen();
                if disabled && rnd_float < disabled_gene_inheriting_chance {
                    child_con_gene.disable();
                }

                // Add a random gene from one of the parents to the child
                child.add_connection_gene(child_con_gene);
            } else {
                let connection_gene = parent_1_con.clone();

                // If it's disjoint or excess, add the most fit one to the child
                child.add_connection_gene(connection_gene);
            }
        }

        child
    }

    /// # compatibility_distance
    /// How identical two genomes are, identical ones will output a zero,
    /// [article describing the equation.](http://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf)
    ///
    /// C1, C2, and C3 are importance coefficients'
    pub fn compatibility_distance(
        genome1: &Genome,
        genome2: &Genome,
        c1: f64,
        c2: f64,
        c3: f64,
    ) -> f64 {
        let excess_genes = Genome::count_excess_genes(&genome1, &genome2) as f64;
        let disjoint_genes = Genome::count_disjoint_genes(&genome1, &genome2) as f64;
        let avg_weight_diff = Genome::average_weight_diff(&genome1, &genome2);

        // Number of genes in the larger genome, normalizes for genome size
        // (can be set to 1 if both genomes are small, i.e, consists of fewer than 20 genes)
        let n = 1.0; // self.nodes.size()

        ((c1 * excess_genes) / n) + ((c2 * disjoint_genes) / n) + c3 * avg_weight_diff
    }

    pub fn count_matching_genes(genome1: &Genome, genome2: &Genome) -> i32 {
        let mut matching_genes: i32 = 0;

        let node_keys1 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome1.get_node_genes().keys()));
        let node_keys2 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome2.get_node_genes().keys()));

        let highest_innovation1 = node_keys1.get(node_keys1.len() - 1).unwrap();
        let highest_innovation2 = node_keys2.get(node_keys2.len() - 1).unwrap();

        let indices = cmp::max(highest_innovation1, highest_innovation2).clone();
        for i in 0..indices {
            let node_genes_genome1 = genome1.get_node_genes();
            let node_genes_genome2 = genome2.get_node_genes();

            let node1 = node_genes_genome1.get(&i);
            let node2 = node_genes_genome2.get(&i);
            let node1_exists = match node1 {
                Some(_) => true,
                None => false,
            };

            let node2_exists = match node2 {
                Some(_) => true,
                None => false,
            };

            // Check if both "exists"
            if node1_exists && node2_exists {
                // Both genomes has the gene w/ this innovation number
                matching_genes += 1;
            }
        }

        let con_keys1 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome1.get_connection_genes().keys()));
        let con_keys2 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome2.get_connection_genes().keys()));

        let highest_innovation1 = con_keys1.get(con_keys1.len() - 1).unwrap();
        let highest_innovation2 = con_keys2.get(con_keys2.len() - 1).unwrap();

        let indices = cmp::max(highest_innovation1, highest_innovation2).clone();
        for i in 0..indices {
            let connection_genes_genome1 = genome1.get_connection_genes();
            let connection_genes_genome2 = genome2.get_connection_genes();

            let connection1 = connection_genes_genome1.get(&i);
            let connection2 = connection_genes_genome2.get(&i);

            let con1_exists = match connection1 {
                Some(_) => true,
                None => false,
            };

            let con2_exists = match connection2 {
                Some(_) => true,
                None => false,
            };

            if con1_exists && con2_exists {
                // both genomes has the gene w/ this innovation number
                matching_genes += 1;
            }
        }

        return matching_genes;
    }

    pub fn count_disjoint_genes(genome1: &Genome, genome2: &Genome) -> i32 {
        let mut disjoint_genes: i32 = 0;

        let node_keys1 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome1.get_node_genes().keys()));
        let node_keys2 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome2.get_node_genes().keys()));

        let highest_innovation1 = node_keys1.get(node_keys1.len() - 1).unwrap();
        let highest_innovation2 = node_keys2.get(node_keys2.len() - 1).unwrap();

        let indices = cmp::max(highest_innovation1, highest_innovation2).clone();
        for i in 0..indices {
            let node_genes_genome1 = genome1.get_node_genes();
            let node_genes_genome2 = genome2.get_node_genes();

            let node1 = node_genes_genome1.get(&i);
            let node2 = node_genes_genome2.get(&i);

            let node1_exists = match node1 {
                Some(_) => true,
                None => false,
            };

            let node2_exists = match node2 {
                Some(_) => true,
                None => false,
            };

            if !node1_exists && highest_innovation1 < &i && node2_exists {
                // Genome 1 lacks gene, genome 2 has gene, genome 1 has more genes with higher innovation numbers
                disjoint_genes += 1;
            } else if !node2_exists && highest_innovation2 < &i && node1_exists {
                disjoint_genes += 1;
            }
        }

        let con_keys1 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome1.get_connection_genes().keys()));
        let con_keys2 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome2.get_connection_genes().keys()));

        let highest_innovation1 = con_keys1.get(con_keys1.len() - 1).unwrap();
        let highest_innovation2 = con_keys2.get(con_keys2.len() - 1).unwrap();

        let indices = cmp::max(highest_innovation1, highest_innovation2).clone();
        for i in 0..indices {
            let connection_genes_genome1 = genome1.get_connection_genes();
            let connection_genes_genome2 = genome2.get_connection_genes();

            let connection1 = connection_genes_genome1.get(&i);
            let connection2 = connection_genes_genome2.get(&i);
            let con1_exists = match connection1 {
                Some(_) => true,
                None => false,
            };

            let con2_exists = match connection2 {
                Some(_) => true,
                None => false,
            };

            if !con1_exists && highest_innovation1 > &i && con2_exists {
                disjoint_genes += 1;
            } else if !con2_exists && highest_innovation2 > &i && con1_exists {
                disjoint_genes += 1;
            }
        }

        disjoint_genes
    }

    pub fn count_excess_genes(genome1: &Genome, genome2: &Genome) -> i32 {
        let mut excess_genes: i32 = 0;

        let node_keys1 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome1.get_node_genes().keys()));
        let node_keys2 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome2.get_node_genes().keys()));

        let highest_innovation1 = node_keys1.get(node_keys1.len() - 1).unwrap();
        let highest_innovation2 = node_keys2.get(node_keys2.len() - 1).unwrap();

        let indices = cmp::max(highest_innovation1, highest_innovation2).clone();
        for i in 0..indices {
            let node_genes_genome1 = genome1.get_node_genes();
            let node_genes_genome2 = genome2.get_node_genes();

            let node1 = node_genes_genome1.get(&i);
            let node2 = node_genes_genome2.get(&i);

            let node1_exists = match node1 {
                Some(_) => true,
                None => false,
            };

            let node2_exists = match node2 {
                Some(_) => true,
                None => false,
            };

            if !node1_exists && highest_innovation1 < &i && node2_exists {
                excess_genes += 1;
            } else if !node2_exists && highest_innovation2 < &i && node1_exists {
                excess_genes += 1;
            }
        }

        let con_keys1 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome1.get_connection_genes().keys()));
        let con_keys2 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome2.get_connection_genes().keys()));

        let highest_innovation1 = con_keys1.get(con_keys1.len() - 1).unwrap();
        let highest_innovation2 = con_keys2.get(con_keys2.len() - 1).unwrap();

        let indices = cmp::max(highest_innovation1, highest_innovation2).clone();
        for i in 0..indices {
            let connection_genes_genome1 = genome1.get_connection_genes();
            let connection_genes_genome2 = genome2.get_connection_genes();

            let connection1 = connection_genes_genome1.get(&i);
            let connection2 = connection_genes_genome2.get(&i);
            let con1_exists = match connection1 {
                Some(_) => true,
                None => false,
            };

            let con2_exists = match connection2 {
                Some(_) => true,
                None => false,
            };

            if !con1_exists && highest_innovation1 < &i && con2_exists {
                excess_genes += 1;
            } else if !con2_exists && highest_innovation2 < &i && con1_exists {
                excess_genes += 1;
            }
        }

        excess_genes
    }

    pub fn average_weight_diff(genome1: &Genome, genome2: &Genome) -> f64 {
        let mut matching_genes: i32 = 0;
        let mut weight_difference: f64 = 0.0;

        let con_keys1 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome1.get_connection_genes().keys()));
        let con_keys2 =
            Genome::as_sorted_vec(Genome::keys_to_vec(genome2.get_connection_genes().keys()));

        let highest_innovation1 = con_keys1.get(con_keys1.len() - 1).unwrap();
        let highest_innovation2 = con_keys2.get(con_keys2.len() - 1).unwrap();

        let indices = cmp::max(highest_innovation1, highest_innovation2).clone();
        for i in 0..indices {
            let connection_genes_genome1 = genome1.get_connection_genes();
            let connection_genes_genome2 = genome2.get_connection_genes();

            let connection1 = connection_genes_genome1.get(&i);
            let connection2 = connection_genes_genome2.get(&i);

            let con1_exists = match connection1 {
                Some(_) => true,
                None => false,
            };

            let con2_exists = match connection2 {
                Some(_) => true,
                None => false,
            };

            if con1_exists && con2_exists {
                // both genomes has the gene w/ this innovation number
                matching_genes += 1;
                weight_difference +=
                    (connection1.unwrap().get_weight() - connection2.unwrap().get_weight()).abs();
            }
        }

        weight_difference / (matching_genes as f64)
    }

    /// # as_sorted_vec
    /// Sorts a vector in a ascending order
    pub fn as_sorted_vec<K>(c: Vec<K>) -> Vec<K>
    where
        K: Clone,
        K: Ord,
    {
        let mut c: Vec<K> = c.iter().map(|e| e.clone()).collect();
        c.sort_by(|a, b| a.cmp(&b));

        c
    }

    /// # keys_to_vec
    /// Takes HashMap keys and converts them to a owned vector
    pub fn keys_to_vec<K, V>(c: Keys<'_, K, V>) -> Vec<K>
    where
        K: Clone,
    {
        // Might be slow as we are allocating memory every time the function is called
        let mut keys: Vec<K> = Vec::new();
        for k in c {
            keys.push(k.clone());
        }

        keys
    }
}
