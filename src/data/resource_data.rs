#[derive(Debug, Clone, Default)]
pub struct ResourceData {
    metals: f32,
    population: i32,
    mana: f32,
    metal_change: f32,
    population_change: i32,
    mana_change: f32,
}

impl ResourceData {
    pub fn new() -> Self {
        Self { 
            metals: 0., 
            population: 0,
            mana: 0., 
            metal_change: 1., 
            population_change: 1, 
            mana_change: 1.
        } 
    }

    // population handling
    pub fn get_pop(&self) -> i32 {
        self.population
    }

    pub fn increase_pop(&mut self, pop: i32) {
        self.population += pop;
    }

    pub fn decrease_pop(&mut self, pop: i32) {
        self.population -= pop;
    }

    pub fn set_pop_change(&mut self, pop: i32) {
        self.population = pop;
    }

    // metals handling
    pub fn get_metals(&self) -> f32 {
        self.metals
    }

    pub fn increase_metals(&mut self, metals: f32) {
        self.metals += metals;
    }

    pub fn decrease_metals(&mut self, metals: f32) {
        self.metals -= metals;
    }

    pub fn set_metals_change(&mut self, metals: f32) {
        self.metals = metals;
    }

    // mana handling
    pub fn get_mana(&self) -> f32 {
        self.mana
    }

    pub fn increase_mana(&mut self, mana: f32) {
        self.mana += mana;
    }

    pub fn decrease_mana(&mut self, mana: f32) {
        self.mana -= mana;
    }

    pub fn set_mana_change(&mut self, mana: f32) {
        self.mana = mana;
    }

    pub fn turn_change(&mut self) {
        self.increase_mana(self.mana_change);
        self.increase_metals(self.metal_change);
        self.increase_pop(self.population_change);
    }
}
