use bevy::{pbr::wireframe::{WireframePlugin}, DefaultPlugins, app::Startup, ecs::system::Commands, prelude::{PluginGroup, App, default}, window::{Window, WindowPlugin}};

mod scene_plugin;

use crate::{scene_plugin::ScenePlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Latrop".to_owned(),
                    name: Some("bevy.app".into()),
                    ..default()
                }),
                ..default()}),
            #[cfg(not(target_arch = "wasm32"))]
            WireframePlugin::default(),
            ScenePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup (
    mut commands: Commands,
) {
    
}