use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::blob_world::{Blob, Blobness, BLOB_HEIGHT, BLOB_WIDTH, WORLD_HEIGHT, WORLD_WIDTH};

#[derive(SystemDesc)]
pub struct BlobSystem;

const MOVEMENT_SCALE_FACTOR: f32 = 1.5;

impl<'s> System<'s> for BlobSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Blob>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, blob, input): Self::SystemData) {
        for (blob, transform) in (&blob, &mut transforms).join() {
            if let Some(transform_x) = input.axis_value("leftright") {
                let scaled_transform_x = MOVEMENT_SCALE_FACTOR * transform_x as f32;
                let blob_x = transform.translation().x;
                transform.set_translation_x(
                    (blob_x + scaled_transform_x)
                        .min(WORLD_WIDTH - BLOB_WIDTH * 0.5)
                        .max(BLOB_WIDTH),
                );
            }
            if let Some(transform_y) = input.axis_value("updown") {
                let scaled_transform_y = MOVEMENT_SCALE_FACTOR * transform_y as f32;
                let blob_y = transform.translation().y;
                transform.set_translation_y(
                    (blob_y + scaled_transform_y)
                        .min(WORLD_HEIGHT - BLOB_HEIGHT * 0.5)
                        .max(BLOB_HEIGHT),
                );
            }
        }
    }
}
