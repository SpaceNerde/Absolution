#[derive(Debug, Clone, Default)]
pub struct ResourceData {
    metals: f32,
    population: i32,
    mana: f32,
    founds: f32,
    metal_change: f32,
    population_change: i32,
    mana_change: f32,
    found_change: f32,
}

impl ResourceData {
    pub fn new() -> Self {
        Self { 
            metals: 0., 
            population: 8_000,
            mana: 0., 
            metal_change: 1., 
            population_change: 1, 
            mana_change: 1.,
            founds: 0.,
            found_change: 1.0,
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
        self.population_change = pop;
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
        self.metal_change = metals;
    }

    pub fn get_metals_change(&self) -> f32 {
        self.metal_change
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
        self.mana_change = mana;
    }

    // founds handling
    pub fn get_founds(&self) -> f32 {
        self.founds
    }

    pub fn increase_founds(&mut self, founds: f32) {
        self.founds += founds;
    }

    pub fn decrease_founds(&mut self, founds: f32) {
        self.founds -= founds;
    }

    pub fn set_founds_change(&mut self, founds: f32) {
        self.found_change = founds;
    }

    pub fn turn_change(&mut self) {
        self.increase_mana(self.mana_change);
        self.increase_metals(self.metal_change);
        self.increase_pop(self.population_change);
        self.increase_founds(self.found_change);
    }
}
