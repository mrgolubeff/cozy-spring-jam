use bevy::prelude::*;

fn main() {
    // Set up DefaultPlugins to render window.
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Cozy Spring Jam".into(),
            resolution: (1280, 720).into(),
            ..default()
        }),
        ..default()
    });

    App::new().add_plugins(default_plugins).run();
}
