use crate::Genome;

#[derive(Debug, Clone, PartialEq)]
pub struct FitnessGenome {
    pub fitness: f64,
    pub genome: Genome,
}

impl FitnessGenome {
    pub fn new(genome: Genome, fitness: f64) -> FitnessGenome {
        FitnessGenome { fitness, genome }
    }

    pub fn new_empty(genome: Genome) -> FitnessGenome {
        FitnessGenome { fitness: 0.0, genome }
    }

    pub fn get_fitness(&self) -> f64 {
        self.fitness
    }

    pub fn get_genome(&self) -> Genome {
        self.genome.clone()
    }

    pub fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness;
    }
}