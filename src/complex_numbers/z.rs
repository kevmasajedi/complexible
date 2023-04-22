use super::*;

/// Represents a complex number in Cartesian form.
///
/// # Fields
///
/// * `real` - The real part of the complex number.
/// * `imaginary` - The imaginary part of the complex number.
#[derive(Debug)]
pub struct CartesianComplexNumber {
    pub real: f64,
    pub imaginary: f64,
}
impl CartesianComplexNumber {
    /// Creates a new `CartesianComplexNumber` with the given real and imaginary parts.
    ///
    /// # Arguments
    ///
    /// * `real` - The real part of the complex number.
    /// * `imaginary` - The imaginary part of the complex number.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::z::CartesianComplexNumber;
    ///
    /// let complex = CartesianComplexNumber::new(1.0, 2.0);
    /// assert_eq!(complex.real, 1.0);
    /// assert_eq!(complex.imaginary, 2.0);
    /// ```
    pub fn new(real: f64, imaginary: f64) -> CartesianComplexNumber {
        CartesianComplexNumber { real, imaginary }
    }
    /// Converts the `CartesianComplexNumber` to polar form.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::z::CartesianComplexNumber;
    ///
    /// let complex = CartesianComplexNumber { real: 1.0, imaginary: 1.0 };
    /// let polar = complex.to_polar();
    /// assert_eq!(polar.magnitude, 1.4142135623730951);
    /// assert_eq!(polar.angle.d.value, 45.0);
    /// ```
    pub fn to_polar(&self) -> PolarComplexNumber {
        let magnitude = (self.real.powi(2) + self.imaginary.powi(2)).sqrt();
        let angle = Angle::from_radians(self.imaginary.atan2(self.real));
        PolarComplexNumber { magnitude, angle }
    }
}


/// Represents a complex number in polar form.
///
/// # Fields
///
/// * `magnitude` - The magnitude of the complex number.
/// * `angle` - The angle of the complex number, represented as an `Angle` struct.
#[derive(Debug)]
pub struct PolarComplexNumber {
    pub magnitude: f64,
    pub angle: Angle,
}
impl PolarComplexNumber {
    /// Creates a new `PolarComplexNumber` with the given magnitude and angle.
    ///
    /// # Arguments
    ///
    /// * `magnitude` - The magnitude of the complex number.
    /// * `angle` - The angle of the complex number, represented as an `Angle` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::{ * , z::*};
    ///
    /// let angle = Angle::from_degrees(45.0);
    /// let complex = PolarComplexNumber::new(1.0, angle);
    /// assert_eq!(complex.magnitude, 1.0);
    /// assert_eq!(complex.angle.d.value, 45.0);
    /// ```
    pub fn new(magnitude: f64, angle: Angle) -> PolarComplexNumber {
        PolarComplexNumber { magnitude, angle }
    }

    /// Converts the `PolarComplexNumber` to Cartesian form.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::{ * , z::*};
    ///
    /// let angle = Angle::from_degrees(45.0);
    /// let polar = PolarComplexNumber { magnitude: 1.0, angle };
    /// let cartesian = polar.to_cartesian();
    /// assert_eq!(cartesian.real, 0.7071067811865476);
    /// assert_eq!(cartesian.imaginary, 0.7071067811865476    );
    /// ```
    pub fn to_cartesian(&self) -> CartesianComplexNumber {
        let r = self.angle.r.value;
        let real = self.magnitude * r.cos();
        let imaginary = self.magnitude * r.sin();
        CartesianComplexNumber { real, imaginary }
    }
}
 