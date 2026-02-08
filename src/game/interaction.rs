use bevy::prelude::*;

use crate::game::messages::{ChangeScene, TalkToNPC};


pub struct InteractionPlugin;

impl Plugin for InteractionPlugin{
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
pub struct Interactive {
    description: String,
    action: InteractionAction,
    condition: Option<Condition>,
}

enum InteractionAction {
    GoToScene(String),
    PickUpItem,
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
    interactive_query: Query<&Interactive>,
){
    for interactive in interactive_query.iter() {
        
        match &interactive.action {
            InteractionAction::GoToScene(scene) => {
                cmds.write_message(ChangeScene(scene.clone()));
            }
            InteractionAction::PickUpItem => {

            }
            InteractionAction::TalkToNPC(npc) => {
                cmds.write_message(TalkToNPC(npc.clone()));
            }   
            _ => {}
        }
    }
}
