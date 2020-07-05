#[path = "./genome/genome.rs"]
pub mod genome;
pub use genome::connection_gene::ConnectionGene;
pub use genome::node_gene::NodeGene;
pub use genome::node_gene::NodeGeneType;
pub use genome::Genome;

#[path = "./debugging/printer.rs"]
pub mod printer;
pub use printer::GenomePrinter;

#[path = "./genome/counter.rs"]
pub mod counter;
pub use counter::Counter;

#[path = "./evaluator/evaluator.rs"]
pub mod evaluator;
pub use evaluator::Evaluator;

#[path = "./config.rs"]
pub mod config;
pub use config::Config;