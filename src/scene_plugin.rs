use bevy::{app::Update, camera::Camera3d, color::palettes::css::SILVER, input::{ButtonInput, keyboard::KeyCode}, math::{Vec3, primitives::Plane3d}, pbr::{MeshMaterial3d, wireframe::WireframeConfig}, prelude::{App, Assets, Color, Commands, Mesh, Mesh3d, Meshable, Plugin, PositionType, Res, ResMut, StandardMaterial, Startup, default}, transform::components::Transform, ui::{Node, px, widget::Text}};

const PLANE_X: f32 = 1000.0;
const PLANE_Y: f32 = 1000.0;
const PLANE_SUB_DIVISIONS: u32 = 10;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, toggle_wireframe);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(PLANE_X, PLANE_Y).subdivisions(PLANE_SUB_DIVISIONS))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
    
    // camera positioned slightly above the ground plane
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 14.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        Text::new("Press space to toggle wireframes"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
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
