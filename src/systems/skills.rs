use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::resources::SkillTree;
use crate::states::AppState;

pub fn upgrade_skill(mut contexts: EguiContexts, mut skills: ResMut<SkillTree>, player_query: Query<&mut Player>) {
    egui::Window::new("Skill Tree").show(contexts.ctx_mut(), |ui| {
        ui.label("Hakowanie:");
        if ui.button("Upgrade (koszt: 100 XP)").clicked() && player_query.single().xp >= 100 {
            skills.hack_speed_level += 1;
            // Update player
        }
        ui.label("Stealth:");
        // Podobnie dla innych ścieżek
    });
}
