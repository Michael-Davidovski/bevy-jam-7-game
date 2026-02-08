use bevy::prelude::*;

use crate::game::{Item, messages::{ChangeScene, TalkToNPC}};


pub struct InteractionPlugin;

impl Plugin for InteractionPlugin{
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
pub struct Interactable {
    pub description: String,
    pub action: InteractionAction,
    pub condition: Option<Condition>,
}

impl Interactable {
    pub fn new(action: InteractionAction, description: String, condition: Option<Condition>) -> Self{
        Self { description, action, condition }
    }

    pub fn from_item(item: Item, description: String, condition: Option<Condition>) -> Self{
        Self { 
            description,
            action: InteractionAction::PickUpItem(item),
            condition: condition,
        }
    }

    pub fn from_scene(scene: String, description: String, condition: Option<Condition>) -> Self{
        Self { 
            description,
            action: InteractionAction::GoToScene(scene),
            condition: condition,
        }
    }

    pub fn from_npc(npc: String, description: String, condition: Option<Condition>) -> Self{
        Self { 
            description,
            action: InteractionAction::TalkToNPC(npc),
            condition: condition,
        }
    }
}

enum InteractionAction {
    GoToScene(String),
    PickUpItem(Item),
    TalkToNPC(String),
}

struct Condition{
    fullfilled: bool,
    condition_type: ConditionType,
}

enum ConditionType {
    NeedsItem,
    TalkedToNPC(String),
}

fn handle_interaction(
    mut cmds: Commands,
    interactable_query: Query<&Interactable>,
){
    for interactable in interactable_query.iter() {

        match &interactable.action {
            InteractionAction::GoToScene(scene) => {
                cmds.write_message(ChangeScene(scene.clone()));
            }
            InteractionAction::PickUpItem(item) => {

            }
            InteractionAction::TalkToNPC(npc) => {
                cmds.write_message(TalkToNPC(npc.clone()));
            }   
            _ => {}
        }
    }
}
