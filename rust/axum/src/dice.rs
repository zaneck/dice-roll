use rand::Rng;

pub struct Dice {
    pub sides: u32,
}

impl Dice {
    pub fn new(sides: u32) -> Self {
        Self { sides }
    }

    pub fn roll(&self) -> u32 {
        let mut rng = rand::rng();
        rng.random_range(1..=self.sides)
    }
}
