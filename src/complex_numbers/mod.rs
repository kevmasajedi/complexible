pub mod angle;
pub mod z;

use angle::*;
use std::fmt;
use z::*;

/// Represents an angle, stored in both degrees and radians.
///
/// # Fields
///
/// * `d` - The angle value in degrees, represented as a `Degree` struct.
/// * `r` - The angle value in radians, represented as a `Radian` struct.
#[derive(Debug)]
pub struct Angle {
    pub d: Degree,
    pub r: Radian,
}
impl Angle {
    /// Creates a new `Angle` value from a given angle value in degrees.
    ///
    /// # Arguments
    ///
    /// * `d` - The value of the angle in degrees.
    ///
    /// # Example
    ///
    /// ```  
    /// use complexible::complex_numbers::*;
    /// let angle = Angle::from_degrees(45.0);
    /// ```
    pub fn from_degrees(d: f64) -> Angle {
        let d = Degree::from(d);
        let r = d.to_radians();
        Angle { d, r }
    }

    /// Creates a new `Angle` value from a given angle value in radians.
    ///
    /// # Arguments
    ///
    /// * `r` - The value of the angle in radians.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    /// let angle = Angle::from_radians(std::f64::consts::PI);
    /// ```
    pub fn from_radians(r: f64) -> Angle {
        let r = Radian::from(r);
        let d = r.to_degrees();
        Angle { d, r }
    }
}
impl Clone for Angle {
    fn clone(&self) -> Angle {
        Angle::from_degrees(self.d.value)
    }
}

/// Represents a complex number in both Cartesian and polar form.
///
/// # Fields
///
/// * `cartesian` - The complex number in Cartesian form, represented as a `CartesianComplexNumber` struct.
/// * `polar` - The complex number in polar form, represented as a `PolarComplexNumber` struct.
#[derive(Debug)]
pub struct ComplexNumber {
    cartesian: CartesianComplexNumber,
    polar: PolarComplexNumber,
}

impl ComplexNumber {
    /// Creates a new `ComplexNumber` from its Cartesian coordinates.
    ///
    /// # Arguments
    ///
    /// * `real` - The real part of the complex number.
    /// * `imaginary` - The imaginary part of the complex number.
    ///
    /// # Example
    ///
    /// ```  
    /// use complexible::complex_numbers::*;
    /// let complex = ComplexNumber::from_cartesian(1.0, 1.0); //1 + 1 J
    /// ```
    pub fn from_cartesian(real: f64, imaginary: f64) -> ComplexNumber {
        let cartesian = CartesianComplexNumber { real, imaginary };
        let polar = cartesian.to_polar();
        ComplexNumber { cartesian, polar }
    }

    /// Creates a new `ComplexNumber` from its polar coordinates.
    ///
    /// # Arguments
    ///
    /// * `magnitude` - The magnitude (or absolute value) of the complex number.
    /// * `angle` - The angle of the complex number, represented as an `Angle` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let angle = Angle::from_degrees(45.0);
    /// let complex = ComplexNumber::from_polar(1.0, angle);
    /// ```
    pub fn from_polar(magnitude: f64, angle: Angle) -> ComplexNumber {
        let polar = PolarComplexNumber { magnitude, angle };
        let cartesian = polar.to_cartesian();
        ComplexNumber { cartesian, polar }
    }

    /// Creates a new `ComplexNumber` from a real number.
    ///
    /// # Arguments
    ///
    /// * `real` - The real part of the complex number.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let complex = ComplexNumber::from_real(1.0);
    /// ```
    pub fn from_real(real: f64) -> ComplexNumber {
        ComplexNumber::from_cartesian(real, 0.0)
    }

    /// Returns the absolute value (or magnitude) of the complex number.
    ///
    /// # Returns
    ///
    /// The absolute value (or magnitude) of the complex number as a `f64`.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let complex = ComplexNumber::from_cartesian(3.0, 4.0);
    /// assert_eq!(complex.abs(), 5.0);
    /// ```
    pub fn abs(&self) -> f64 {
        self.polar.magnitude
    }

    /// Returns the angle (in radians) of the complex number.
    ///
    /// # Returns
    ///
    /// The angle (in radians) of the complex number as a `f64`.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let complex = ComplexNumber::from_cartesian(1.0, 1.0);
    /// assert_eq!(complex.angle_in_rads(), 0.7854);
    /// ```
    pub fn angle_in_rads(&self) -> f64 {
        round_five_zeros(self.polar.angle.r.value)
    }

    /// Returns the angle (in degrees) of the complex number.
    ///
    /// # Returns
    ///
    /// The angle (in degrees) of the complex number as a `f64`.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let complex = ComplexNumber::from_cartesian(1.0, 1.0);
    /// assert_eq!(complex.angle_in_degs(), 45.0);
    /// ```
    pub fn angle_in_degs(&self) -> f64 {
        round_three_zeros(self.polar.angle.d.value)
    }

