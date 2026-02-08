use bevy::prelude::*;

pub mod interactable;
pub mod messages;
pub mod items;
pub mod ice_cream;
pub mod inventory;
pub mod scene;

pub use interactable::*;
pub use messages::*;
pub use items::*;
pub use ice_cream::*;
pub use inventory::*;
pub use scene::*;

pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            messages::MessagesPlugin,
            interactable::InteractionPlugin,
        ));
    }
}
