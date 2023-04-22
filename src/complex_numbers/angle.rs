use std::f64::consts::PI;

#[derive(Debug)]
pub struct Radian {
    pub value: f64,
}
impl Radian {
    pub fn from(value: f64) -> Radian {
        Radian { value }
    } 
    pub fn toDegrees(&self) -> Degree {
        radianToDegrees(self.value)
    }
}

#[derive(Debug)]
pub struct Degree {
    pub value: f64,
}
impl Degree {
    pub fn from(value: f64) -> Degree {
        Degree { value }
    } 
    pub fn toRadians(&self) -> Radian {
        degreesToRadians(self.value)
    }
}


pub fn degreesToRadians(d: f64) -> Radian {
    let value = d * (PI / 180.0);
    Radian { value }
}
pub fn radianToDegrees(r: f64) -> Degree {
    let value = r * (180.0 / PI);
    Degree { value }
}
 
