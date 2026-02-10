use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::game::{interactable::{Interactable, InteractionType}, item::{Item}};

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Component)]
pub struct NPC{
    pub name: String,
}

pub enum Reward{
    Points(f32),
    Key,
}

pub fn spawn_npc(
    cmds: &mut Commands,
    position: Vec2,
    npc_name: String,
    wants: String,
    reward: Reward
){
    cmds.spawn((
        NPC {
            name: npc_name,
        },
        Interactable {interaction_type: InteractionType::Quest { wants, reward }},
        Interactable {interaction_type: InteractionType::Talk},
        Transform::from_xyz(position.x, position.y, 0.0),
        Sprite::from_color(Color::linear_rgb(0.3, 0.3, 0.3), vec2(64.0, 128.0)),
        Collider::cuboid(32.0, 64.0),
        Sensor,
    ));
    
}