    /// Returns the angle of the complex number as an `Angle` struct.
    ///
    /// # Returns
    ///
    /// An `Angle` struct representing the angle of the complex number.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let complex = ComplexNumber::from_cartesian(1.0, 1.0);
    /// let angle = complex.angle_in_angle();
    /// assert_eq!(angle.d.value, 45.0);
    /// ```
    pub fn angle_in_angle(&self) -> Angle {
        self.polar.angle.clone()
    }

    /// Returns the real part of the complex number.
    ///
    /// # Returns
    ///
    /// The real part of the complex number as a `f64`.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let complex = ComplexNumber::from_cartesian(3.0, 4.0);
    /// assert_eq!(complex.real(), 3.0);
    /// ```
    pub fn real(&self) -> f64 {
        self.cartesian.real
    }

    /// Returns the imaginary part of the complex number.
    ///
    /// # Returns
    ///
    /// The imaginary part of the complex number as a `f64`.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let complex = ComplexNumber::from_cartesian(3.0, 4.0);
    /// assert_eq!(complex.imag(), 4.0);
    /// ```
    pub fn imag(&self) -> f64 {
        self.cartesian.imaginary
    }

    /// Adds the given complex number to this complex number.
    ///
    /// # Arguments
    ///
    /// * `z2` - A reference to the complex number to add to this complex number.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the result of adding the two complex numbers.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_cartesian(1.0, 2.0);
    /// let z2 = ComplexNumber::from_cartesian(3.0, 4.0);
    /// let result = z1.add(&z2);
    /// assert_eq!(result.real(), 4.0);
    /// assert_eq!(result.imag(), 6.0);
    /// ```
    pub fn add(&self, z2: &ComplexNumber) -> ComplexNumber {
        let real = self.real() + z2.real();
        let imaginary = self.imag() + z2.imag();
        ComplexNumber::from_cartesian(real, imaginary)
    }

    /// Subtracts the given complex number from this complex number.
    ///
    /// # Arguments
    ///
    /// * `z2` - A reference to the complex number to subtract from this complex number.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the result of subtracting the two complex numbers.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_cartesian(3.0, 4.0);
    /// let z2 = ComplexNumber::from_cartesian(1.0, 2.0);
    /// let result = z1.sub(&z2);
    /// assert_eq!(result.real(), 2.0);
    /// assert_eq!(result.imag(), 2.0);
    /// ```
    pub fn sub(&self, z2: &ComplexNumber) -> ComplexNumber {
        let real = self.real() - z2.real();
        let imaginary = self.imag() - z2.imag();
        ComplexNumber::from_cartesian(real, imaginary)
    }

    /// Multiplies this complex number by the given complex number.
    ///
    /// # Arguments
    ///
    /// * `z2` - A reference to the complex number to multiply by.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the result of multiplying the two complex numbers.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_polar(2.0, Angle::from_degrees(30.0));
    /// let z2 = ComplexNumber::from_polar(3.0, Angle::from_degrees(45.0));
    /// let result = z1.mul(&z2);
    /// assert_eq!(result.abs(), 6.0);
    /// assert_eq!(result.angle_in_degs(), 75.0);
    /// ```
    pub fn mul(&self, z2: &ComplexNumber) -> ComplexNumber {
        let magnitude = self.abs() * z2.abs();
        let angle = Angle::from_radians(self.angle_in_rads() + z2.angle_in_rads());
        ComplexNumber::from_polar(magnitude, angle)
    }

    /// Multiplies this complex number by the given scalar value.
    ///
    /// # Arguments
    ///
    /// * `n` - The scalar value to multiply by.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the result of multiplying this complex number by the scalar value.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_polar(2.0, Angle::from_degrees(30.0));
    /// let result = z1.mul_n(3.0);
    /// assert_eq!(result.abs(), 6.0);
    /// assert_eq!(result.angle_in_degs(), 30.0);
    /// ```
    pub fn mul_n(&self, n: f64) -> ComplexNumber {
        let magnitude = self.abs() * n;
        ComplexNumber::from_polar(magnitude, self.angle_in_angle())
    }

    /// Divides this complex number by the given complex number.
    ///
    /// # Arguments
    ///
    /// * `z2` - A reference to the complex number to divide by.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the result of dividing this complex number by the given complex number.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_polar(2.0, Angle::from_degrees(30.0));
    /// let z2 = ComplexNumber::from_polar(3.0, Angle::from_degrees(45.0));
    /// let result = z1.div(&z2);
    /// assert_eq!(result.abs(), 2.0/3.0);
    /// assert_eq!(result.angle_in_degs(), -15.0);
    /// ```
    pub fn div(&self, z2: &ComplexNumber) -> ComplexNumber {
        let magnitude = self.abs() / z2.abs();
        let angle = Angle::from_radians(self.angle_in_rads() - z2.angle_in_rads());
        ComplexNumber::from_polar(magnitude, angle)
    }

