use crate::Genome;

#[derive(Debug, Clone, PartialEq)]
pub struct FitnessGenome {
    fitness: f32,
    genome: Genome,
}

impl FitnessGenome {
    pub fn new(genome: Genome, fitness: f32) -> FitnessGenome {
        FitnessGenome { fitness, genome }
    }

    pub fn get_fitness(&self) -> f32 {
        self.fitness
    }

    pub fn get_genome(&self) -> Genome {
        self.genome.clone()
    }
}