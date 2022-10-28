use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const WORLD_HEIGHT: f32 = 800.0;
pub const WORLD_WIDTH: f32 = 800.0;

#[derive(Default)]
pub struct BlobWorld {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for BlobWorld {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        let sprite_sheet_handle = load_sprite_sheet(world);
        world.register::<BlobBall>();
        initialise_blob_ball(world, sprite_sheet_handle.clone());
        initialise_blobs(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WORLD_WIDTH * 0.5, WORLD_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WORLD_WIDTH, WORLD_HEIGHT))
        .with(transform)
        .build();
}

pub const BLOB_HEIGHT: f32 = 24.0;
pub const BLOB_WIDTH: f32 = 24.0;

pub enum Blobness {
    Dead,
    Alive,
}

pub struct Blob {
    pub blobness: Blobness,
    pub width: f32,
    pub height: f32,
}

impl Blob {
    fn new(blobness: Blobness) -> Blob {
        Blob {
            blobness,
            width: BLOB_WIDTH,
            height: BLOB_HEIGHT,
        }
    }
}

impl Component for Blob {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_blobs(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };
    let mut blob_transform = Transform::default();
    let y = WORLD_HEIGHT / 2.0;
    let x = WORLD_WIDTH / 2.0;
    blob_transform.set_translation_xyz(x, y, 0.0);
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Blob::new(Blobness::Alive))
        .with(blob_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/slime_monster_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/slime_monster_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub struct BlobBall {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for BlobBall {
    type Storage = DenseVecStorage<Self>;
}

pub const BLOB_BALL_VELOCITY_X: f32 = 75.0;
pub const BLOB_BALL_VELOCITY_Y: f32 = 75.0;
pub const BLOB_BALL_RADIUS: f32 = 16.0;

fn initialise_blob_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0, 0.0);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(BlobBall {
            radius: BLOB_BALL_RADIUS,
            velocity: [BLOB_BALL_VELOCITY_X, BLOB_BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}
