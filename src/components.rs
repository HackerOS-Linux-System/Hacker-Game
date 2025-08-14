use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub hack_speed: f32,
    pub xp: i32,
}

#[derive(Component)]
pub struct Firewall;

#[derive(Component)]
pub struct Guard {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Terminal;
