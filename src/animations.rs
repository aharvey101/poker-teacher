use bevy::prelude::*;
use rand::Rng;

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

// Helper functions to create animations
pub fn animate_card_deal(
    commands: &mut Commands,
    entity: Entity,
    start_pos: Vec3,
    end_pos: Vec3,
    duration: f32,
) {
    commands.entity(entity).insert(CardAnimation {
        start_pos,
        end_pos,
        progress: 0.0,
        duration,
        animation_type: AnimationType::Deal,
    });
}

pub fn animate_chip_movement(
    commands: &mut Commands,
    entity: Entity,
    start_pos: Vec3,
    end_pos: Vec3,
    duration: f32,
) {
    commands.entity(entity).insert(ChipAnimation {
        start_pos,
        end_pos,
        progress: 0.0,
        duration,
    });
}

// Particle effect for celebrations
#[derive(Component)]
pub struct ParticleEffect {
    pub particles: Vec<Particle>,
    pub duration: f32,
    pub elapsed: f32,
}

#[derive(Clone)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub color: Color,
}

pub fn create_win_particles(commands: &mut Commands, center: Vec3) {
    let mut particles = Vec::new();
    
    // Create 20 particles
    for _ in 0..20 {
        let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
        let speed = 50.0 + rand::random::<f32>() * 100.0;
        
        particles.push(Particle {
            position: center,
            velocity: Vec3::new(angle.cos() * speed, angle.sin() * speed, 0.0),
            lifetime: 2.0,
            max_lifetime: 2.0,
            color: Color::srgb(1.0, 0.8, 0.0), // Gold color
        });
    }
    
    commands.spawn((
        ParticleEffect {
            particles,
            duration: 2.0,
            elapsed: 0.0,
        },
        TransformBundle::from_transform(Transform::from_translation(center)),
    ));
}

// System to update particle effects
fn update_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ParticleEffect)>,
) {
    for (entity, mut effect) in query.iter_mut() {
        effect.elapsed += time.delta_seconds();
        
        if effect.elapsed >= effect.duration {
            commands.entity(entity).despawn();
            continue;
        }
        
        // Update each particle
        for particle in &mut effect.particles {
            particle.lifetime -= time.delta_seconds();
            particle.position += particle.velocity * time.delta_seconds();
            particle.velocity.y -= 150.0 * time.delta_seconds(); // Gravity
        }
        
        // Remove dead particles
        effect.particles.retain(|p| p.lifetime > 0.0);
    }
}
