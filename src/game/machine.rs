use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::game::{interactable::{Interactable, InteractionType}, recipes::Recipes};

pub struct MachinePlugin;
impl Plugin for MachinePlugin{
    fn build(&self, app: &mut App) {
        
    }
}

const MACHIEN_MAX_HP: i32 = 5;
const MACHINE_CAPACITY: usize = 5;

#[derive(Component)]
pub struct Machine{
    pub items: Vec<String>,
    pub capacity: usize,
    pub hp: i32,
}

fn handle_production(
    mut machine_q: Query<(&mut Machine, &Transform)>,
    recipes: Res<Recipes>
){
    let Ok((mut machine, machine_transform)) = machine_q.single_mut() else {return};

    let ingredients = machine.items.clone();

    if let Some(result_item_name) = recipes.check_machine(ingredients) {
        info!("Rezept gefunden! Erstelle: {}", result_item_name);
        // Hier spawn_item() aufrufen...
        machine.items.clear();
    }
}

pub fn spawn_machine(cmds: &mut Commands, pos: Vec2) {
    cmds.spawn((
        Machine{
            items: Vec::new(),
            capacity: MACHINE_CAPACITY,
            hp: MACHIEN_MAX_HP
        },
        Interactable {interaction_type: InteractionType::Machine},
        Sprite::default(),
        Transform::from_xyz(pos.x, pos.y, 0.0),
        Collider::cuboid(64.0, 64.0),
        Sensor
    ));
    cmds.spawn((
        Transform::from_xyz(pos.x, pos.y, 0.0),
        Collider::cuboid(64.0, 64.0),
    ));
}
