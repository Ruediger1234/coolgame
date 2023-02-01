use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use super::debug::debug_ui;

pub fn game_main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(EguiPlugin);
    app.add_system(debug_ui::open_debug_window);
    app.run();
}
