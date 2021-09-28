#![allow(dead_code)]
use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    /// Constructs a new Ray with given origin and direction
    fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    fn origin(&self) -> Point3 {
        self.orig
    }

    fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Returns a position along the vector, with `t` the distance from the origin
    fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
