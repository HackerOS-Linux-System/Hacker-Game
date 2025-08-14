use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;

mod components;
mod resources;
mod states;
mod systems;
mod utils;

use crate::states::AppState;
use crate::systems::{player::*, ai::*, hacking::*, skills::*, level_gen::*, hacker_os::*, graphics::*};

pub struct HackerGamePlugin;

impl Plugin for HackerGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(EguiPlugin)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_state::<AppState>()
            .init_resource::<resources::SkillTree>()
            .add_startup_system(setup_level)
            .add_startup_system(setup_graphics)
            .add_system(player_movement.in_set(OnUpdate(AppState::Playing)))
            .add_system(guard_ai.in_set(OnUpdate(AppState::Playing)))
            .add_system(hacking_system.in_set(OnUpdate(AppState::Playing)))
            .add_system(upgrade_skill.in_set(OnUpdate(AppState::Menu)))
            .add_system(terminal_system.in_set(OnUpdate(AppState::Playing)))
            .add_system(generate_level.in_schedule(OnEnter(AppState::Playing)));
    }
}

fn main() {
    App::new()
        .add_plugins(HackerGamePlugin)
        .run();
}
