// Campaigns are the main part of the early stage of the game
// Will provide boostes for the turn income of resources

use core::f32;

// TODO! rewrite using trait
// TODO! increase level when finishing campaign

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CampaignStatus {
    Running,
    Finished,
    #[default]
    Paused
}

#[derive(Debug, Clone, Copy, Default)]
pub enum CampaignKind {
    #[default]
    None,
    MiningCampaign,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Campaign {
    effect: f32,
    mining_progress: f32,
    mining_level: i32,
    running: CampaignStatus,
    kind: CampaignKind,
}

impl Campaign {
    pub fn new() -> Self {
        Self {
            effect: 0.25, 
            mining_progress: 0., 
            mining_level: 1,
            running: CampaignStatus::Paused,
            kind: CampaignKind::default(), 
        }
    }
    
    pub fn start_new(&mut self, kind: CampaignKind) {
        match kind {
            CampaignKind::None => (),
            CampaignKind::MiningCampaign => self.kind = CampaignKind::MiningCampaign,
        }

        self.running = CampaignStatus::Running;
    }
    
    pub fn get_effect(&self) -> f32 {
        self.effect
    }

    // called on each turn this campaign is running
    // Returns CampaignStatus Finished when the Campaign Progress reaches 100%
    // Campaign Status will be handled in the game loop 
    // TODO! Go into more detail
    pub fn update(&mut self, population: i32) -> CampaignStatus {
        match self.check_status() {
            CampaignStatus::Running => (),
            CampaignStatus::Finished => return CampaignStatus::Finished,
            CampaignStatus::Paused => return CampaignStatus::Paused,
        }
        
        match self.kind {
            CampaignKind::None => (),
            CampaignKind::MiningCampaign => {
                self.mining_progress += (population / 100 / ((self.mining_level as f32) / 0.2) as i32) as f32;
            }
        }

        CampaignStatus::Running
    }
    
    fn check_status(&mut self) -> CampaignStatus {
        if self.running == CampaignStatus::Paused {
            return CampaignStatus::Paused
        }

        if self.mining_progress >= 100. {
            self.mining_progress = 0.;
            self.mining_level += 1;

            return CampaignStatus::Finished;
        }
        
        CampaignStatus::Running
    }

    pub fn get_current_campaign(&self) -> CampaignKind {
        self.kind
    }

    // TODO! will rewrite the campaign shit anyways so i can let this stay like this for now
    // change later so! pls dont forget :,)
    pub fn get_progress(&self) -> f32 {
        self.mining_progress
    }

    pub fn get_level(&self) -> i32 {
        self.mining_level
    }
}
