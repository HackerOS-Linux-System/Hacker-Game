use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy_sprite_animation::prelude::*;

// Materiał dla shadera neonowego
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct NeonMaterial {
    #[uniform(0)]
    time: f32,
}

impl Material2d for NeonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/neon.wgsl".into()
    }
}

// Setup grafik: tło, shadery, animacje
pub fn setup_graphics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<NeonMaterial>>,
    mut animations: ResMut<Assets<SpriteAnimation>>,
) {
    // Tło z neonowym shaderem
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle::default(),
            material: materials.add(NeonMaterial { time: 0.0 }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0))
                .with_scale(Vec3::new(800.0, 600.0, 1.0)),
            ..default()
        },
    ));

    // Animacja gracza (np. skok)
    let animation = SpriteAnimation::from_frames(
        vec![
            asset_server.load("animations/hacker_jump_1.png"),
            asset_server.load("animations/hacker_jump_2.png"),
        ],
        0.2,
        true, // Loop
    );
    commands.insert_resource(animations.add(animation));
}

// Aktualizacja czasu dla shadera
pub fn update_neon_material(time: Res<Time>, mut materials: ResMut<Assets<NeonMaterial>>) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds();
    }
}

// System animacji gracza
pub fn animate_player(
    mut query: Query<(&mut Handle<SpriteAnimation>, &Transform), With<crate::components::Player>>,
    animations: Res<Assets<SpriteAnimation>>,
) {
    for (mut animation_handle, transform) in query.iter_mut() {
        // Przykład: zmień animację na skok, gdy gracz jest w powietrzu
        if transform.translation.z > 0.0 {
            // Załóż, że masz animację skoku w resources
        }
    }
}
