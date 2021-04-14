use crate::consts::COLOR_CHANNELS;

mod vec3;
mod mat3;
mod vec_n;
mod mat_n;

mod ray;
mod cam;

pub use vec3::{Vec3, X, Y, Z, O};
pub use mat3::{Mat3, I3, O3};
pub use vec_n::Vec;
pub use mat_n::Mat;
pub use ray::Ray;
pub use cam::{Cam, Image};

pub type ColorVec = Vec<COLOR_CHANNELS>;
pub const COLOR_ZERO: ColorVec = ColorVec::new_zero();
pub type ColorMat = Mat<COLOR_CHANNELS>;
