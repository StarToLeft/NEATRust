#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /***
     * constant used in genomic distance calculation - this is the weight of excess genes
     */
    pub c1: f64,
    
    /***
     * constant used in genomic distance calculation - this is the weight of disjoint genes
     */
    pub c2: f64,
    
    /**
     * constant used in genomic distance calculation - this is the weight of average connection weight difference
     */
    pub c3: f64,
    
    /**
     * genomic distance we allow before two genomes are in seperate species - two genomes belong to the same species if genomic difference is less than this number
     */
    pub dt: f64,

    /**
     * Fraction of children genomes resulting from mutation without crossover. The remaining children come from mating with corssover.
     */
    pub a_sexual_reproduction_rate: f64,
    
    /**
     * chance for each child to have it's weights mutated, each weight in the genome having a PERTURBING_RATE chance of being uniformly perturbed, and 1-PERTURBING_RATE chance of being assigned a new random value
     */
    pub mutation_rate: f32,	
    
    /**
     * This applies to mutation of genomes.
     * Each child has a MUTATION_RATE chance of mutating in each generation
     */
    pub pertrubing_rate: f32,
    
    /**
     * Chance of a weight being disabled if it is disabled in either parent
     */
    pub disabled_gene_inheriting_chance: f32,
    
    /**
     * Chance of mutating a child in a way that adds a node to the genome.
     */
    pub add_connection_rate: f32,
    
    /**
     * Chance of mutating a child in a way that adds a connection to the genome.
     */
    pub add_node_rate: f32,

    /**
     * Chance of adding a new layer while mutating node.
     */
    pub add_layer_rate: f32,

    /**
     * Percentage of offspring generated using crossover of two parents - the rest comes from asexual mutation
     */
    pub offspring_from_crossover: f64,

    /**
     * Coefficient for putting genomes into species
     */
    pub compatibility_threshold: f64,

    /**
     * Coefficient for when to kill a inactive species
     */
    pub max_species_staleness_before_kill: i32,

    pub population_size: usize,
    pub generation_count: usize,
    pub max_species: usize,
}

impl Config {
    pub fn new(population_size: usize, generation_count: usize, max_species: usize) -> Config {
        Config {
            c1: 1.5,
            c2: 0.8,
            c3: 1.0,
            dt: 3.0,

            a_sexual_reproduction_rate: 0.25,

            mutation_rate: 0.9,

            pertrubing_rate: 0.9,
            
            disabled_gene_inheriting_chance: 0.15,

            add_connection_rate: 0.25,
            add_node_rate: 0.23,
            add_layer_rate: 0.08,

            offspring_from_crossover: 0.9,

            compatibility_threshold: 1.0,

            max_species_staleness_before_kill: 15,

            population_size,
            generation_count,
            max_species,
        }
    }

    pub fn get_population_size(&self) -> usize {
        self.population_size
    }

    pub fn get_generation_count(&self) -> usize {
        self.generation_count
    }
}