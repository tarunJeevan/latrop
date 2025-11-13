use bevy::{pbr::wireframe::{WireframePlugin}, DefaultPlugins, prelude::{PluginGroup, App, default}, window::{Window, WindowPlugin}};

mod scene_plugin;
mod physics;
mod components;
mod player_plugin;

use crate::{scene_plugin::ScenePlugin, player_plugin::PlayerPlugin, physics::PhysicsPlugin};

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
            PhysicsPlugin,
            PlayerPlugin,
        ))
        .run();
}
