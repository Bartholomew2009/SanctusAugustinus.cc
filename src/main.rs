use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui::{self}, EguiPlugin, EguiPrimaryContextPass,};

mod menu;
mod about_window;
mod search_menu;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(237, 237, 216)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SanctusAugustinus".into(),
                resizable: true,
                resolution: WindowResolution::new(1920.0,1080.0).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        })
        )   
        .add_systems(Startup, (spawn).chain())
        .add_plugins(EguiPlugin::default())
        .add_systems(EguiPrimaryContextPass, (menu::spawn_system,about_window::about_window,search_menu::search_menu_window))
        .init_resource::<search_menu::UiState>()
        .run();
}
fn spawn(mut command: Commands,mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut().unwrap();
    window.set_maximized(true);
    command.spawn(Camera2d);
}
