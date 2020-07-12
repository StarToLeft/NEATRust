use crate::Config;

use crate::Counter;
use crate::Genome;
use crate::Species;

use crate::GenomePrinter;

pub mod fitness_genome;
use crate::FitnessGenome;

use std::fs::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    this_generation_kills: i32,
}

pub struct Evaluator {
    config: Config,

    players: Vec<FitnessGenome>,
    species: Vec<Species>,

    best_player: FitnessGenome,

    gen_players: Vec<FitnessGenome>,

    best_fitness: f64,

    generation_id: Counter,

    node_innovation: Counter,
    connection_innovation: Counter,

    stats: Stats,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            config: Config::new(0, 0, 0),

            players: Vec::new(),
            species: Vec::new(),

            best_player: FitnessGenome::new_empty(Genome::new()),

            gen_players: Vec::new(),
            best_fitness: 0.0,

            generation_id: Counter::new(),

            node_innovation: Counter::new(),
            connection_innovation: Counter::new(),

            stats: Stats {
                this_generation_kills: 0,
            },
        }
    }

    /// # init
    /// Initialize the evaluator
    pub fn init(
        &mut self,
        config: &Config,
        default_genome: &Genome,
        genome_provider: Box<dyn GenesisGenomeProvider>,

        node_innovation: &mut Counter,
        connection_innovation: &mut Counter,
    ) {
        self.config = config.clone();
        // Clear the values
        self.species.clear();
        self.players.clear();

        self.node_innovation.current_innovation = node_innovation.load_innovation();
        self.connection_innovation.current_innovation = connection_innovation.load_innovation();

        for _ in 0..self.config.get_population_size() {
            let g: Genome = genome_provider
                .as_ref()
                .generate_genesis_genome(&default_genome);
            self.players.push(FitnessGenome::new_empty(g));
        }
    }

    /// # evaluate_generation
    /// Evaluates a generation, speciates it and kills old unused genomes
    pub fn evaluate_generation(&mut self, fitness_provider: Box<dyn FitnessGenomeProvider>) {
        self.stats.this_generation_kills = 0;

        self.speciate();
        self.calculate_fitness(&fitness_provider);
        self.sort_players();
        self.sort_species();
        self.kill_species();
        self.set_best_player();
        self.kill_stale_species();
        self.kill_bad_species();

        let average_sum = self.get_avg_fitness_sum();

        // Offspring from current generation
        let mut children: Vec<FitnessGenome> = Vec::new();

        for s in self.species.iter_mut() {
            children.push(s.players.get(0).unwrap().clone());

            // TODO: Debug
            let mut no_of_children = (s.average_fitness / average_sum).floor() as i64 - 1;

            if average_sum == 0.0 {
                no_of_children = 1;
            }

            for _ in 0..no_of_children {
                children.push(s.generate_offspring(
                    &mut self.connection_innovation,
                    &mut self.node_innovation,
                    &self.config,
                ));
            }
        }

        // Check for flooring issues leading to wrong amount of children being created
        while children.len() < self.players.len() {
            // Generate children from the best species
            // Check if all species have gotten killed
            if self.species.len() < 1 {
                println!("All species have gotten killed, ending generation");
                return;
            }

            children.push(self.species.get_mut(0).unwrap().generate_offspring(
                &mut self.connection_innovation,
                &mut self.node_innovation,
                &self.config,
            ));
        }

        // Assign the new players
        self.players = children;

        self.generation_id.get_innovation();
        println!(
            "generation {}\n\tmutations count {}\n\tspecies: {}\n\tkills: {}\n\tplayers: {}",
            self.generation_id.load_innovation(),
            self.node_innovation.load_innovation() + self.connection_innovation.load_innovation(),
            self.species.len(),
            self.stats.this_generation_kills,
            self.players.len()
        );

        // Saving each species and all its genomes into directories
        let mut printer = GenomePrinter::new();
        for (i, s) in self.species.iter().enumerate() {
            let _i = i.to_string();
            let mut name = String::from("species_");
            name.push_str(&_i);

            create_dir_all(String::from("./output/") + name.as_str()).unwrap();

            for (i, p) in s.players.iter().enumerate() {
                let _i = i.to_string();
                let mut p_name = String::from("genome_");
                p_name.push_str(&_i);

                printer.print_genome(
                    &mut p.get_genome(),
                    &(String::from(name.clone()) + "/" + &p_name),
                    &p_name,
                );
            }
        }
    }

    pub fn update_players(&mut self) {
        self.players.clear();

        for s in self.species.iter() {
            let mut species_players = s.players.clone();

            self.players.append(&mut species_players);
        }
    }

    pub fn speciate(&mut self) {
        // Clear all species
        for s in self.species.iter_mut() {
            s.players.clear();
        }

        // Speciate all individuals
        for p in self.players.iter_mut() {
            let mut species_found = false;
            for s in self.species.iter_mut() {
                if s.same_species(p.get_genome(), &self.config) {
                    s.add_player(p.clone());
                    species_found = true;
                    break;
                }
            }

            // If the player doesn't match any species, create a new one
            if !species_found {
                self.species.push(Species::new(p.get_genome()));
            }
        }
    }

    pub fn calculate_fitness(&mut self, fitness_provider: &Box<dyn FitnessGenomeProvider>) {
        // Calculate the fitness values for all individuals
        for g in &mut self.players {
            let fitness = fitness_provider
                .as_ref()
                .fitness_genome_evaluator(&g.get_genome());
            g.set_fitness(fitness);
        }
    }

    pub fn sort_species(&mut self) {
        // Sort the individuals by fitness within the species
        for s in self.species.iter_mut() {
            s.sort_players();
        }

        let mut temp: Vec<Species> = Vec::new();
        let mut s = 0;
        while s != self.species.len() {
            let mut max: f64 = 0.0;
            let mut max_index: usize = 0;
            for j in 0..self.species.len() {
                if self.species.get(j).unwrap().get_best_fitness() > max {
                    max = self.species.get(j).unwrap().get_best_fitness();
                    max_index = j;
                }
            }

            temp.push(self.species.get(max_index).unwrap().clone());
            self.species.remove(max_index);
            s -= 1;
            s += 1;
        }

        self.species = temp;
    }

    pub fn sort_players(&mut self) {
        self.players.sort_by(|a, b| {
            b.get_fitness()
                .partial_cmp(&a.get_fitness())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn kill_species(&mut self) {
        for s in self.species.iter_mut() {
            // Kill bad individuals
            self.stats.this_generation_kills += s.kill();

            // Protect unique individuals
            s.fitness_sharing();

            s.get_average_fitness();
        }

        // Kill empty species
        let mut i = 0;
        while i < self.species.len() {
            let s = self.species.get(i).unwrap();
            if s.players.len() == 0 {
                self.species.remove(i);
                i -= 1;
            }

            i += 1;
        }

        // Sort the species, kill the worst in the next step
        self.sort_players();
        self.sort_species();

        // To keep performance stable, only keep a max amount of species alive
        let mut temp = self.species.clone();
        temp.reverse();

        let mut i = 0;
        while i < temp.len() && temp.len() > self.config.max_species {
            temp.remove(i);
            i -= 1;

            i += 1;
        }

        self.species = temp;
    }

    pub fn set_best_player(&mut self) {
        let temp_best_player = self.players.get(0).unwrap();

        self.gen_players.push(temp_best_player.clone());

        // Set global best if it's better than current best score
        if temp_best_player.fitness > self.best_fitness {
            println!("Old best: {}", self.best_fitness);
            println!("New best: {}", temp_best_player.fitness);
            self.best_fitness = temp_best_player.fitness;
            self.best_player = temp_best_player.clone();
        }
    }

    pub fn kill_stale_species(&mut self) {
        let mut i = 0;
        while i < self.species.len() {
            let s = self.species.get(i).unwrap();
            if s.staleness >= self.config.max_species_staleness_before_kill {
                self.species.remove(i);
                i -= 1;
            }

            i += 1;
        }
    }

    pub fn kill_bad_species(&mut self) {
        let average_sum = self.get_avg_fitness_sum();

        let mut i = 0;
        while i < self.species.len() {
            let s = self.species.get(i).unwrap();
            if (s.average_fitness / average_sum * self.players.len() as f64) < 1.0 {
                self.species.remove(i);
                i -= 1;
            }

            i += 1;
        }
    }

    pub fn get_avg_fitness_sum(&mut self) -> f64 {
        let mut average_sum = 0.0;
        for s in self.species.iter_mut() {
            average_sum += s.get_average_fitness();
        }

        average_sum
    }

    pub fn get_fittest_genome(&self) -> FitnessGenome {
        self.best_player.clone()
    }
}

pub trait GenesisGenomeProvider {
    fn generate_genesis_genome(&self, genome: &Genome) -> Genome;
}

pub trait FitnessGenomeProvider {
    fn fitness_genome_evaluator(&self, genome: &Genome) -> f64;
}
