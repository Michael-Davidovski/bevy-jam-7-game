use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier2d::prelude::*;

use crate::game::interactable::Interactable;
use crate::game::rooms::ROOM_SIZE;

const PIXELS_PER_METER: f32 = 32.0;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ice Nudel Salat".to_string(),
                resolution: WindowResolution::new(ROOM_SIZE.x as u32, ROOM_SIZE.y as u32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER)
        )
        .add_plugins(
            RapierDebugRenderPlugin::default()
        )
        .add_plugins(game::GamePlugin)
        .run();
}
