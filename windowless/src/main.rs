
use bevy::{prelude::*,diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore}};

fn main() {
    App::new()
            .add_plugins((
            (
                // bevy::log::LogPlugin::default(),
    
                bevy::core::TaskPoolPlugin::default(),
                bevy::core::TypeRegistrationPlugin,
                bevy::core::FrameCountPlugin,
                bevy::time::TimePlugin,

                bevy::app::ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f64(1.0 / 60.0,)),
                 
                // MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f64(1.0 / 60.0,))), 

                FrameTimeDiagnosticsPlugin::default(),
            ),
        ))
        .add_systems(Update, (show_fps,))
        .run();
}

fn show_fps( 
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    mut time_accum:Local<f64>,
 ) {
    let (fps,avg,smoothed) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS)
        .map(|x|(x.value().unwrap_or_default(),x.average().unwrap_or_default(),x.smoothed().unwrap_or_default()))
        .unwrap_or_default();

    if *time_accum>10.0 {
        let secs = time.elapsed().as_secs();

        println!("[{}:{:0>2}:{:0>2}] {fps:.0} {avg:.0} {smoothed:.0}",
            secs/(60*60),(secs/60)%60,secs%60
        );

        *time_accum=0.0;
    }

    *time_accum+= time.delta_seconds_f64();
}
