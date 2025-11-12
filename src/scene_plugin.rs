use bevy::{app::Update, camera::visibility::RenderLayers, color::palettes::tailwind, ecs::{system::{Single}}, input::{ButtonInput, keyboard::KeyCode}, light::PointLight, math::{Vec2, Vec3, primitives::{Cuboid, Plane3d}}, pbr::{MeshMaterial3d, wireframe::WireframeConfig}, prelude::{App, Assets, Color, Commands, Mesh, Mesh3d,  Plugin, Res, ResMut, StandardMaterial, Startup, default}, transform::components::Transform, window::{CursorGrabMode, CursorOptions}};

use crate::components::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup, spawn_lights, grab_mouse_cursor))
            .add_systems(Update, (toggle_wireframe, toggle_mouse_cursor_on_esc));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);
    
    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone())));
    
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));
    
    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material),
        Transform::from_xyz(0.75, 1.75, 0.0)
    ));

}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

/// Shows wireframe across meshes, good for debug and setting texture/mesh boundaries
#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

/// Toggles mouse cursor on pressing the Escape key and unlocks the mouse cursor
fn toggle_mouse_cursor_on_esc(
    mut cursor_options: Single<&mut CursorOptions>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        cursor_options.visible = !cursor_options.visible;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

/// Hides the mouse cursor on game start and locks the mouse cursor
fn grab_mouse_cursor(
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}
