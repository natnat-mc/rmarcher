use crate::structs::{Vec3, ColorVec, COLOR_ZERO, Ray, Y, X};
use crate::object::Obj;
use crate::material::{Material, SurfaceType};
use crate::consts::*;
use crate::light::Light;
use std::f64::consts::PI;
use rand::prelude::*;
use crossbeam_channel::unbounded;
use std::io::Write;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cam {
    pos: Vec3,
    dir: Vec3,
    up: Vec3,
    right: Vec3
}

impl Cam {
    pub fn new(pos: Vec3, dir: Vec3, fov: f64) -> Cam {
        let right = (dir ^ UP).unit() * fov;
        let up = (right ^ dir).unit() * fov;
        let dir = dir.unit();
        Cam { pos, dir, up, right }
    }

    pub fn new_pointing(pos: Vec3, pointing: Vec3, fov: f64) -> Cam {
        Self::new(pos, pointing - pos, fov)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct RayData {
    pub pos: Vec3,
    pub dir: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub dist: f64,
    pub steps: u32
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct LightRayData<'a> {
    pub light: &'a Light,
    pub dir: Vec3,
    pub dist: f64
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Slice {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize
}

type CastData = [Option<RayData>; MAX_BOUNCES as usize];
const NONE_CAST_DATA: CastData = [None; MAX_BOUNCES as usize];
pub type Image = [u8; IMG_BYTE_SIZE];

fn l2s(l: f64) -> u8 {
    (f64::powf(f64::clamp(l, 0., 1.), 1./2.2) * 255. + 0.5) as u8
}

fn randab(a: f64, b: f64) -> f64 {
    random::<f64>() * (b-a) + a
}

fn shoot_ray<T: Obj>(scene: &T, pos: Vec3, dir: Vec3) -> Option<RayData> {
    let dir = dir.unit();
    let mut steps = 0;
    let mut dist = 0.;
    let ray = Ray::new(pos, dir);
    while steps < MAX_STEPS && dist < MAX_DIST {
        let pos = ray.point(dist);
        let d = scene.distance_to(pos);
        if d < EPSILON {
            return Some(RayData {
                pos,
                dir,
                normal: scene.normal_at(pos),
                material: scene.material_at(pos),
                dist,
                steps
            })
        }
        dist += d;
        steps += 1;
    }
    None
}

fn shoot_ray_at<T: Obj>(scene: &T, pos: Vec3, dest: Vec3) -> bool {
    let dir = (dest - pos).unit();
    let mut steps = 0;
    let mut dist = EPSILON; //XXX 0.
    let ray = Ray::new(pos, dir);
    while steps < MAX_STEPS && dist < MAX_DIST {
        let point = ray.point(dist);
        let sd = scene.distance_to(point);
        let dd = dest.distance_to(point);
        if sd <= 0. {
            return false;
        }
        if dd <= sd {
            return true;
        }
        dist += if sd < EPSILON { EPSILON } else { sd };
        steps += 1;
    }
    false
}

impl Cam {
    pub fn render<T: Obj + Clone>(&self, scene: &T) -> Image {
        if THREAD_COUNT > 1 {
            self.render_multithreaded(scene)
        } else {
            self.render_singlethreaded(scene)
        }
    }

    pub fn render_singlethreaded<T: Obj>(&self, scene: &T) -> Image {
        let mut stderr = std::io::stderr();
        if REPORT_STATUS {
            stderr.write_all(format!("Rendering... 0/{} rows (0.00%)", IMG_HEIGHT).as_bytes()).unwrap();
            stderr.flush().unwrap();
        }

        let lights = scene.get_lights();
        let mut pixels = [0; IMG_BYTE_SIZE];
        for y in 0..IMG_HEIGHT {
            for x in 0..IMG_WIDTH {
                let field_pos = (x + y*IMG_WIDTH) * 3;
                self.render_single(scene, &lights, x, y, &mut pixels[field_pos..(field_pos+3)]);
            }

            if REPORT_STATUS {
                stderr.write_all(format!("\x1b[1K\x1b[GRendering... {}/{} rows ({:.2}%)", y+1, IMG_HEIGHT, (y+1) as f64/IMG_HEIGHT as f64).as_bytes()).unwrap();
                stderr.flush().unwrap();
            }
        }

        if REPORT_STATUS {
            stderr.write_all(format!("\x1b[1K\x1b[GRendering... Done\n").as_bytes()).unwrap();
            stderr.flush().unwrap();
        }

        pixels
    }

    pub fn render_multithreaded<T: Obj + Clone>(&self, scene: &T) -> Image {
        let mut pixels = [0; IMG_BYTE_SIZE];

        crossbeam::scope(|scope| {
            // initialize everything for render
            let lights = scene.get_lights();
            let slice_width = IMG_WIDTH / THREAD_COUNT;
            let slice_height = IMG_HEIGHT / SLICES_PER_THREAD;
            let slice_byte_size = slice_width * slice_height * 3;

            let (slice_tx, slice_rx) = unbounded::<Slice>();
            let (data_tx, data_rx) = unbounded();

            // spawn a bunch of threads
            for _i in 0..THREAD_COUNT {
                let cam = self.clone();
                let scene = scene.clone();
                let lights = lights.clone();
                let slice_rx = slice_rx.clone();
                let data_tx = data_tx.clone();

                scope.spawn(move |_| {
                    for slice in slice_rx.iter() {
                        let mut pixels = Vec::with_capacity(slice_byte_size);
                        pixels.resize(slice_byte_size, 0 as u8);
                        let data = pixels.as_mut_slice();

                        for x in 0..slice.w {
                            for y in 0..slice.h {
                                let i = (x + y*slice_width) * 3;
                                cam.render_single(&scene, &lights, slice.x+x, slice.y+y, &mut data[i..(i+3)]);
                            }
                        }

                        data_tx.send((slice, pixels)).unwrap();
                    }
                });
            }

            // don't forget to close the ends of the channel we don't use
            drop(slice_rx);
            drop(data_tx);

            // send the slice data
            let mut total_slices = 0u32;
            let mut y = 0;
            while y < IMG_HEIGHT {
                let mut x = 0;
                while x < IMG_WIDTH {
                    let w = usize::min(slice_width, IMG_WIDTH - x);
                    let h = usize::min(slice_height, IMG_HEIGHT - y);
                    slice_tx.send(Slice { x, y, w, h }).unwrap();
                    x += slice_width;

                    if REPORT_STATUS {
                        total_slices += 1;
                    }
                }
                y += slice_height;
            }
            drop(slice_tx);

            // merge stuff as we get it
            let mut stderr = std::io::stderr();
            if REPORT_STATUS {
                stderr.write_all(format!("Rendering... 0/{} slices (0.00%), 0.00% pixels", total_slices).as_bytes()).unwrap();
                stderr.flush().unwrap();
            }
            let mut rendered_slices = 0u32;
            let mut rendered_pixels = 0u64;
            for (slice, data) in data_rx {
                let data = data.as_slice();
                for sy in 0..slice.h {
                    let py = sy + slice.y;
                    for sx in 0..slice.w {
                        let px = sx + slice.x;

                        let pi = (px + py * IMG_WIDTH) * 3;
                        let si = (sx + sy * slice_width) * 3;
                        pixels[pi + 0] = data[si + 0];
                        pixels[pi + 1] = data[si + 1];
                        pixels[pi + 2] = data[si + 2];
                    }
                }

                if REPORT_STATUS {
                    rendered_slices += 1;
                    rendered_pixels += (slice.w*slice.h) as u64;
                    let pct_slices = rendered_slices as f64 / total_slices as f64 * 100.;
                    let pct_pixels = rendered_pixels as f64 / (IMG_WIDTH * IMG_HEIGHT) as f64 * 100.;
                    stderr.write_all(format!("\x1b[1K\x1b[GRendering... {}/{} slices ({:.02}%), {:.02}% pixels", rendered_slices, total_slices, pct_slices, pct_pixels).as_bytes()).unwrap();
                    stderr.flush().unwrap();
                }
            }
            if REPORT_STATUS {
                stderr.write_all(format!("\x1b[1K\x1b[GRendering... Done\n").as_bytes()).unwrap();
                stderr.flush().unwrap();
            }
        }).unwrap();

        pixels
    }

    fn render_single<T: Obj>(&self, scene: &T, lights: &Vec<Light>, x: usize, y: usize, field: &mut [u8]) -> () {
        let mut color = COLOR_ZERO;
        for xs in 0..SUPERSAMPLING {
            for ys in 0..SUPERSAMPLING {
                for _i in 0..RAYS_PER_PIXEL {
                    let cast = self.cast_single(scene, x * SUPERSAMPLING + xs, y * SUPERSAMPLING + ys);
                    color = color + self.color_single(scene, &lights, cast);
                }
            }
        }
        color = color / (SUPERSAMPLING * SUPERSAMPLING * RAYS_PER_PIXEL) as f64;

        field[0] = l2s(color.v(0));
        field[1] = l2s(color.v(1));
        field[2] = l2s(color.v(2));
    }

    fn color_single<'a, T: Obj>(&self, scene: &'a T, lights: &'a Vec<Light>, cast: CastData) -> ColorVec {
        let mut color = COLOR_ZERO;
        let mut dist: f64 = 1.;

        for i in (0..MAX_BOUNCES).rev() {
            if let Some(ray) = cast[i as usize] {
                match ray.material.surface() {
                    SurfaceType::Diffuse => {
                        color = color / dist.powf(DIST_POWER);

                        color = ray.material.reflection() * color + ray.material.emission();
                        for lightray in Self::get_lights(scene, lights, ray.pos + (scene.normal_at(ray.pos)- ray.dir) * LIGHT_EPSILON) {
                            let angle = f64::max(0., ray.dir.angle_to(lightray.dir).abs() / PI / ANGLE_CORRECTION + 1. - ANGLE_FIX_CORRECTION);
                            let dist = f64::max(1., lightray.dist / DIST_CORRECTION - DIST_FIX_CORRECTION);
                            color = color + ray.material.reflection() * lightray.light.color() / angle.powf(ANGLE_POWER) / dist.powf(DIST_POWER);
                        }
                    },
                    SurfaceType::Reflective => {
                        color = ray.material.reflection() * color + ray.material.emission();
                    },
                    SurfaceType::Stop => {
                        color = ray.material.emission();
                    }
                }
                dist = f64::max(1., ray.dist / DIST_CORRECTION - DIST_FIX_CORRECTION);
            }
        }

        color / dist.powf(DIST_POWER)
    }

    fn cast_single<T: Obj>(&self, scene: &T, x: usize, y: usize) -> CastData {
        let mut cast = NONE_CAST_DATA;
        let mut pos = self.pos;
        let mut dir = self.dir_for(x, y);
        let mut i = 0;
        while i < MAX_BOUNCES {
            let hit = shoot_ray(scene, pos, dir);
            cast[i as usize] = hit;
            match hit {
                Some(hit) => {
                    i += 1;
                    pos = hit.pos + (hit.normal - dir) * EPSILON;
                    match hit.material.surface() {
                        SurfaceType::Stop => break,
                        SurfaceType::Diffuse => {
                            let r1 = randab(0., 2.*PI);
                            let r2 = randab(0., 1.);
                            let w = hit.normal;
                            let u = ((if w.x().abs() > 0.5 { Y } else { X }) ^ w).unit();
                            let v = w ^ u;
                            let x = r1.cos()*r2.sqrt();
                            let y = r1.sin()*r2.sqrt();
                            let z = (1.-r2).sqrt();
                            dir = (u*x + v*y + w*z).unit();
                        },
                        SurfaceType::Reflective => {
                            dir = dir - hit.normal * (hit.normal * dir) * 2.;
                        }
                    }
                },
                None => break,
            }
        }
        cast
    }

    fn get_lights<'a, T: Obj>(scene: &'a T, lights: &'a Vec<Light>, pos: Vec3) -> Vec<LightRayData<'a>> {
        lights
            .into_iter()
            .filter_map(|light| {
                if shoot_ray_at(scene, pos, light.pos()) {
                    Some(LightRayData {
                        light,
                        dir: (pos - light.pos()),
                        dist: pos.distance_to(light.pos())
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn dir_for(&self, x: usize, y: usize) -> Vec3 {
        let x = (((x + (IMG_DIM - IMG_WIDTH) * SUPERSAMPLING / 2) as f64) / ((IMG_DIM * SUPERSAMPLING) as f64) - 0.5) * 2.;
        let y = (0.5 - ((y + (IMG_DIM - IMG_HEIGHT) * SUPERSAMPLING / 2) as f64) / ((IMG_DIM * SUPERSAMPLING) as f64)) * 2.;
        (self.dir + self.right * x + self.up * y).unit()
    }
}