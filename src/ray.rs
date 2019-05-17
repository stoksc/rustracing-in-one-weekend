use crate::vec3::Vec3;

pub struct Ray<'a> {
    pub a: &'a Vec3,
    pub b: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn origin(&self) -> &Vec3 {
        self.a
    }

    pub fn direction(&self) -> &Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + &(t * self.b)
    }
}
