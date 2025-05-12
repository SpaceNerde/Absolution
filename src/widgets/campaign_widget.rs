// Widget to display all current resources and the changes
// in resources in the next turn

use ratatui::{
    buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, widgets::{Block, BorderType, Gauge, Paragraph, Widget}
};

use crate::systems::{campaigns::CampaignKind, game_system::GameSystem};

pub struct CampaignWidget {
    system: GameSystem,
}

impl CampaignWidget {
    pub fn new(system: GameSystem) -> Self {
        Self { system }
    }
}

impl Widget for CampaignWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let interface = Block::bordered()
            .border_type(BorderType::Thick)
            .title("Campaign");
    
        interface.clone().render(area, buf);
        
        let interface_area = interface.inner(area);

        let outer_layout = Layout::new(Direction::Vertical,
            vec![Constraint::Ratio(1, 3), Constraint::Ratio(1, 3), Constraint::Ratio(1, 3)]
        ).split(interface_area);

        let current_campaign = match self.system.campaign.get_current_campaign() {
            CampaignKind::None => None,
            CampaignKind::MiningCampaign => Some("Mining"),
        };
        
        // only render when a campaign is actually running
        if let Some(_current_campaign) = current_campaign {
            Paragraph::new(current_campaign.unwrap()).render(outer_layout[0], buf);
            Gauge::default()
                .gauge_style(Style::new().white().on_black().italic())
                .percent((self.system.campaign.get_progress() as u16).clamp(0, 100))
                .render(outer_layout[1], buf);
            Paragraph::new(self.system.campaign.get_level().to_string()).render(outer_layout[2], buf);
        }
    }
}
