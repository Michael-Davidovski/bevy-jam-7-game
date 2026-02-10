use bevy::prelude::*;

pub mod camera;
pub mod hand;
pub mod interactable;
pub mod item;
pub mod machine;
pub mod npc;
pub mod recipes;
pub mod rooms;
pub mod spawner;
pub mod states;


pub struct GamePlugin;
impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            hand::HandPlugin,
            interactable::InteractablePlugin,
            item::ItemPlugin,
            machine::MachinePlugin,
            npc::NPCPlugin,
            recipes::RecepiePlugin,
            rooms::RoomPlugin,
            spawner::SpawnerPlugin,
            states::StatesPlugin,
        ));
    }
}
