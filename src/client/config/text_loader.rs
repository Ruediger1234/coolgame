use std::marker::PhantomData;

use bevy::asset::{AssetLoader, Asset, LoadedAsset, Error};
use bevy::utils::BoxedFuture;

pub struct TextAssetLoader<T> {
    pub extentions: Vec<&'static str>,
    _marker: PhantomData<T>
}

#[derive(bevy::reflect::TypeUuid, )]
#[uuid = "ab3b6858-4b58-4732-a10c-f2b40ed5f44f"]
pub struct TextAsset {
    text: String,
}

impl TextAsset {
    pub fn get_text(&self) -> &String {
        &self.text
    }
}

impl<T> TextAssetLoader<T> {
    pub fn new(exts: Vec<&'static str>) -> Self {
        Self {
            extentions: exts,
            _marker: PhantomData,
        }
    }
}

impl<T: Send + Sync + 'static> AssetLoader for TextAssetLoader<T> {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let as_string = String::from_utf8(bytes.to_vec()).unwrap();
            let asset = TextAsset {
                text: as_string,
            };
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extentions
    }
}

