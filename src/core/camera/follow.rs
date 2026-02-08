use bevy::prelude::*;

pub struct FollowPlugin;

impl Plugin for FollowPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_follow);
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Follow{
    pub target_position: Vec3,
    pub smoothing: f32,
    pub offset: Vec3,
}

impl Default for Follow {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            smoothing: 0.0,
            offset: Vec3::ZERO,
        }
    }
}

impl Follow {
    pub fn new(target_position: Vec3) -> Self {
        Self {
            target_position: target_position,
            ..default()
        }
    }

    pub fn with_smoothing(mut self, smoothing: f32) -> Self {
        self.smoothing = smoothing;
        self
    }

    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }
}

fn handle_follow(
    mut follow_query: Query<(Entity, &Follow, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, follow, mut follow_transform) in follow_query.iter_mut() {

        let target_pos = follow.target_position + follow.offset;

        if follow.smoothing > 0.0 {
            follow_transform.translation = follow_transform.translation.lerp(
                target_pos,
                (follow.smoothing * time.delta_secs()).min(1.0),
            );
        } else {
            follow_transform.translation = target_pos;
        }
    }
}

// fn handle_follow(
//     mut follow_query: Query<(Entity, &Follow, &mut Transform)>,
//     target_query: Query<&Transform, Without<Follow>>,
//     time: Res<Time>,
// ) {
//     for (entity, follow, mut follow_transform) in follow_query.iter_mut() {
//         let Some(target_entity) = follow.target_entity else {
//             continue; // Skip this entity, not return
//         };

//         // Skip if trying to follow itself
//         if target_entity == entity {
//             continue;
//         }

//         let Ok(target_transform) = target_query.get(target_entity) else {
//             continue;
//         };

//         let target_pos = target_transform.translation + follow.offset;

//         if follow.smoothing > 0.0 {
//             follow_transform.translation = follow_transform.translation.lerp(
//                 target_pos,
//                 (follow.smoothing * time.delta_secs()).min(1.0),
//             );
//         } else {
//             follow_transform.translation = target_pos;
//         }
//     }
// }
