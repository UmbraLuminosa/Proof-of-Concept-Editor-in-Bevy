use bevy::prelude::*;
use egui_dock::DockState;

use crate::file_io::{read_json_config, save_json_config};

use super::{EguiWindow, UiState};

const FILE_NAME: &str = "egui_config";

pub struct EguiPersistence;

impl Plugin for EguiPersistence {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, restore_panel_state)
            .add_systems(PreUpdate, on_before_close);
    }
}

fn restore_panel_state(mut ui_state: ResMut<UiState>) {
    let Some(state) = load_panel_config() else {
        println!("Could not load egui panel config file");
        return;
    };

    ui_state.state = state;
}

fn on_before_close(
    ui_state: Res<UiState>,
    mut ev_window_will_close: EventReader<bevy::window::WindowCloseRequested>,
) {
    for _ in ev_window_will_close.read() {
        save_panel_config(&ui_state.state);
    }
}

fn load_panel_config() -> Option<DockState<EguiWindow>> {
    let Ok(config) = read_json_config(FILE_NAME) else {
        return None;
    };

    let Ok(state) = serde_json::from_str::<DockState<EguiWindow>>(config.as_str()) else {
        return None;
    };

    Some(state)
}

fn save_panel_config(state: &DockState<EguiWindow>) {
    let serialized = serde_json::to_string(state).unwrap();
    save_json_config(FILE_NAME, serialized);
}
