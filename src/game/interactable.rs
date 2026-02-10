use bevy::prelude::*;
use bevy_rapier2d::{plugin::{RapierContext, ReadRapierContext}, prelude::Collider};

use crate::game::{hand::{Hand, HandUpdateSet}, interactable, item::{self, Item}, machine::{self, Machine}, npc::{NPC, Reward}};

pub struct InteractablePlugin;
impl Plugin for InteractablePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_interactions.before(HandUpdateSet));
    }
}

#[derive(Component)]
pub struct Interactable{
    pub interaction_type: InteractionType
}

pub enum InteractionType {
    None,
    Item,
    Talk,
    Button{ action: Box<dyn Fn(&mut Commands) + Send + Sync>},
    Quest { wants: String, reward: Reward },
    Machine,
}

pub fn spawn_any_button<M: Message + Clone>(
    cmds: &mut Commands, 
    pos: Vec2, 
    message: M
) {
    cmds.spawn((
        Interactable { 
            interaction_type: InteractionType::Button {
                action: Box::new(move |cmds| {
                    // This sends the specific message type M
                    cmds.write_message(message.clone());
                })
            }
        },
        Transform::from_xyz(pos.x, pos.y, 0.0),
        Collider::cuboid(8.0, 8.0),
    ));
}


pub fn handle_interactions(
    mut cmds: Commands,
    rapier_context: ReadRapierContext,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut hand_q: Query<(Entity, &mut Hand)>,
    interactable_q: Query<(Entity, &Interactable, &Transform)>,
    mut machine_q: Query<(&mut Machine)>,
    item_q: Query<&Item>,
) {
    let Ok((hand_entity, mut hand)) = hand_q.single_mut() else { return };
    let rapier_context = rapier_context.single().unwrap();

    if mouse_buttons.just_released(MouseButton::Left) {
        if let Some(held_entity) = hand.grabbed_body {
            println!("Item {:?} wurde losgelassen", held_entity);
            for (target_entity, interactable, transform) in interactable_q.iter() {
                if target_entity == held_entity {continue;}
                let collision = rapier_context.intersection_pair(held_entity, target_entity) == Some(true);
                if collision {
                    println!("Kollision mit Interactable erkannt!");
                    match &interactable.interaction_type {
                        // InteractionType::Quest { wants, reward } => {
                        //     if let Ok(item) = item_q.get(held_entity) {
                        //         if item.name == *wants {
                        //             info!("Quest Complete! Reward: {:?}", reward);
                        //             cmds.entity(held_entity).despawn();
                        //             hand.grabbed_body = None; // Clear hand
                        //         }
                        //     }
                        // }
                        InteractionType::Machine => {
                            println!("interaction type: Machine");
                            if let Ok(item) = item_q.get(held_entity) {
                                if let Ok(mut machine) = machine_q.get_mut(target_entity){
                                    if machine.items.len() <= machine.capacity{
                                        machine.items.push(item.name.clone());
                                        cmds.entity(held_entity).despawn();
                                        println!("Item in Maschine gelegt: {}", item.name);
                                    }
                                }
                            }
                        }
                        _ => {println!("interaction type: unknown")}
                    }
                }
            }
        }
    }
}
