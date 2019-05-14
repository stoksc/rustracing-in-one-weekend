use std::ops;

#[derive(Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn dot(&self, other: &Vec3) -> Vec3 {
        Vec3(self[0] * other[0], self[1] * other[1], self[2] * self[2])
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    pub fn length(&self) -> f32 {
        (self[0] * self[0] + self[0] * self[0] + self[0] * self[0]).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self[0] * self[0] + self[0] * self[0] + self[0] * self[0]
    }

    pub fn unit_vector(self) -> Vec3 {
        self.clone() / self.length()
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3(self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2])
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
        self[2] /= rhs[2];
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of range"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of range"),
        }
    }
}
