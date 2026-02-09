use bevy::prelude::*;

// struct ComponensPlugin;
// impl Plugin for ComponensPlugin{
//     fn build(&self, app: &mut App) {
        
//     }
// }

#[derive(Component)]
pub struct Interactable{
    pub interaction_type: InteractionType
}

impl Interactable{
    pub fn new() -> Self{
        Self { interaction_type: InteractionType::None }
    }
    pub fn new_item() -> Self{
        Self { interaction_type: InteractionType::Item }
    }

}

pub enum InteractionType{
    None,
    Item,
    Talk,
    Button,
    Give
}


