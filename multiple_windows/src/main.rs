//! Uses two windows to visualize a 3D model from different angles.

use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef, diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore}};

fn main() {
    App::new()
        // By default, a primary window gets spawned by `WindowPlugin`, contained in `DefaultPlugins`
        .add_plugins((DefaultPlugins,FrameTimeDiagnosticsPlugin::default(),))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (bevy::window::close_on_esc,show_fps_in_title))
        .run();
}

fn show_fps_in_title( mut windows: Query<&mut Window>, diagnostics: Res<DiagnosticsStore>, ) {

    let (fps,avg,smoothed) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS)
        .map(|x|(x.value().unwrap_or_default(),x.average().unwrap_or_default(),x.smoothed().unwrap_or_default()))
        .unwrap_or_default();

    for mut window in windows.iter_mut() {
        window.title = format!("{fps:.0} {avg:.0} {smoothed:.0}");

    }

}
fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add entities to the world
    commands.spawn(SceneBundle {
        scene: asset_server.load("torus.gltf#Scene0"),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });
    // main camera, cameras default to the primary window
    // so we don't need to specify that.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    for _i in 0 .. 3 {

        // Spawn a second window
        let second_window = commands
            .spawn(Window {
                title: "Second window".to_owned(),
                ..default()
            })
            .id();

        // second window camera
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Entity(second_window)),
                ..default()
            },
            ..default()
        });
    }

}