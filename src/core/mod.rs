use bevy::prelude::*;

pub mod camera;
pub mod input;
pub mod persistence;
pub mod utils;

pub use camera::*;
pub use input::*;
pub use persistence::*;
pub use utils::*;


pub mod message;
pub mod message_handlers;
pub mod resources;
pub mod states;

pub use message::*;
pub use resources::*;
pub use states::*;

use crate::core::message_handlers::MessageHandlersPlugin;

pub struct CorePlugins;

impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            CameraPlugin,
            MessagesPlugin,
            MessageHandlersPlugin,
            ResourcesPlugin,
            CoreStatesPlugin,
        ));
    }
}
