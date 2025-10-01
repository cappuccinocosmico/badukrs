use bevy::prelude::*;

mod rendering;
mod game;

use rendering::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Baduk/Go Game".to_string(),
                resolution: (800.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}