    /// Raises this complex number to the given power.
    ///
    /// # Arguments
    ///
    /// * `n` - The power to raise this complex number to.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the result of raising this complex number to the given power.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_polar(2.0, Angle::from_degrees(30.0));
    /// let result = z1.pow(2.0);
    /// assert_eq!(result.abs(), 4.0);
    /// assert_eq!(result.angle_in_degs(), 60.0);
    /// ```
    pub fn pow(&self, n: f64) -> ComplexNumber {
        let magnitude = self.abs().powf(n);
        let angle = Angle::from_radians(self.angle_in_rads() * n);
        ComplexNumber::from_polar(magnitude, angle)
    }

    /// Calculates the nth root of this complex number.
    ///
    /// # Arguments
    ///
    /// * `n` - The root to calculate.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the result of taking the nth root of this complex number.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_polar(2.0, Angle::from_degrees(30.0));
    /// let result = z1.nth_root(2.0);
    /// assert_eq!(result.abs(), 1.4142135623730951);
    /// assert_eq!(result.angle_in_degs(), 15.0);
    /// ```
    pub fn nth_root(&self, n: f64) -> ComplexNumber {
        let magnitude: f64 = self.abs().powf(1.0 / n);
        let angle = Angle::from_radians(self.angle_in_rads() / n);
        ComplexNumber::from_polar(magnitude, angle)
    }

    /// Calculates the natural logarithm of this complex number.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the natural logarithm of this complex number.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_polar(2.0, Angle::from_degrees(30.0));
    /// let result = z1.ln();
    /// assert_eq!(result.real(), 0.6002830669264718);
    /// assert_eq!(result.imag(), 0.3465735902799726);
    /// ```
    ///
    pub fn ln(&self) -> ComplexNumber {
        let magnitude = self.abs().ln();
        ComplexNumber::from_polar(magnitude, self.angle_in_angle())
    }
    /// Calculates the base-10 logarithm of this complex number.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the base-10 logarithm of this complex number.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_polar(2.0, Angle::from_degrees(30.0));
    /// let result = z1.log10();
    /// assert_eq!(result.real(), 0.26069962354612713);
    /// assert_eq!(result.imag(), 0.15051499783199057);
    /// ```
    pub fn log10(&self) -> ComplexNumber {
        let magnitude = self.abs().log10();
        ComplexNumber::from_polar(magnitude, self.angle_in_angle())
    }
    /// Calculates the logarithm of this complex number with respect to an arbitrary base.
    ///
    /// # Arguments
    ///
    /// * `arb` - The base to calculate the logarithm with respect to.
    ///
    /// # Returns
    ///
    /// A new `ComplexNumber` object that represents the logarithm of this complex number with respect to the given base.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::*;
    ///
    /// let z1 = ComplexNumber::from_polar(2.0, Angle::from_degrees(30.0));
    /// let result = z1.log(2.0);
    /// assert_eq!(result.real(), 0.8660254037844387);
    /// assert_eq!(result.imag(), 0.49999999999999994);
    /// ```
    pub fn log(&self, arb: f64) -> ComplexNumber {
        let magnitude = self.abs().log(arb);
        ComplexNumber::from_polar(magnitude, self.angle_in_angle())
    }


    pub fn print_cartesian(&self) {
        print!("cartesian form: {} + {} j", self.real(), self.imag());
    }
    pub fn print_polar(&self) {
        println!(
            "polar form (radian): {} e ^ {} ㎭ j",
            self.abs(),
            self.angle_in_rads()
        );
        println!(
            "polar form (degree): {} e ^ {}° j",
            self.abs(),
            self.angle_in_degs()
        );
    } 
}
impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mag_as_sqrt = self.polar.magnitude.powi(2);
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
            mag_as_sqrt,
            self.polar.angle.r.value,
            mag_as_sqrt,
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
        let c1 = round_five_zeros(self.abs()) == round_five_zeros(other.abs());
        let c2 = round_five_zeros(self.angle_in_degs()) == round_five_zeros(other.angle_in_degs());
        let c3 = round_five_zeros(self.angle_in_rads()) == round_five_zeros(other.angle_in_rads());
        let c4 = round_five_zeros(self.imag()) == round_five_zeros(other.imag());
        let c5 = round_five_zeros(self.real()) == round_five_zeros(other.real());

        c1 && c2 && c3 && c4 && c5
    }
}
fn round_five_zeros(n: f64) -> f64 {
    let five_zeros = 1_00000_f64;
    (n * five_zeros).round() / five_zeros
}
fn round_three_zeros(n: f64) -> f64 {
    let three_zeros = 1_000_f64;
    (n * three_zeros).round() / three_zeros
}
