use crate::config::WanderballConfig;
use crate::resources::save::CameraRecord;
use amethyst::{
    core::{transform::Transform, Parent},
    ecs::{Entity, World},
    prelude::*,
    renderer::Camera,
};

pub fn load_camera(world: &mut World, camera_record: CameraRecord, parent: Entity) {
    world
        .create_entity()
        .with(camera_record.transform.clone())
        .with(camera_record.camera)
        .with(Parent { entity: parent })
        .build();
}

pub fn initialize_camera(world: &mut World, parent: Entity) {
    let (view_height, view_width) = {
        let config = &world.read_resource::<WanderballConfig>();
        (config.view_height, config.view_width)
    };

    let camera = Camera::orthographic(
        -view_width / 2.0,
        view_width / 2.0,
        -view_height / 2.0,
        view_height / 2.0,
        1.0,
        4.0,
    );

    world
        .create_entity()
        .with(Transform::default())
        .with(camera)
        .with(Parent { entity: parent })
        .build();
}

pub fn zoom(camera: &mut Camera, width: f32, height: f32) {
    let left = -width / 2.0;
    let right = width / 2.0;
    let bottom = -height / 2.0;
    let top = height / 2.0;
    camera.matrix[(0, 0)] = 2.0 / (right - left);
    camera.matrix[(1, 1)] = -2.0 / (top - bottom);
    camera.matrix[(0, 3)] = -(right + left) / (right - left);
    camera.matrix[(1, 3)] = -(top + bottom) / (top - bottom);
    camera.inverse = camera.matrix.try_inverse().expect("Camera projection matrix is not invertible. This is normally due to having inverse values being superimposed (near=far, right=left)")
}
