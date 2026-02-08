use bevy::prelude::*;

pub struct MessagesPlugin;

impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChangeScene>();
        app.add_message::<PickUpItem>();
        app.add_message::<TalkToNPC>();
    }
}

#[derive(Message)]
pub struct ChangeScene(pub String);

#[derive(Message)]
pub struct PickUpItem{
    pub entity: Entity,
    pub name: String
}

#[derive(Message)]
pub struct TalkToNPC(pub String);
