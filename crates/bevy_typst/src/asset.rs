use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::{
        thiserror::{self, Error},
        BoxedFuture,
    },
};

pub struct TypstAssetPlugin;

impl Plugin for TypstAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<RawTypstAsset>()
            .init_asset_loader::<RawTypstAssetLoader>();
    }
}

#[derive(Asset, TypePath)]
pub struct RawTypstAsset(String);

impl RawTypstAsset {
    pub fn content(&self) -> &str {
        &self.0
    }
}

#[derive(Default)]
pub struct RawTypstAssetLoader;

impl AssetLoader for RawTypstAssetLoader {
    type Asset = RawTypstAsset;

    type Settings = ();

    type Error = RawTypstAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut string = String::new();
            reader.read_to_string(&mut string).await?;
            Ok(RawTypstAsset(string))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["typ"]
    }
}

/// Possible errors that can be produced by [`RawTypstAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RawTypstAssetLoaderError {
    /// An [Io](std::io) Error
    #[error("Could not load typst file: {0}")]
    Io(#[from] std::io::Error),
}
