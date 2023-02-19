use std::borrow::BorrowMut;

use bevy::prelude::*;
use super::config::text_loader::{TextAssetLoader, TextAsset};

#[cfg(feature = "editor")]
use bevy_editor_pls::prelude::*;

pub fn game_main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "editor")]
    app.add_plugin(EditorPlugin);

    app.add_asset::<TextAsset>().add_asset_loader(TextAssetLoader::<TextAsset>::new(vec!["txt"]));
    app.insert_resource(Msaa { samples: 4 });
    app.add_startup_system(setup);
    app.add_system(debug);
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text = TextHandleList(vec![asset_server.load("test.txt")]);
    commands.insert_resource(text);
}

fn debug(text_handle: Res<TextHandleList>, test: Res<Assets<TextAsset>>) {
    for i in &text_handle.0 {
        println!("{:?}", test.get(&i).unwrap().get_text());
    }
}

#[derive(Resource)]
struct TextHandleList(Vec<Handle<TextAsset>>);