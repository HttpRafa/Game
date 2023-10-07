use bevy::app::App;
use bevy::prelude::{Assets, AssetServer, Commands, Handle, Image, Plugin, PreStartup, Res, ResMut, Resource, TextureAtlas, Vec2};

const VEC_16: Vec2 = Vec2::new(16.0, 16.0);
const VEC_10: Vec2 = Vec2::new(10.0, 10.0);

pub const PLAYER_IDLE_ANIMATION: GameAtlas = GameAtlas::new("texture/animation/player/idle.png", VEC_16, 2,  1);
pub const PLAYER_WALKING_ANIMATION: GameAtlas = GameAtlas::new("texture/animation/player/walking.png", VEC_16, 3, 8);
pub const UI_INVENTORY: GameAtlas = GameAtlas::new("texture/ui/inventory.png", VEC_16, 3, 1);
pub const WORLD_GROUND_TILES: GameAtlas = GameAtlas::new("texture/world/ground_tiles.png", VEC_10, 5, 3);

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_textures);
    }
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    commands.insert_resource(GameTextures {
        player_idle: PLAYER_IDLE_ANIMATION.create_atlas_handle(&asset_server, &mut texture_atlases),
        player_walking: PLAYER_WALKING_ANIMATION.create_atlas_handle(&asset_server, &mut texture_atlases),
        ui_inventory: UI_INVENTORY.create_atlas_handle(&asset_server, &mut texture_atlases),
        world_ground_tiles: WORLD_GROUND_TILES.create_handle(&asset_server)
    });
}

#[derive(Resource)]
pub struct GameTextures {
    pub player_idle: Handle<TextureAtlas>,
    pub player_walking: Handle<TextureAtlas>,
    pub ui_inventory: Handle<TextureAtlas>,
    pub world_ground_tiles: Handle<Image>
}

pub struct GameAtlas {
    path: &'static str,
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize
}

impl GameAtlas {
    fn create_handle(self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        asset_server.load(self.path)
    }

    fn create_atlas_handle(self, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> Handle<TextureAtlas> {
        let texture_atlas = TextureAtlas::from_grid(asset_server.load(self.path), self.tile_size, self.columns, self.rows, None, None);
        texture_atlases.add(texture_atlas)
    }

    const fn new(path: &'static str, tile_size: Vec2, columns: usize, rows: usize) -> Self {
        Self {
            path,
            tile_size,
            columns,
            rows
        }
    }
}