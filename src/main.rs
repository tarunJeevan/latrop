use bevy::{DefaultPlugins, app::Startup, ecs::system::Commands, prelude::{PluginGroup, App, default}, window::{Window, WindowPlugin}};

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
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup (
    mut commands: Commands,
) {
    
}