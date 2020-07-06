use crate::Config;

use crate::Counter;
use crate::Genome;

mod fitness_genome;
use fitness_genome::FitnessGenome;

use rand::Rng;

// https://github.com/hydrozoa-yt/hydroneat/blob/master/src/com/hydrozoa/hydroneat/Evaluator.java

pub struct Evaluator {
    config: Config,

    next_generation: Vec<Genome>, // stores next generation of genomes (used during evaluation)
    genomes: Vec<Genome>,         // stores all genomes of current generation
    evaluated_genomes: Vec<FitnessGenome>, // stores all genomes with fitness of current generation (used during evaluation). Incidentally, this list contains results from previous generation.

    fittest_genome: FitnessGenome, // Last generation fittest genome
    last_generation_results: Vec<FitnessGenome>, // Last generations genome fitness-results

    node_innovation: Counter,
    connection_innovation: Counter,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            config: Config::new(0),

            next_generation: Vec::new(),
            genomes: Vec::new(),
            evaluated_genomes: Vec::new(),

            fittest_genome: FitnessGenome::new(Genome::new(), 0.0),
            last_generation_results: Vec::new(),

            node_innovation: Counter::new(),
            connection_innovation: Counter::new(),
        }
    }

    pub fn init(
        &mut self,
        config: &Config,
        default_genome: &Genome,
        genome_provider: Box<dyn GenesisGenomeProvider>,
    ) {
        self.config = config.clone();
        self.genomes = Vec::with_capacity(self.config.get_population_size());
        for _ in 0..self.config.get_population_size() {
            let g: Genome = genome_provider
                .as_ref()
                .generate_genesis_genome(&default_genome);
            self.genomes.push(g);
        }
        // Reset the values, as this function might be used more than one time
        self.evaluated_genomes = Vec::new();
        self.next_generation = Vec::new();

        self.last_generation_results = Vec::new();
        self.node_innovation = Counter::new();
        self.connection_innovation = Counter::new();
    }

    pub fn evaluate_generation(&mut self, fitness_provider: Box<dyn FitnessGenomeProvider>) {
        // Reset
        self.last_generation_results.clear();
        self.evaluated_genomes.clear();

        // Score the genomes
        for g in &mut self.genomes {
            let fitness_genome = FitnessGenome::new(
                g.clone(),
                fitness_provider.as_ref().fitness_genome_evaluator(&g),
            );
            self.evaluated_genomes.push(fitness_genome);
        }

        // Sort evalutated genomes by fitness score
        self.evaluated_genomes.sort_by(|a, b| {
            b.get_fitness()
                .partial_cmp(&a.get_fitness())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Append genomes to last generation
        // Needs a temporary clone due to append function clearing the reference
        let mut temp_evaluated_genomes = self.evaluated_genomes.clone();
        self.last_generation_results
            .append(&mut temp_evaluated_genomes);

        // Kill the worst 9 / 10 genomes
        let cutoff_index = self.evaluated_genomes.len() / 10;
        let ev_genomes = self.evaluated_genomes.clone();
        let mut index = 0;
        for _ in ev_genomes.iter() {
            if index > cutoff_index {
                self.evaluated_genomes.remove(index);
            } else {
                index += 1;
            }
        }

        // Clear the "old" next generation
        self.next_generation.clear();

        // Pick out the most fittest genome
        let fittest_genome: Genome = self.evaluated_genomes.get(0).unwrap().get_genome();

        println!(
            "FITTEST GENOME: {:?}\n\n",
            (fittest_genome.get_node_genes())
        );
        self.next_generation.push(fittest_genome);
        self.fittest_genome = self.evaluated_genomes.get(0).unwrap().clone();

        // Fill the next generation, and mutate it, also add random mating ,)
        while self.next_generation.len() < self.config.get_population_size() {
            // Sexual reproduction
            let mut rng = rand::thread_rng();

            let should_sexually_reproduce: f32 = rng.gen();
            if should_sexually_reproduce > self.config.a_sexual_reproduction_rate {
                // Sexual reproduction
                let parent1 = self
                    .evaluated_genomes
                    .get(rng.gen_range(0, self.evaluated_genomes.len()))
                    .unwrap();
                let parent2 = self
                    .evaluated_genomes
                    .get(rng.gen_range(0, self.evaluated_genomes.len()))
                    .unwrap();

                // Initialize child
                let mut child: Genome;
                // Crossover between parents
                if parent1.get_fitness() > parent2.get_fitness() {
                    child = Genome::crossover(
                        &parent1.get_genome(),
                        &parent2.get_genome(),
                        self.config.disabled_gene_inheriting_chance,
                    );
                } else {
                    child = Genome::crossover(
                        &parent2.get_genome(),
                        &parent1.get_genome(),
                        self.config.disabled_gene_inheriting_chance,
                    );
                }

                // Random weights mutation
                if rng.gen::<f32>() < self.config.mutation_rate {
                    child.mutation(self.config.pertrubing_rate);
                }

                // Random add node mutation
                if rng.gen::<f32>() < self.config.add_node_rate {
                    // ! Panics, 'UniformSampler::sample_single: low >= high'
                    
                    // child.add_node_mutation(
                    //     &mut self.connection_innovation,
                    //     &mut self.node_innovation,
                    // );
                }

                // Random connection mutation
                if rng.gen::<f32>() < self.config.add_connection_rate {
                    child.add_connection_mutation(&mut self.connection_innovation, 100);
                }

                self.next_generation.push(child);
            } else {
                // Get a random parent to base the child from
                let random_parent_index = rng.gen_range(0, self.evaluated_genomes.len());
                let parent = self.evaluated_genomes.get(random_parent_index).unwrap();
                let mut child = parent.get_genome().clone();

                // Mutate the childs weights based on the configs pertrubing rate
                child.mutation(self.config.pertrubing_rate);

                self.next_generation.push(child);
            }
        }

        // Transfer next generation to next current generation
        self.genomes.clear();
        for g in &self.next_generation {
            self.genomes.push(g.clone());
        }
    }

    pub fn get_fittest_genome(&self) -> FitnessGenome {
        self.fittest_genome.clone()
    }

    pub fn get_genome_amount(&self) -> usize {
        self.genomes.len()
    }

    pub fn get_genomes(&self) -> Vec<Genome> {
        self.genomes.clone()
    }

    pub fn get_last_generation_results(&self) -> Vec<FitnessGenome> {
        self.last_generation_results.clone()
    }
}

pub trait GenesisGenomeProvider {
    fn generate_genesis_genome(&self, genome: &Genome) -> Genome;
}

pub trait FitnessGenomeProvider {
    fn fitness_genome_evaluator(&self, genome: &Genome) -> f32;
}
