use bevy::prelude::*;

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChangeScene>();
    }
}

#[derive(Message)]
pub struct ChangeScene(String);

#[derive(Message)]
pub struct PickUpItem{
    entity: Entity,
}

#[derive(Message)]
pub struct TalkToNPC(String);
