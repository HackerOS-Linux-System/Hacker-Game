use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub fn check_collision(
    query: Query<(Entity, &Transform, &Collider), With<crate::components::Player>>,
    targets: Query<(Entity, &Transform, &Collider), Without<crate::components::Player>>,
) -> Option<Entity> {
    let (player_entity, player_transform, player_collider) = query.single();
    for (target_entity, target_transform, target_collider) in targets.iter() {
        if let Some(_) = rapier2d::geometry::test_collision(
            player_transform,
            player_collider,
            target_transform,
            target_collider,
        ) {
            return Some(target_entity);
        }
    }
    None
}

pub fn random_position(min: f32, max: f32) -> Vec2 {
    let mut rng = rand::thread_rng();
    Vec2::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}
