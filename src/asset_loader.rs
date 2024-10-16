use bevy::asset::LoadState;
use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;

use crate::constants::WINDOW_SIZE;
use crate::states::AppState;

#[derive(Resource)]
pub struct ImageHandles {
    pub explosion: Handle<Image>,
    pub spaceship: Handle<Image>,
    pub ufo: Handle<Image>,
    pub stars: Handle<Image>,
}

#[derive(Resource)]
pub struct MeshHandles {
    pub bullet: (Handle<Mesh>, Handle<ColorMaterial>),
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets)
            .add_systems(Update, check_assets.run_if(in_state(AppState::Loading)));
    }
}
fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ImageHandles {
        explosion: asset_server.load("explosion.png"),
        spaceship: asset_server.load("spaceship.png"),
        ufo: asset_server.load("ufo.png"),
        stars: asset_server.load("stars.png"),
    });
    commands.insert_resource(MeshHandles {
        bullet: (
            meshes.add(Rectangle::default()),
            materials.add(Color::from(YELLOW)),
        ),
    });
}

fn create_star_texture_atlas(
    star_id: AssetId<Image>,
    star_texture: &Image,
) -> (TextureAtlasLayout, Image) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    texture_atlas_builder.initial_size(UVec2::new(WINDOW_SIZE.x as u32, WINDOW_SIZE.y as u32 / 2));
    for _ in 0..2 {
        texture_atlas_builder.add_texture(Some(star_id), star_texture);
    }
    return texture_atlas_builder.build().unwrap();
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
    next_state.set(AppState::Game);
}

fn asset_is_loaded(id: AssetId<Image>, asset_server: &Res<AssetServer>) -> bool {
    match asset_server.get_load_state(id).unwrap() {
        LoadState::Loaded => true,
        LoadState::Failed(error) => panic!("{}", error),
        _ => false,
    }
}
