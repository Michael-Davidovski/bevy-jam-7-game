use bevy::prelude::*;

pub mod interaction;
pub mod messages;
pub mod items;
pub mod ice_cream;
pub mod inventory;

pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            messages::MessagesPlugin,
            interaction::InteractionPlugin,
        ));
    }
}
