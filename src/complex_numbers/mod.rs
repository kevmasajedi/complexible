pub mod angle;
pub mod z;

use angle::*;
use std::{f64::consts::PI, fmt};
use z::*;

#[derive(Debug)]
pub struct Angle {
    pub d: Degree,
    pub r: Radian,
}
impl Angle {
    pub fn fromDegrees(d: f64) -> Angle {
        let d = Degree::from(d);
        let r = d.toRadians();
        Angle { d, r }
    }
    pub fn fromRadians(r: f64) -> Angle {
        let r = Radian::from(r);
        let d = r.toDegrees();
        Angle { d, r }
    }
}
impl Clone for Angle {
    fn clone(&self) -> Angle {
        Angle::fromDegrees(self.d.value)
    }
}

#[derive(Debug)]
pub struct ComplexNumber {
    cartesian: CartesianComplexNumber,
    polar: PolarComplexNumber,
}

impl ComplexNumber {
    pub fn fromCartesian(real: f64, imaginary: f64) -> ComplexNumber {
        let cartesian = CartesianComplexNumber { real, imaginary };
        let polar = cartesian.toPolar();
        ComplexNumber { cartesian, polar }
    }
    pub fn fromPolar(magnitude: f64, angle: Angle) -> ComplexNumber {
        let polar = PolarComplexNumber { magnitude, angle };
        let cartesian = polar.toCartesian();
        ComplexNumber { cartesian, polar }
    }
    pub fn fromReal(real: f64) -> ComplexNumber {
        ComplexNumber::fromCartesian(real, 0.0)
    }
    pub fn abs(&self) -> f64 {
        self.polar.magnitude
    }
    pub fn angleInRads(&self) -> f64 {
        self.polar.angle.r.value
    }
    pub fn angleInDegs(&self) -> f64 {
        self.polar.angle.d.value
    }
    pub fn angleInAngle(&self) -> Angle {
        self.polar.angle.clone()
    }
    pub fn real(&self) -> f64 {
        self.cartesian.real
    }
    pub fn imag(&self) -> f64 {
        self.cartesian.imaginary
    }
    pub fn add(&self, z2: &ComplexNumber) -> ComplexNumber {
        let real = self.real() + z2.real();
        let imaginary = self.imag() + z2.imag();
        ComplexNumber::fromCartesian(real, imaginary)
    }
    pub fn sub(&self, z2: &ComplexNumber) -> ComplexNumber {
        let real = self.real() - z2.real();
        let imaginary = self.imag() - z2.imag();
        ComplexNumber::fromCartesian(real, imaginary)
    }
    pub fn mul(&self, z2: &ComplexNumber) -> ComplexNumber {
        let magnitude = self.abs() * z2.abs();
        let angle = Angle::fromRadians(self.angleInRads() + z2.angleInRads());
        ComplexNumber::fromPolar(magnitude, angle)
    }
    pub fn mul_n(&self, n: f64) -> ComplexNumber {
        let magnitude = self.abs() * n;
        ComplexNumber::fromPolar(magnitude, self.angleInAngle())
    }
    pub fn div(&self, z2: &ComplexNumber) -> ComplexNumber {
        let magnitude = self.abs() / z2.abs();
        let angle = Angle::fromRadians(self.angleInRads() - z2.angleInRads());
        ComplexNumber::fromPolar(magnitude, angle)
    }
    pub fn pow(&self, n: f64) -> ComplexNumber {
        let magnitude = self.abs().powf(n);
        let angle = Angle::fromRadians(self.angleInRads() * n);
        ComplexNumber::fromPolar(magnitude, angle)
    }
    pub fn nth_root(&self, n: f64) -> ComplexNumber {
        let magnitude: f64 = self.abs().powf(1.0 / n);
        let angle = Angle::fromRadians(self.angleInRads() / n);
        ComplexNumber::fromPolar(magnitude, angle)
    }
    pub fn ln(&self) -> ComplexNumber {
        let magnitude = self.abs().ln();
        ComplexNumber::fromPolar(magnitude, self.angleInAngle())
    }
    pub fn log10(&self) -> ComplexNumber {
        let magnitude = self.abs().log10();
        ComplexNumber::fromPolar(magnitude, self.angleInAngle())
    }
    pub fn log(&self, arb: f64) -> ComplexNumber {
        let magnitude = self.abs().log(arb);
        ComplexNumber::fromPolar(magnitude, self.angleInAngle())
    }
    pub fn print_cartesian(&self) {
        print!("cartesian form: {} + {} j", self.real(), self.imag());
    }
    pub fn print_polar(&self) {
        println!("polar form (radian): {} e ^ {} ㎭ j", self.abs(), self.angleInRads()); 
        println!("polar form (degree): {} e ^ {}° j", self.abs(), self.angleInDegs()); 
    }
    pub fn print_all_forms(&self){
        self.print_cartesian();
        println!();
        self.print_polar();
    }
}
impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let magAsSqrt = self.polar.magnitude.powi(2);
        writeln!(
            f,
            "
            Pretty Values: 
            Cartesian Form (Pretty): {:.1} + {:.1} j 
            Polar Form (Pretty): √{:.0} e ^ {:.1}㎭ j
            Polar Form (Pretty): √{:.0} e ^ {:.1}° j 
            
            Precision Values:
            Cartesian Form (Pretty): {} + {} j 
            Polar Form (Pretty): {} e ^ {}㎭ j
            Polar Form (Pretty): {} e ^ {}° j ",
            self.cartesian.real,
            self.cartesian.imaginary,
            magAsSqrt,
            self.polar.angle.r.value,
            magAsSqrt,
            self.polar.angle.d.value,
            self.cartesian.real,
            self.cartesian.imaginary,
            self.polar.magnitude,
            self.polar.angle.r.value,
            self.polar.magnitude,
            self.polar.angle.d.value,
        )
    }
}

impl PartialEq for ComplexNumber {
    fn eq(&self, other: &Self) -> bool {
        let c1 = round_millionth(self.abs()) == round_millionth(other.abs());
        let c2 = round_millionth(self.angleInDegs()) == round_millionth(other.angleInDegs());
        let c3 = round_millionth(self.angleInRads()) == round_millionth(other.angleInRads());
        let c4 = round_millionth(self.imag()) == round_millionth(other.imag()); 
        let c5 = round_millionth(self.real()) == round_millionth(other.real()); 

        c1 && c2 && c3 && c4 && c5 
    }
}
fn round_millionth(n:f64) -> f64 {
    let millionth = 1_000_000_f64 ; 
    (n * millionth).round() / millionth
}