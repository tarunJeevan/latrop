use bevy::{app::Startup, asset::Assets, camera::{Camera, Camera3d, PerspectiveProjection, Projection, visibility::{RenderLayers, Visibility}}, color::{Color, palettes::tailwind}, ecs::{children, system::{Commands, ResMut}}, light::NotShadowCaster, math::{Vec3, primitives::Cuboid}, mesh::{Mesh, Mesh3d}, pbr::{MeshMaterial3d, StandardMaterial}, prelude::{App, Plugin, SpawnRelated}, transform::components::Transform, utils::default};

use crate::components::{AccumulatedInput, CameraSensitivity,PhysicalTranslation, Player, PreviousPhysicalTranslation, VIEW_MODEL_RENDER_LAYER, Velocity, ViewModel, WorldModelCamera};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    // Bevy automatically handles children components relative Transform
    // For example with the player, the 'arm' here will stay relatively to he player where they move
    // Different RenderLayer's can be set so that different cameras only render things on their layer
    // the default layer is 0
    commands.spawn((
        Player {},
        Visibility::default(),
        Transform::from_xyz(0.0, 0.5, 0.0),
        AccumulatedInput::default(),
        PhysicalTranslation(Vec3 { x: 0.0, y: 0.5, z: 0.0 }),
        PreviousPhysicalTranslation(Vec3 { x: 0.0, y: 0.5, z: 0.0 }),
        Velocity(
            Vec3::ZERO
        ),
        children![
            (
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
                CameraSensitivity::default(),
            ),
            // Spawn view model camera
            (
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ),
            // Spawn the player's right arm
            (
                ViewModel{},
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The are is free-floating, as shadows would look weird
                NotShadowCaster,
            ),
        ],
    ));
}
