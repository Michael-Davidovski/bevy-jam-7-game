use bevy::prelude::*;

pub mod camera;
pub mod interactable;
pub mod hand;
pub mod states;
pub mod rooms;
pub mod item;

pub struct GamePlugin;
impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hand::HandPlugin,
            camera::CameraPlugin,
            rooms::RoomPlugin,
        ));
    }
}
