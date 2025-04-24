#[derive(Debug, Clone, Default)]
pub struct ResourceData {
    metals: f32,
    population: i32,
    mana: f32,
}

impl ResourceData {
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
}
