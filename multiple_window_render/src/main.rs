
use bevy::{prelude::*,diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore}};

fn main() {
    App::new()
            .add_plugins((
            (
                // // bevy::log::LogPlugin::default(),
    
                bevy::core::TaskPoolPlugin::default(),
                bevy::core::TypeRegistrationPlugin,
                bevy::core::FrameCountPlugin,
                bevy::time::TimePlugin,

                // // MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0,))), 
                // bevy::app::ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0,)),
                    
                // // bevy::transform::TransformPlugin,
                // // bevy::hierarchy::HierarchyPlugin,
                // // bevy::diagnostic::DiagnosticsPlugin,
        
                bevy::input::InputPlugin,
                
                // bevy::window::WindowPlugin::default(),

                WindowPlugin {
                    // primary_window: Some(Window { present_mode: bevy::window::PresentMode::AutoNoVsync, ..default() }),
                    ..default()
                },

                bevy::a11y::AccessibilityPlugin,
                bevy::asset::AssetPlugin::default(), //

                // // bevy::scene::ScenePlugin,

                bevy::winit::WinitPlugin::default(),
                bevy::render::RenderPlugin::default(), //
                bevy::render::texture::ImagePlugin::default(), //
            ),
            (
                // // bevy::render::pipelined_rendering::PipelinedRenderingPlugin,
                // // bevy::core_pipeline::CorePipelinePlugin,
                // // bevy::sprite::SpritePlugin,
                // // bevy::text::TextPlugin,
                // // bevy::ui::UiPlugin,
                // // bevy::pbr::PbrPlugin::default(),
                // // bevy::gltf::GltfPlugin::default(),
                // // bevy::audio::AudioPlugin::default(),
                // // bevy::gilrs::GilrsPlugin,
                // // bevy::animation::AnimationPlugin,
                // // bevy::gizmos::GizmoPlugin,

                FrameTimeDiagnosticsPlugin::default(),
            )
        ))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (bevy::window::close_on_esc,show_fps_in_title))
        .run();
}

fn show_fps_in_title( 
    mut windows: Query<&mut Window>, 
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    mut time_accum:Local<f64>,
 ) {
    let (fps,avg,smoothed) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        .map(|x|(x.value().unwrap_or_default(),x.average().unwrap_or_default(),x.smoothed().unwrap_or_default()))
        .unwrap_or_default();

    for mut window in windows.iter_mut() {
        window.title = format!("{fps:.0} {avg:.0} {smoothed:.0}");
    }

    if *time_accum>10.0 {
        let secs = time.elapsed().as_secs();

        println!("[{}:{:0>2}:{:0>2}] {fps:.0} {avg:.0} {smoothed:.0}",
            secs/(60*60),(secs/60)%60,secs%60
        );

        *time_accum=0.0;
    }

    *time_accum+= time.delta_seconds_f64();
}

fn setup_scene(
    mut commands: Commands,
) {
    for _ in 0 .. 3 {
        commands.spawn(Window { 
            // present_mode: bevy::window::PresentMode::AutoNoVsync, 
            ..default() 
        });
    }
}