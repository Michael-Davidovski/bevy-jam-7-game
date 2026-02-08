use bevy::prelude::*;
use std::f32::consts::PI;

pub struct CameraShakePlugin;

impl Plugin for CameraShakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_camera_shake, apply_camera_shake).chain());
    }
}

/// Component that applies screen shake to a camera
#[derive(Component)]
pub struct CameraShake {
    /// Current trauma level (0.0 to 1.0)
    pub trauma: f32,
    /// How quickly trauma decays over time
    pub decay_rate: f32,
    /// Maximum displacement in world units
    pub max_offset: f32,
    /// Maximum rotation in radians
    pub max_rotation: f32,
    /// Current shake offset (applied to transform)
    offset: Vec3,
    /// Current shake rotation (applied to transform)
    rotation_offset: f32,
    /// Internal time accumulator for noise
    time_offset: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            decay_rate: 1.5,
            max_offset: 100.0,
            max_rotation: 0.1,
            offset: Vec3::ZERO,
            rotation_offset: 0.0,
            time_offset: 0.0,
        }
    }
}

impl CameraShake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add trauma to trigger a shake (0.0 to 1.0)
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }

    /// Set trauma directly (0.0 to 1.0)
    pub fn set_trauma(&mut self, amount: f32) {
        self.trauma = amount.clamp(0.0, 1.0);
    }

    /// Trigger a small shake
    pub fn shake_small(&mut self) {
        self.add_trauma(0.3);
    }

    /// Trigger a medium shake
    pub fn shake_medium(&mut self) {
        self.add_trauma(0.6);
    }

    /// Trigger a large shake
    pub fn shake_large(&mut self) {
        self.add_trauma(1.0);
    }

    /// Get current shake intensity (trauma squared for better feel)
    fn shake_intensity(&self) -> f32 {
        self.trauma * self.trauma
    }
}

/// Store the original transform to restore after shake
#[derive(Component)]
struct ShakeOriginalTransform {
    translation: Vec3,
    rotation: Quat,
}

fn update_camera_shake(
    mut cmds: Commands,
    mut query: Query<(Entity, &mut CameraShake, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut shake, mut transform) in query.iter_mut() {
        // Store original transform if not already stored

        cmds.entity(entity).try_insert(ShakeOriginalTransform {
            translation: transform.translation,
            rotation: transform.rotation,
        });

        // Decay trauma over time
        shake.trauma = (shake.trauma - shake.decay_rate * time.delta_secs()).max(0.0);

        if shake.trauma > 0.0 {
            let intensity = shake.shake_intensity();
            shake.time_offset += time.delta_secs();

            // Use perlin-like noise by combining multiple sine waves
            // This creates more organic-looking shake than pure random
            let offset_x = (shake.time_offset * 10.0).sin()
                * (shake.time_offset * 15.7).cos()
                * shake.max_offset
                * intensity;

            let offset_y = (shake.time_offset * 12.5).sin()
                * (shake.time_offset * 13.3).cos()
                * shake.max_offset
                * intensity;

            let rotation = (shake.time_offset * 11.0).sin()
                * (shake.time_offset * 14.2).cos()
                * shake.max_rotation
                * intensity;

            shake.offset = Vec3::new(offset_x, offset_y, 0.0);
            shake.rotation_offset = rotation;
        } else {
            shake.offset = Vec3::ZERO;
            shake.rotation_offset = 0.0;
        }
    }
}

fn apply_camera_shake(
    mut query: Query<(&CameraShake, &mut Transform, &ShakeOriginalTransform)>,
) {
    for (shake, mut transform, original) in query.iter_mut() {
        // Apply shake offset to original position
        transform.translation = original.translation + shake.offset;

        // Apply shake rotation to original rotation
        let shake_rotation = Quat::from_rotation_z(shake.rotation_offset);
        transform.rotation = original.rotation * shake_rotation;
    }
}

// Example usage system
#[allow(dead_code)]
fn example_trigger_shake(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut shake_query: Query<&mut CameraShake>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut shake in shake_query.iter_mut() {
            shake.shake_medium();
        }
    }
}
