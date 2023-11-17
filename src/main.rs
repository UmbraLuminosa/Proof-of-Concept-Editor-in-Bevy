use bevy::prelude::*;
use game_editor::GameEditorPlugin;

fn main() {
    App::new().add_plugins(GameEditorPlugin).run();
}
