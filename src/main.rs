#![allow(unused_variables, dead_code, unused_imports, unused_assignments)]
/// Implementation of NEAT algorithm in Rust
/// Developed by **StarToLeft**
///
/// Coded from:
/// http://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf
///
/// Made in 2020-07-03 in Gothenburg, Sweden.

mod lib;
use lib::evaluator::FitnessGenomeProvider;
use lib::evaluator::GenesisGenomeProvider;
use lib::Config;
use lib::ConnectionGene;
use lib::Counter;
use lib::Evaluator;
use lib::Genome;
use lib::GenomePrinter;
use lib::NodeGene;
use lib::NodeGeneType;

use rand::Rng;
use rand_distr::{Distribution, Normal};

// Genesis provider
pub struct GenesisProvider {}

impl GenesisProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl GenesisGenomeProvider for GenesisProvider {
    fn generate_genesis_genome(&self, genome: &Genome) -> Genome {
        for connection in genome.get_connection_genes().values_mut() {
            let normal = Normal::new(0.0, 1.0).unwrap();
            let v = normal.sample(&mut rand::thread_rng());

            connection.set_weight(v);
        }

        return genome.to_owned();
    }
}

// Genome fitness evaluator provider
pub struct GenomeFitnessProvider {}

impl GenomeFitnessProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl FitnessGenomeProvider for GenomeFitnessProvider {
    fn fitness_genome_evaluator(&self, genome: &Genome) -> f32 {
        return genome.get_connection_genes().len() as f32;
    }
}

fn main() {
    let timer = std::time::Instant::now();

    let mut printer = GenomePrinter::new();

    let mut genome = Genome::new();

    let mut connection_innovation: Counter = Counter::new();
    let mut node_innovation: Counter = Counter::new();

    // Nodes
    let n1 = node_innovation.get_innovation();
    let n2 = node_innovation.get_innovation();
    let n3 = node_innovation.get_innovation();

    let input1 = NodeGene::new(NodeGeneType::INPUT, n1);
    let input2 = NodeGene::new(NodeGeneType::INPUT, n2);
    let output = NodeGene::new(NodeGeneType::OUTPUT, n3);

    genome.add_node_gene(input1);
    genome.add_node_gene(input2);
    genome.add_node_gene(output);

    // Connections
    let c1 = connection_innovation.get_innovation();
    let c2 = connection_innovation.get_innovation();
    genome.add_connection_gene(ConnectionGene::new(n1, n3, 0.5, true, c1));
    genome.add_connection_gene(ConnectionGene::new(n2, n3, 0.5, true, c1));

    // Configuration
    // Assign a starting population
    let config: Config = Config::new(10);

    // Genesis provider
    let provider = GenesisProvider::new();

    // Initialize the genome evaluator
    let mut evaluator = Evaluator::new();
    evaluator.init(&config, &genome, Box::new(provider));

    for i in 1..config.get_population_size() + 1 {
        println!("Beginning of generation {}", i);

        // Fitness provider
        let fitness_provider = GenomeFitnessProvider::new();

        // Evaluate the generation
        evaluator.evaluate_generation(Box::new(fitness_provider));

        // println!("Generation: {}", i);
        // println!(
        //     "\t Highest fitness: {}",
        //     evaluator.get_fittest_genome().get_fitness()
        // );
        // println!("\t Amount of genomes: {}", evaluator.get_genome_amount());
        // println!("\t Printing all genomes");
        // let mut k: i32 = 0;
        // for fitness_genome in evaluator.get_last_generation_results() {
        //     k += 1;
        //     println!(
        //         "\t\t Index={}\t N{}\t C{}\t fitness={}",
        //         k,
        //         fitness_genome.get_genome().get_node_genes().len(),
        //         fitness_genome.get_genome().get_connection_genes().len(),
        //         fitness_genome.get_fitness()
        //     );
        // }

        // Print populations
        let mut fittest_genome = evaluator.get_fittest_genome().get_genome();

        let _i = i.to_string();
        let mut name = String::from("genome_");
        name.push_str(&_i);

        printer.print_genome(&mut fittest_genome, &name, &name);
    }

    let time = timer.elapsed();
    println!("Finished after {:?}", time);
}
