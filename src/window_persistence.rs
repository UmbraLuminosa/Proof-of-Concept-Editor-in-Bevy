use crate::file_io::{read_json_config, save_json_config};
use bevy::{prelude::*, winit::WinitWindows};
use bevy_window::{PrimaryWindow, WindowMode, WindowResolution};

const FILE_NAME: &str = "window_config";

pub struct WindowPersistencePlugin;

impl Plugin for WindowPersistencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, restore_window_state)
            .add_systems(PreUpdate, on_before_close);
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MainWindowConfig {
    pub mode: WindowMode,
    pub position: WindowPosition,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

fn restore_window_state(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = primary_window.single_mut();

    let Some(config) = load_window_config() else {
        println!("Could not load window config file");
        return;
    };

    println!("Loaded window config file");
    window.resolution = WindowResolution::new(config.width as f32, config.height as f32);
    window.mode = config.mode;
    window.position = config.position;
    window.set_maximized(config.maximized);
}

fn on_before_close(
    primary_window: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    mut ev_window_will_close: EventReader<bevy::window::WindowCloseRequested>,
    winit_window: NonSend<WinitWindows>,
) {
    for _ in ev_window_will_close.read() {
        let Ok((entity, window)) = primary_window.get_single() else {
            return;
        };

        let Some(winit_window) = winit_window.get_window(entity) else {
            return;
        };

        save_window_state(&window, winit_window.is_maximized());
    }
}

fn load_window_config() -> Option<MainWindowConfig> {
    let Ok(config) = read_json_config(FILE_NAME) else {
        return None;
    };

    let Ok(state) = serde_json::from_str::<MainWindowConfig>(config.as_str()) else {
        return None;
    };

    Some(state)
}

fn save_window_state(window: &Window, maximized: bool) {
    let config = MainWindowConfig {
        mode: window.mode.clone(),
        position: window.position.clone(),
        width: window.physical_width(),
        height: window.physical_height(),
        maximized,
    };

    let serialized = serde_json::to_string(&config).unwrap();
    save_json_config(FILE_NAME, serialized);
}
