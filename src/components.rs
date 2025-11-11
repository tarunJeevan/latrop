use bevy::{ecs::{component::Component, resource::Resource}, math::{Vec2, Vec3}, prelude::{Deref, DerefMut}};

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct ViewModel {}

#[derive(Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        return Self (
            // Arbitrary setting values, will be adjustable by the player
            Vec2::new(0.003, 0.002),
        );
    }
}

/// Vector representing the player's input, accumulated over all frames that
/// ran since last time physics simulation was advance
#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput {
    pub movement: Vec2,
}

/// Represents velocity in the physics simulation
#[derive(Component)]
pub struct Velocity(pub Vec3);

/// Actual position of the entity in the physics simulation
/// Separate from `Transform` which is really the visual representation
#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec3);

/// The value the `PhysicalTranslation` had in the last fixed timestep
/// Used for interpolation in the `interpolated_rendered_transform` system
#[derive(Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec3);

/// Simple resource that tells us whether a fixed timestep ran this frame
#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct DidFixedTimestepRunThisFrame(pub bool);

// View Model that represents the world with all of its contents
#[derive(Component)]
pub struct WorldModelCamera;

/// Used implicitly by all entites without a `RenderLayers` component
/// The `WorldModelCamera` and everything except the player are on this layer
pub const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera
/// View Model is model that represents the player's body
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;
