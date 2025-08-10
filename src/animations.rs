use bevy::prelude::*;

// Animation types
#[derive(Component)]
pub struct CardAnimation {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub progress: f32,
    pub duration: f32,
    pub animation_type: AnimationType,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Variants will be used when animation system is fully implemented
pub enum AnimationType {
    Deal,
    Flip,
    Collect,
    Slide,
}

#[derive(Component)]
pub struct ChipAnimation {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub progress: f32,
    pub duration: f32,
}

// Plugin for animations
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            animate_cards,
            animate_chips,
            cleanup_finished_animations,
        ));
    }
}

// System to animate card movements
fn animate_cards(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut CardAnimation)>,
) {
    for (mut transform, mut animation) in query.iter_mut() {
        animation.progress += time.delta_seconds() / animation.duration;
        
        if animation.progress <= 1.0 {
            // Smooth easing function (ease-out)
            let eased_progress = 1.0 - (1.0 - animation.progress).powi(3);
            
            // Interpolate position
            transform.translation = animation.start_pos.lerp(animation.end_pos, eased_progress);
            
            // Add slight rotation for dealing animation
            if matches!(animation.animation_type, AnimationType::Deal) {
                let rotation_amount = (1.0 - eased_progress) * 0.3;
                transform.rotation = Quat::from_rotation_z(rotation_amount);
            }
        } else {
            // Animation finished, set final position
            transform.translation = animation.end_pos;
            transform.rotation = Quat::IDENTITY;
        }
    }
}

// System to animate chip movements
fn animate_chips(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut ChipAnimation)>,
) {
    for (mut transform, mut animation) in query.iter_mut() {
        animation.progress += time.delta_seconds() / animation.duration;
        
        if animation.progress <= 1.0 {
            // Smooth easing with slight arc for chip movement
            let eased_progress = 1.0 - (1.0 - animation.progress).powi(2);
            
            // Create slight arc for chip movement
            let base_pos = animation.start_pos.lerp(animation.end_pos, eased_progress);
            let arc_height = 30.0 * (eased_progress * (1.0 - eased_progress) * 4.0);
            
            transform.translation = Vec3::new(base_pos.x, base_pos.y + arc_height, base_pos.z);
        } else {
            // Animation finished
            transform.translation = animation.end_pos;
        }
    }
}

// Clean up finished animations
fn cleanup_finished_animations(
    mut commands: Commands,
    card_query: Query<(Entity, &CardAnimation)>,
    chip_query: Query<(Entity, &ChipAnimation)>,
) {
    // Remove finished card animations
    for (entity, animation) in card_query.iter() {
        if animation.progress >= 1.0 {
            commands.entity(entity).remove::<CardAnimation>();
        }
    }
    
    // Remove finished chip animations
    for (entity, animation) in chip_query.iter() {
        if animation.progress >= 1.0 {
            commands.entity(entity).remove::<ChipAnimation>();
        }
    }
}
