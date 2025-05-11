// Handler struct for all the game system

use crate::data::game_data::GameData;

use super::campaigns::{Campaign, CampaignKind, CampaignStatus};

#[derive(Default, Debug, Clone, Copy)]
pub struct GameSystem {
    pub campaign: Campaign
}

impl GameSystem {
    pub fn new() -> Self {
        Self { 
            campaign: Campaign::new(), 
        }
    }

    pub fn start_new(&mut self, kind: CampaignKind) {
        self.campaign.start_new(kind);
    }

    pub fn update(&mut self, data: &mut GameData) {
        // TODO! implement widget with progress indicater!

        match self.campaign.update(data.get_pop()) {
            CampaignStatus::Running => (),
            CampaignStatus::Finished => data.set_metals_change(data.get_metals_change() + self.campaign.get_effect()),
            CampaignStatus::Paused => (),
        }
    }
}
