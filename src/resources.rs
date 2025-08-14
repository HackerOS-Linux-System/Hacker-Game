use bevy::prelude::*;

#[derive(Resource)]
pub struct SkillTree {
    pub hack_speed_level: i32,
    pub stealth_level: i32,
    pub exploration_level: i32,
}

#[derive(Resource)]
pub struct HackingGame {
    pub code: String,
    pub input: String,
    pub time_left: f32,
}

#[derive(Resource)]
pub struct TerminalState {
    pub input: String,
    pub output: Vec<String>,
}
