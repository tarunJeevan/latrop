use std::f32::consts::FRAC_PI_2;

use bevy::{app::{App, FixedPreUpdate, FixedUpdate, Plugin, PreUpdate, RunFixedMainLoop, RunFixedMainLoopSystems}, camera::{Camera, Camera3d}, ecs::{query::{With}, schedule::IntoScheduleConfigs, system::{Query, Res, ResMut, Single}}, input::{ButtonInput, keyboard::KeyCode, mouse::AccumulatedMouseMotion}, math::{EulerRot, Quat, Vec2, Vec3}, time::{Fixed, Time}, transform::components::Transform};

use crate::components::{AccumulatedInput, CameraSensitivity, DidFixedTimestepRunThisFrame, PhysicalTranslation, Player, PreviousPhysicalTranslation, Velocity};

// Needed because, for mathmatical reasons, the yaw will effectively be flipped when the pitch is as the extremes
// For that reason we clamp the pitch
const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DidFixedTimestepRunThisFrame>()
            // beginning of each frame clear the flag that indicates whether the fixed timestep has run this frame
            .add_systems(PreUpdate, clear_fixed_timestep_flag)
            // beginning of each fixed timestep, set the flag that indicates whether the fixed timestep has run this frame
            .add_systems(FixedPreUpdate, set_fixed_time_step_flag)
            .add_systems(FixedUpdate, advance_physics)
            .add_systems(
                // Allows us to schedule systems to run before and after the fixed timesetp
                RunFixedMainLoop,
                (
                    (
                        // Camera needs to be rotated before the physics simulation is advanced so the physics simulation can use the current rotation
                        rotate_camera,
                        accumulate_input,
                    )
                        // chain() groups systems together and makes them run in the order they are places
                        .chain()
                        .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                    (
                        clear_input.run_if(did_fixed_timestep_run_this_frame),
                        interpolate_rendered_transform,
                    )
                        .chain()
                        .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop)
                )
            );
    }
}

/// Handles updating physical translations of entites calculating based on a fixed time
fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &Velocity,
    )>,
) {
    for (mut current_physical_translation, mut previous_physical_translation, velocity) in query.iter_mut() {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();
    }
}

/// Accumulates the players input for setting the appropriate movement velocity
/// Calculated based on the input movement with the entities rotation
fn accumulate_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut AccumulatedInput, &mut Velocity)>,
    camera: Single<&Transform, (With<Camera3d>, With<CameraSensitivity>)>,
) {
    // Bevy's 3D renderer assumes SI units, unit of meters per second
    let (mut input, mut velocity) = player.into_inner();
    
    // Reset the input to zero before reading the new input
    input.movement = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        input.movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input.movement.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input.movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input.movement.x += 1.0;
    }
    
    // Remap 2D input to Bevy's 3D coordinate system
    // Pressing W makes `input.y` go up. Bevy assumes that -Z is forward
    let input_3d = Vec3 {
        x: input.movement.x,
        y: 0.0,
        z: -input.movement.y,
    };
    
    let rotated_input = camera.rotation * input_3d;
    velocity.0 = rotated_input.clamp_length_max(1.0) * 4.0;
}

/// Clear input after it was process in the fixed timestep
fn clear_input(mut input: Single<&mut AccumulatedInput>) {
    **input = AccumulatedInput::default();
}

/// Handles rotating the player's camera based on the AccumulatedMouseMotion, CameraSensitvity, and the player's Transform
fn rotate_camera(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Camera>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();
    
    let delta: Vec2 = accumulated_mouse_motion.delta;
    
    if delta != Vec2::ZERO {
        /*
         * Mouse movement already provides the full movement so no need to multiply by delta time
         */
        let delta_yaw: f32 = -delta.x * camera_sensitivity.x;
        let delta_pitch: f32 = -delta.y * camera_sensitivity.y;
        
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw: f32 = yaw + delta_yaw;
        
        let pitch: f32 = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

/// Update the visual transform to the physical transform
fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in query.iter_mut() {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;
        
        // The overstep fration is a value between 0 and 1 that tells us how far we are between two fixed timesteps
        let alpha = fixed_time.overstep_fraction();
        // Performs a linear interpolation between self and current based on alpha
        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation;
    }
}

/// Reset flag at the start of every frame
fn clear_fixed_timestep_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = false;
}

/// Set the flag during each fixed timestep
fn set_fixed_time_step_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = true;
}

fn did_fixed_timestep_run_this_frame(
    did_fixed_timestep_run_this_frame: Res<DidFixedTimestepRunThisFrame>
) -> bool {
    did_fixed_timestep_run_this_frame.0
}
