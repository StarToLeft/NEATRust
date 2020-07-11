use crate::Config;

use crate::Counter;
use crate::Genome;

use crate::FitnessGenome;

use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Species {
    pub players: Vec<FitnessGenome>,

    // Best fitness of all "players"
    pub best_fitness: f64,
    pub average_fitness: f64,

    // How many generations the species have gone without improvements
    pub staleness: i32,

    // Representation of the species
    pub rep: Genome,
}

impl Species {
    pub fn new(rep: Genome) -> Species {
        let mut players = Vec::new();
        players.push(FitnessGenome::new_empty(rep.clone()));

        Species {
            players,

            best_fitness: 0.0,
            average_fitness: 0.0,

            staleness: 0,

            rep,
        }
    }

    /// # kill
    /// kills the worst performing genomes ("players")
    pub fn kill(&mut self) -> bool {
        // Sort the players by fitness
        self.sort_players();

        if self.players.len() < 2 {
            return false;
        }

        let cutoff_index = self.players.len() / 2;
        let ev_genomes = self.players.clone();
        let mut index = 0;
        for _ in ev_genomes.iter() {
            if index > cutoff_index {
                self.players.remove(index);
            } else {
                index += 1;
            }
        }

        true
    }

    /// # sort_players
    /// Sorts all players within the Species by their fitness, also updates staleness
    pub fn sort_players(&mut self) {
        self.players.sort_by(|a, b| {
            b.get_fitness()
                .partial_cmp(&a.get_fitness())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !self.players.is_empty() {
            let best = self.players.get(0).unwrap();
            if best.get_fitness() > self.best_fitness {
                self.staleness = 0;
                self.best_fitness = best.get_fitness();
                self.rep = best.get_genome();
            } else {
                self.staleness += 1;
            }
        }
    }

    /// # sort_species
    /// Sorts a list of species by their best scoring genomes fitness
    pub fn sort_species(species: &mut Vec<Species>) {
        // Update all best fitness values for the species
        for s in species.iter_mut() {
            s.refresh_best_fitness();
        }

        species.sort_by(|a, b| {
            let a_fitness = a.get_best_fitness();
            let b_fitness = b.get_best_fitness();

            a_fitness
                .partial_cmp(&b_fitness)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// # same_species
    /// Checks if a genome belongs to the same species
    pub fn same_species(&mut self, g: Genome, config: &Config) -> bool {
        let distance =
            Genome::compatibility_distance(&self.rep, &g, config.c1, config.c2, config.c3);

        config.compatibility_threshold > distance
    }

    /// # add_player
    /// Adds a new player to the species
    pub fn add_player(&mut self, player: FitnessGenome) {
        self.players.push(player);
    }

    pub fn get_average_fitness(&mut self) -> f64 {
        let mut sum: f64 = 0.0;
        for i in 0..self.players.len() {
            sum += self.players.get(i).unwrap().get_fitness();
        }

        self.average_fitness = sum;

        sum / self.players.len() as f64
    }

    pub fn refresh_best_fitness(&mut self) -> f64 {
        self.sort_players();

        self.best_fitness = self.get_best_player().get_fitness();

        self.best_fitness
    }

    pub fn get_best_fitness(&self) -> f64 {
        self.best_fitness
    }

    pub fn get_best_player(&mut self) -> &FitnessGenome {
        self.sort_players();

        self.players.get(0).unwrap()
    }

    /// # fitness_sharing
    /// Protect unique individuals ("players") from getting killed by dividing their fitness by the number of players
    pub fn fitness_sharing(&mut self) {
        let len = self.players.len();
        for p in self.players.iter_mut() {
            p.fitness /= len as f64;
        }
    }

    /// # generate_offspring
    /// Generates a new "child" from two sort of random parents within the species
    pub fn generate_offspring(
        &mut self,
        mut connection_innovation: &mut Counter,
        mut node_innovation: &mut Counter,
        config: &Config,
    ) -> FitnessGenome {
        let mut child: FitnessGenome = FitnessGenome::new_empty(Genome::new());

        // Select sort of random parents
        let parent1 = self.select_parent().clone();
        let parent2 = self.select_parent().clone();

        // Create an empty fitnessgenome and generate crossover the parents

        if parent1.fitness < parent2.fitness {
            child = FitnessGenome::new_empty(Genome::crossover(
                &parent2.genome,
                &parent1.genome,
                config.disabled_gene_inheriting_chance,
            ));
          } else {
            child = FitnessGenome::new_empty(Genome::crossover(
                &parent1.genome,
                &parent2.genome,
                config.disabled_gene_inheriting_chance,
            ));
          }

        

        let mut rng = rand::thread_rng();

        // Random weights mutation
        if rng.gen::<f32>() < config.mutation_rate {
            child.genome.mutation(config.pertrubing_rate);
        }

        // Random add node mutation
        if rng.gen::<f32>() < config.add_node_rate {
            child
                .genome
                .add_node_mutation(&mut connection_innovation, &mut node_innovation);
        }

        // Random connection mutation
        if rng.gen::<f32>() < config.add_connection_rate {
            child
                .genome
                .add_connection_mutation(&mut connection_innovation, 100);
        }

        child
    }

    pub fn select_parent(&mut self) -> &FitnessGenome {
        let mut fitness_sum = 1.0;
        for p in self.players.iter() {
            fitness_sum += p.fitness;
        }

        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0.0, fitness_sum);
        let mut running_sum = 0.0;

        for i in 0..self.players.len() {
            running_sum += self.players[i].fitness;

            if running_sum > rand {
                return &self.players[i];
            }
        }

        return self.get_best_player();
    }
}
