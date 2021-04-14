use crate::structs::{ColorVec, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Light {
    pos: Vec3,
    color: ColorVec,
}

impl Light {
    pub fn new(pos: Vec3, color: ColorVec) -> Light {
        Light { pos, color }
    }

    pub fn pos(&self) -> Vec3 { self.pos }
    pub fn color(&self) -> ColorVec { self.color }
}
