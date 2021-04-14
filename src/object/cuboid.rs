use crate::structs::Vec3;
use crate::object::Obj;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cuboid {
    center: Vec3,
    radius: Vec3
}

impl Cuboid {
    pub fn new(center: Vec3, radius: Vec3) -> Cuboid {
        Cuboid { center, radius }
    }
    pub fn new_xyz(cx: f64, cy: f64, cz: f64, rx: f64, ry: f64, rz: f64) -> Cuboid {
        let center = Vec3::new(cx, cy, cz);
        let radius = Vec3::new(rx, ry, rz);
        Cuboid { center, radius }
    }
}

impl Obj for Cuboid {
    fn distance_to(&self, point: Vec3) -> f64 {
        let dx = point.x() - self.center.x();
        let dy = point.y() - self.center.y();
        let dz = point.z() - self.center.z();

        if dx.abs()<self.radius.x() && dy.abs()<self.radius.y() && dz.abs()<self.radius.z() {
            -f64::min(
                f64::min(
                    f64::min(
                        f64::abs(dx-self.radius.x()),
                        f64::abs(dx+self.radius.x())
                    ),
                    f64::abs(dy-self.radius.y())
                ),
                f64::min(
                    f64::min(
                        f64::abs(dy+self.radius.y()),
                        f64::abs(dz-self.radius.z())
                    ),
                    f64::abs(dz+self.radius.z())
                )
            )
        } else {
            let dx0 = f64::max(0., dx.abs()-self.radius.x());
            let dy0 = f64::max(0., dy.abs()-self.radius.y());
            let dz0 = f64::max(0., dz.abs()-self.radius.z());

            f64::sqrt(dx0*dx0 + dy0*dy0 + dz0*dz0)
        }
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        let dx = (point.x()-self.center.y())/self.radius.x();
        let dy = (point.y()-self.center.y())/self.radius.y();
        let dz = (point.z()-self.center.z())/self.radius.z();
        let (adx, ady, adz) = (dx.abs(), dy.abs(), dz.abs());

        Vec3::new(
            if adx>=ady && adx>adz { if dx>0. { 1. } else { -1. }} else { 0. },
            if ady>=adz && ady>adx { if dy>0. { 1. } else { -1. }} else { 0. },
            if adz>=adx && adz>ady { if dz>0. { 1. } else { -1. }} else { 0. },
        )
    }
}