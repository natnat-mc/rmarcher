use crate::structs::{Vec3, Y, X, Z};

pub const COLOR_CHANNELS: usize = 4;

pub const EPSILON: f64 = 1. / 1024.;
pub const LIGHT_EPSILON: f64 = 1. / 512.;
//pub const MAX_DIST: f64 = 16.;
pub const MAX_DIST: f64 = 32.;
pub const MAX_STEPS: u32 = 1024;
//pub const MAX_STEPS: u32 = u32::MAX;

pub const DIST_FIX_CORRECTION: f64 = 0.5;
pub const DIST_CORRECTION: f64 = 3.;
pub const DIST_POWER: f64 = 2.;

pub const ANGLE_FIX_CORRECTION: f64 = 0.;
pub const ANGLE_CORRECTION: f64 = 2.;
pub const ANGLE_POWER: f64 = 2.;

//pub const IMG_WIDTH: usize = 480;
//pub const IMG_WIDTH: usize = 1280;
//pub const IMG_WIDTH: usize = 1080;
//pub const IMG_WIDTH: usize = 1920;
pub const IMG_WIDTH: usize = 4961;
//pub const IMG_HEIGHT: usize = 480;
//pub const IMG_HEIGHT: usize = 720;
//pub const IMG_HEIGHT: usize = 1080;
pub const IMG_HEIGHT: usize = 3508;
pub const IMG_DIM: usize = if IMG_HEIGHT > IMG_WIDTH { IMG_HEIGHT } else { IMG_WIDTH };
pub const IMG_SIZE: usize = IMG_WIDTH * IMG_HEIGHT;
pub const IMG_BYTE_SIZE: usize = IMG_SIZE * 3;

pub const SUPERSAMPLING: usize = 1;
//pub const SUPERSAMPLING: usize = 2;
pub const RAYS_PER_PIXEL: usize = 1;
//pub const RAYS_PER_PIXEL: usize = 50;
//pub const RAYS_PER_PIXEL: usize = 500;
//pub const MAX_BOUNCES: u32 = 1;
//pub const MAX_BOUNCES: u32 = 4;
//pub const MAX_BOUNCES: u32 = 8;
pub const MAX_BOUNCES: u32 = 10;

//pub const THREAD_COUNT: usize = 1;
pub const THREAD_COUNT: usize = 12;
pub const SLICES_PER_THREAD: usize = 32;
pub const REPORT_STATUS: bool = true;

pub const UP: Vec3 = Y;
pub const RIGHT: Vec3 = X;
pub const FORWARD: Vec3 = Z;