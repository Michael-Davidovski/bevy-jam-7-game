use bevy::prelude::*;

struct InteractionPlugin;

impl Plugin for InteractionPlugin{
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Component)]
struct Interactive {
    description: String,
    action: InteractionAction,
}

enum InteractionAction {
    GoToScene(String),
    PickUpItem,
    TalkToNPC(String),
}

fn handle_interaction(
    cmds: Commands,
){
    
}
