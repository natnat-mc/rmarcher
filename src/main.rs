mod material;
mod light;
mod object;
mod structs;
mod consts;

use object::*;
use crate::consts::*;
use crate::structs::*;
use crate::material::*;
use image::{ColorType, ImageFormat};
use image::codecs::png::FilterType::Sub;

fn default_cam() -> Cam {
    Cam::new_pointing(Y*3. - X*5., O, 0.5)
}

fn default_scene() -> Scene {
    let s0 = WithMaterial::new(
        Sphere::new_xyz(0., 0., 0., 1.),
        RED
    );
    let s1 = WithMaterial::new(
        Sphere::new_xyz(0., 1., 1., 0.5),
        MIRROR
    );
    let spheres = Union::new(s0, s1);
    let floor = WithMaterial::new(
        Plane::new_xyz(0., 1., 0., 2.),
        WHITE
    );
    let backwall = WithMaterial::new(
        Plane::new_xyz(-1., 0., 0., 2.),
        BLUE
    );
    let sidewalls = WithMaterial::new(
        Union::new(
            Plane::new_xyz(0., 0., -1., 2.5),
            Plane::new_xyz(0., 0., 1., 2.5)
        ),
        GREEN
    );
    let walls = Union::new(
        Union::new(floor, backwall),
        sidewalls
    );
    let scene = Union::new(spheres, walls);
    let light = WithMaterial::new(
        Plane::new_xyz(0., -1., 0., 8.),
        LIGHTSOURCE
    );
    let scene = Union::new(scene, light);
    Scene::new(scene)
}

fn main() {
    // get scene and camera
    let scene = default_scene();
    let cam = default_cam();

    // get stats on the scene we're about to render
    let total_rpp = (RAYS_PER_PIXEL * SUPERSAMPLING * SUPERSAMPLING) as u64;
    let lights = scene.get_lights().len() as u32;
    let nodes = scene.node_count();
    let min_rays = IMG_SIZE as u64 * RAYS_PER_PIXEL as u64 * SUPERSAMPLING as u64;
    let max_rays = min_rays * MAX_BOUNCES as u64 * (1+lights) as u64;
    println!("Image size: {}x{} ({} pixels)", IMG_WIDTH, IMG_HEIGHT, IMG_SIZE);
    println!("Rpp: {}, {}x supersampling ({} total rays per image pixel)", RAYS_PER_PIXEL, SUPERSAMPLING, total_rpp);
    println!("Max bounces: {}, dist: {}, steps: {}", MAX_BOUNCES, MAX_DIST, MAX_STEPS);
    println!("Lights: {}, Nodes: {}", lights, nodes);
    println!("Threads: {}, {} slices per thread", THREAD_COUNT, SLICES_PER_THREAD);
    println!("Total rays: min {} max {}", min_rays, max_rays);

    // generate image and save it
    let data = cam.render(&scene);
    image::save_buffer_with_format(
        "a.png",
        &data,
        IMG_WIDTH as u32,
        IMG_HEIGHT as u32,
        ColorType::Rgb8,
        ImageFormat::Png
    ).unwrap();
}
