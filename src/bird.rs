use rand::*;

#[derive(Debug)]
pub enum BirdSpecies {
    Pelican,
    Finch,
    Eagle,
    Dodo,
}

pub struct Bird {
    pub x: f64,
    pub y: f64,
    pub state: BirdState,
    species: BirdSpecies,
}

#[derive(Debug, PartialEq)]
pub enum BirdState {
    Flying,
    Attacking,
}

pub struct Bullet {
    x: f64,
}

impl Bird {
    pub fn new() -> Self {
        let bird = Bird {
            x: 0f64,
            y: thread_rng().gen_range(1f64, 600f64).round(),
            state: BirdState::Flying,
            species: BirdSpecies::Pelican,

        };
        bird
    }

    pub fn render(&mut self) {
        // we're gonna put some stuff here prob
    }

    pub fn get_position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn shoot(&mut self, edge: f64){
        while self.x < edge as f64 {
            self.x += 1.0;
        }
    }

    pub fn customize(&mut self) {
        let spcs = self.random_species();
        self.species = spcs;
    }

    fn random_species(&mut self) -> BirdSpecies {
        let species = thread_rng().gen_range(0, 4); // 2nd num is exclusive
        println!("species has a value of {}.", species);
        match species {
            0 => BirdSpecies::Pelican,
            1 => BirdSpecies::Finch,
            2 => BirdSpecies::Eagle,
            3 => BirdSpecies::Dodo,
            _ => unreachable!(),
        }
    }

    pub fn attack(&mut self) {
        self.state = BirdState::Attacking;
        println!("self.state changed");
    }
}
