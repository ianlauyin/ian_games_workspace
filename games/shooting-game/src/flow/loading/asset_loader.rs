use bevy::asset::{LoadState, embedded_asset, load_embedded_asset};
use bevy::prelude::*;

use crate::res::ImageHandles;
use crate::states::AppState;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../../assets/explosion.png");
        embedded_asset!(app, "../../assets/spaceship.png");
        embedded_asset!(app, "../../assets/ufo.png");
        embedded_asset!(app, "../../assets/stars.png");

        app.insert_resource(ImageHandles {
            explosion: load_embedded_asset!(app, "../../assets/explosion.png"),
            spaceship: load_embedded_asset!(app, "../../assets/spaceship.png"),
            ufo: load_embedded_asset!(app, "../../assets/ufo.png"),
            stars: load_embedded_asset!(app, "../../assets/stars.png"),
        })
        .add_systems(Update, check_assets.run_if(in_state(AppState::Loading)));
    }
}

fn check_assets(
    mut next_state: ResMut<NextState<AppState>>,
    image_handles: Res<ImageHandles>,
    asset_server: Res<AssetServer>,
) {
    let asset_ids = [
        image_handles.explosion.id(),
        image_handles.ufo.id(),
        image_handles.stars.id(),
        image_handles.spaceship.id(),
    ];
    for asset_id in asset_ids {
        if !asset_is_loaded(asset_id, &asset_server) {
            return;
        }
    }
    next_state.set(AppState::MainMenu);
}

fn asset_is_loaded(id: AssetId<Image>, asset_server: &Res<AssetServer>) -> bool {
    match asset_server.get_load_state(id).unwrap() {
        LoadState::Loaded => true,
        LoadState::Failed(error) => panic!("{}", error),
        _ => false,
    }
}
