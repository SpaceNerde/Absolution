// Campaigns are the main part of the early stage of the game
// Will provide boostes for the turn income of resources

#[derive(Debug, Clone, Copy, Default)]
pub struct MiningCampaign {
    progress: f32,
    effect: f32,
    level: i32,
    running: bool,
}

impl MiningCampaign {
    pub fn new() -> Self {
        Self {
            progress: 0., 
            effect: 0.25, 
            level: 1,
            running: false, 
        }
    }
    
    pub fn run(&mut self) {
        self.running = true;
    }

    // called on each turn this campaign is running
    pub fn update(&mut self, population: i32) {
        self.progress = (population / 1_000_000_000 / ((self.level as f32) / 0.2) as i32) as f32;
    }
}
