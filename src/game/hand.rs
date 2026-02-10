use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{interactable::{Interactable, InteractionType}, states::GameState};

pub struct HandPlugin;
impl Plugin for HandPlugin{
    fn build(&self, app: &mut App) {
        app
        .configure_sets(Update, HandUpdateSet)
        .add_systems(Startup, setup_hand)
        .add_systems(Update, handle_grabbing.in_set(HandUpdateSet))
        .add_systems(PostUpdate, handle_movement)
        ;
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HandUpdateSet;

#[derive(Component)]
pub struct Hand {
    pub is_grabbing: bool,
    pub grab_joint_entity: Option<Entity>,
    pub grabbed_body: Option<Entity>,
}

fn setup_hand(
    mut cmds: Commands
){
    cmds.spawn((
        Hand {
            is_grabbing: false,
            grab_joint_entity: None,
            grabbed_body: None
        },
        Sprite::from_color(Color::linear_rgb(0.1, 0.1, 0.1), vec2(5.0, 5.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::KinematicPositionBased,
        Collider::ball(5.0),
        Sensor,
    ));
}

fn handle_movement(
    mut hand_q: Query<&mut Transform, With<Hand>>,
    
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,

){
    let Ok((camera, camera_transform)) = camera_query.single() else { return };
    let Ok(window) = windows.single() else { return };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            for mut hand_transform in hand_q.iter_mut(){
                hand_transform.translation.x = world_pos.x;
                hand_transform.translation.y = world_pos.y;
            }
        }
    }

}

fn handle_grabbing(
    mut cmds: Commands,
    rapier_context: ReadRapierContext,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut hand_q: Query<(Entity, &mut Hand, &Transform)>,
    rigidbody_query: Query<(Entity, &Transform, &RigidBody), With<Collider>>,
){
    let rapier_context = rapier_context.single().unwrap();
    let Ok((hand_entity, mut hand, hand_transform,)) = hand_q.single_mut() else {return};

    if mouse_buttons.just_pressed(MouseButton::Left) {
        rapier_context.intersect_shape(
            hand_transform.translation.truncate(), 
            0.0, 
            Collider::ball(5.0).raw.make_mut(), 
            QueryFilter::default(),
            | entity| {
                if hand_entity == entity{return true;}

                if let Ok((rigidbody_entity, rigidbody_transform, rigidbody)) = rigidbody_query.get(entity) {
                    if matches!(rigidbody, RigidBody::Dynamic){
                        let grabbed_pos = rigidbody_transform.translation.truncate();
                        let offset_world = hand_transform.translation.truncate() - grabbed_pos;
                        
                        let rotation = rigidbody_transform.rotation;
                        let offset_local = rotation.inverse() * offset_world.extend(0.0);
                        
                        let grab_joint = RevoluteJointBuilder::new()
                            .local_anchor1(offset_local.truncate())
                            .local_anchor2(Vec2::ZERO)
                            .motor_position(0.0, 1.0, 5.0);

                        let joint_entity = cmds.spawn(
                            ImpulseJoint::new(entity, grab_joint)
                        ).set_parent_in_place(hand_entity).id();

                        info!("Grabbed entity: {:?}", rigidbody_entity);
                        
                        hand.is_grabbing = true;
                        hand.grab_joint_entity = Some(joint_entity);
                        hand.grabbed_body = Some(entity);

                        return false;
                    }
                }
                return false;
            }
        );
    }
    if mouse_buttons.just_released(MouseButton::Left){
        if hand.is_grabbing {
            if let Some(joint_entity) = hand.grab_joint_entity {
            cmds.entity(joint_entity).despawn();
        }
            hand.is_grabbing = false;
            hand.grab_joint_entity = None;
            hand.grabbed_body = None;
        }
    }
}

