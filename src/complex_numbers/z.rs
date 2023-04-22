use super::*;
#[derive(Debug)]
pub struct CartesianComplexNumber {
    pub real: f64,
    pub imaginary: f64,
}
impl CartesianComplexNumber {
    pub fn new(real: f64, imaginary: f64) -> CartesianComplexNumber {
        CartesianComplexNumber { real, imaginary }
    }
    pub fn toPolar(&self) -> PolarComplexNumber {
        let magnitude = (self.real.powi(2) + self.imaginary.powi(2)).sqrt();
        let angle = Angle::fromRadians(self.imaginary.atan2(self.real));
        PolarComplexNumber { magnitude, angle }
    }
}
#[derive(Debug)]
pub struct PolarComplexNumber {
    pub magnitude: f64,
    pub angle: Angle,
}
impl PolarComplexNumber {
    pub fn new(magnitude: f64, angle: Angle) -> PolarComplexNumber {
        PolarComplexNumber { magnitude, angle }
    }
    pub fn toCartesian(&self) -> CartesianComplexNumber {
        let r = self.angle.r.value;
        let real = self.magnitude * r.cos();
        let imaginary = self.magnitude * r.sin();
        CartesianComplexNumber { real, imaginary }
    }
}
