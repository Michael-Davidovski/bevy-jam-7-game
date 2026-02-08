use bevy::prelude::*;

pub mod follow;
pub mod shake;
pub mod zoom;

pub use follow::*;
pub use shake::*;
pub use zoom::*;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            FollowPlugin,
            CameraShakePlugin,
            CameraZoomPlugin,
        ));
    }
}
