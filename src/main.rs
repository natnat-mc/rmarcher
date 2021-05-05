mod material;
mod light;
mod object;
mod structs;
mod consts;
mod lua;

use object::*;
use crate::consts::*;
use crate::structs::*;
use crate::material::*;
use image::{ColorType, ImageFormat};
use crate::light::Light;
use crate::lua::scene_from_file;

fn default_cam() -> Cam {
    Cam::new_pointing(Y*3. - X*5., O, 0.5)
}

fn default_scene1() -> Scene {
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

fn default_scene2() -> Scene {
    let s1 = Sphere::new_xyz(0., 0., 0., 1.);
    let scaled = AffineTransform::new_linear(s1, scale(1.2));
    let moved = AffineTransform::new_translate(scaled, Vec3::new(3., 1., 1.));
    let sphere = WithMaterial::new(moved, BLUE);
    let wall = WithMaterial::new(Plane::new_xyz(0., 0., -1., 2.5), GREEN);
    let backwall = WithMaterial::new(Plane::new_xyz(-1., 0., 0., 3.), WHITE);
    let mirror = WithMaterial::new(Plane::new_xyz(0., 1., 0., 2.), MIRROR);
    let light = WithMaterial::new(Sphere::new_xyz(-1., 1.5, 1., 0.5), LIGHTSOURCE);
    Scene::new(Union::new(Union::new(sphere, Union::new(wall, backwall)), Union::new(light, mirror)))
}

fn default_scene3() -> Scene {
    let s1 = WithMaterial::new(Sphere::new_xyz(4., 0., 0., 1.), MIRROR);
    let s2 = WithMaterial::new(Sphere::new_xyz(3., 1., 1., 0.5), GREEN);
    let navion = WithMaterial::new(Plane::new_xyz(0., 1., -1., 3.), BLUE);
    let backwall = WithMaterial::new(Plane::new_xyz(-1., -1., -0.5, 8.), RED);
    let boiboite = Cuboid::new_xyz(4., 1.1, -1.1, 0.5, 0.275, 0.275);
    let decal = AffineTransform::new_translate(boiboite, Vec3::new(-0.25, 0., 0.5));
    let nope = Exclusion::new(s1, Sphere::new_xyz(3.75, 0.75, 0.75, 1.));
    let walls = Union::new(navion, backwall);
    let spheres = Union::new(AffineTransform::new_linear(nope, scale(0.75)), s2);
    let scene = Union::new(spheres, Union::new(walls, decal));
    let light = WithMaterial::new(Plane::new_xyz(0., -1., 0., 8.), BRIGHT_AF_LIGHTSOURCE);
    let scene = WithLight::new_one(scene, Light::new(Y*3.-X*5., ColorVec::new([2., 3., 4., 0.])));
    Scene::new(Union::new(scene, light))
}

fn main() {
    // get scene and camera
    let scene = scene_from_file("scenes/tsm.lua".to_owned()).unwrap();
    //let scene = default_scene3();
    let cam = Cam::new(Z * -1.5, Z, f64::sqrt(2.));
    //let cam = default_cam();

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
