use bevy::prelude::*;

fn main() {
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Cozy Spring Jam".into(),
            resolution: (800, 600).into(),
            ..default()
        }),
        ..default()
    });

    App::new().add_plugins(default_plugins).run();
}
