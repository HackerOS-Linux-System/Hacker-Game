use bevy::prelude::*;
use noise::NoiseFn;
use noise::Perlin;
use rand::Rng;
use crate::components::*;
use crate::utils::*;

// Prosty BSP: dziel pokoje
pub fn generate_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let perlin = Perlin::new(0);
    let mut rng = rand::thread_rng();

    // BSP: stwórz drzewo podziału (przykładowo 4 pokoje)
    let rooms = vec![
        (Vec2::new(-200., -100.), Vec2::new(200., 100.)), // Główny pokój
        // Dodaj więcej poprzez rekurencyjny podział
    ];

    for room in rooms {
        // Umieść firewalli na brzegach
        for x in (room.0.x as i32..room.1.x as i32).step_by(32) {
            let noise_val = perlin.get([x as f64 / 100., room.0.y as f64 / 100.]);
            if noise_val > 0.2 {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("assets/firewall.png"),
                        transform: Transform::from_translation(Vec3::new(x as f32, room.0.y, 0.)),
                        ..default()
                    },
                    Firewall,
                    RigidBody::Static,
                    Collider::cuboid(16., 16.),
                ));
            }
        }
        // Umieść terminal losowo w pokoju
        let tx = rng.gen_range(room.0.x..room.1.x);
        let ty = rng.gen_range(room.0.y..room.1.y);
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("assets/terminal.png"),
                transform: Transform::from_translation(Vec3::new(tx, ty, 0.)),
                ..default()
            },
            Terminal,
            Collider::cuboid(16., 16.),
        ));
        // Strażnik
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("assets/guard.png"),
                transform: Transform::from_translation(Vec3::new(tx + 50., ty, 0.)),
                ..default()
            },
            Guard { direction: Vec2::new(1.0, 0.0) },
            Velocity::linear(Vec2::new(50.0, 0.0)),
            Collider::cuboid(16., 16.),
        ));
    }
}
