use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::states::AppState;

#[derive(Resource, Default)]
pub struct AssetHandles {
    pub explosion: Handle<Image>,
    pub player: Handle<Image>,
    pub stars: Handle<Image>,
    pub ufo: Handle<Image>,
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(Update, check_assets.run_if(in_state(AppState::LoadAsset)));
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetHandles {
        explosion: asset_server.load("explosion.png"),
        player: asset_server.load("player.png"),
        stars: asset_server.load("stars.png"),
        ufo: asset_server.load("ufo.png"),
    });
}

fn check_assets(
    mut next_state: ResMut<NextState<AppState>>,
    asset_handles: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
) {
    let asset_ids = [
        asset_handles.explosion.id(),
        asset_handles.stars.id(),
        asset_handles.ufo.id(),
        asset_handles.player.id(),
    ];
    for asset_id in asset_ids {
        if !asset_is_loaded(asset_id, &asset_server) {
            return;
        }
    }
    next_state.set(AppState::InPlay);
}

fn asset_is_loaded(id: AssetId<Image>, asset_server: &Res<AssetServer>) -> bool {
    match asset_server.get_load_state(id).unwrap() {
        LoadState::Loaded => true,
        LoadState::Failed(error) => panic!("{}", error),
        _ => false,
    }
}
