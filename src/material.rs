use crate::structs::{ColorVec, ColorMat};
use crate::consts::COLOR_CHANNELS;
use crate::material::SurfaceType::{Diffuse, Reflective, Stop};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SurfaceType {
    Diffuse,
    Reflective,
    Stop
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    emission: ColorVec,
    reflection: ColorMat,
    surface: SurfaceType
}

impl Material {
    pub const fn new(emission: ColorVec, reflection: ColorMat, surface: SurfaceType) -> Material {
        Material { emission, reflection, surface }
    }

    pub const fn new_from_diagonal(emission: ColorVec, reflection: ColorVec, surface: SurfaceType) -> Material {
        Material { emission, reflection: ColorMat::new_from_diagonal(reflection), surface }
    }

    pub const fn emission(&self) -> ColorVec { self.emission }
    pub const fn reflection(&self) -> ColorMat { self.reflection }
    pub const fn surface(&self) -> SurfaceType { self.surface }
}

const COLOR_ZERO: ColorVec = ColorVec::new_zero();
const COLOR_ONE: ColorVec = ColorVec::new([1.; COLOR_CHANNELS]);

pub const WHITE: Material = Material::new_from_diagonal(
    COLOR_ZERO,
    COLOR_ONE,
    Diffuse
);
pub const RED: Material = Material::new_from_diagonal(
    COLOR_ZERO,
    ColorVec::new([0.75, 0.25, 0.25, 0.]),
    Diffuse
);
pub const GREEN: Material = Material::new_from_diagonal(
    COLOR_ZERO,
    ColorVec::new([0.25, 0.75, 0.25, 0.]),
    Diffuse
);
pub const BLUE: Material = Material::new_from_diagonal(
    COLOR_ZERO,
    ColorVec::new([0.25, 0.25, 0.75, 0.]),
    Diffuse
);
pub const LIGHTSOURCE: Material = Material::new_from_diagonal(
    ColorVec::new([1., 1., 1., 0.]),
    COLOR_ONE,
    Diffuse
);

pub const MIRROR: Material = Material::new_from_diagonal(
    COLOR_ZERO,
    ColorVec::new([0.9, 0.9, 0.9, 0.9]),
    Reflective
);

pub const STRONG_LIGHTSOURCE: Material = Material::new_from_diagonal(
    ColorVec::new([5., 5., 5., 0.]),
    COLOR_ZERO,
    Stop
);

pub const UV_LIGHTSOURCE: Material = Material::new_from_diagonal(
    ColorVec::new_one(3),
    ColorVec::new([0.25, 0.25, 0.25, 1.]),
    Diffuse
);
pub const FLUORESCENT: Material = Material::new(
    ColorVec::new([0.25; COLOR_CHANNELS]),
    ColorMat::new([
        [1., 0., 0., 0.75],
        [0., 1., 0., 0.25],
        [0., 0., 1., 0.],
        [0., 0., 0., 0.]
    ]),
    Diffuse
);