use std::f64::consts::PI;

/// Represents an angle value in radians.
///
/// # Fields
///
/// * `value` - The value of the angle in radians.
#[derive(Debug)]
pub struct Radian {
    pub value: f64,
}
impl Radian {
    /// Creates a new `Radian` value from a given angle value in radians.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the angle in radians.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::angle::* ;
    ///
    /// let radian = Radian::from(std::f64::consts::PI);
    /// assert_eq!(radian.value, std::f64::consts::PI);
    /// ```
    pub fn from(value: f64) -> Radian {
        Radian { value }
    } 
    /// Converts the `Radian` value to degrees.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::angle::* ;
    ///
    /// let radian = Radian { value: std::f64::consts::PI };
    /// let degree = radian.to_degrees();
    /// assert_eq!(degree.value, 180.0);
    /// ```
    pub fn to_degrees(&self) -> Degree {
        radianto_degrees(self.value)
    }
}

/// Represents an angle value in degrees.
///
/// # Fields
///
/// * `value` - The value of the angle in degrees.
#[derive(Debug)]
pub struct Degree {
    pub value: f64,
}
impl Degree {
     /// Creates a new `Degree` value from a given angle value in degrees.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the angle in degrees.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::angle::* ;
    ///
    /// let degree = Degree::from(180.0);
    /// assert_eq!(degree.value, 180.0);
    /// ```
    pub fn from(value: f64) -> Degree {
        Degree { value }
    } 

    /// Converts the `Degree` value to radians.
    ///
    /// # Example
    ///
    /// ```
    /// use complexible::complex_numbers::angle::* ;
    ///
    /// let degree = Degree { value: 180.0 };
    /// let radian = degree.to_radians();
    /// assert_eq!(radian.value, std::f64::consts::PI);
    /// ```
    pub fn to_radians(&self) -> Radian {
        degreesto_radians(self.value)
    }
}

/// Converts degrees to radians.
///
/// # Arguments
///
/// * `d` - The value in degrees.
///
/// # Example
///
/// ```  
/// use complexible::complex_numbers::angle::* ;
/// let radian = degreesto_radians(180.0);
/// assert_eq!(radian.value, std::f64::consts::PI);
/// ```
pub fn degreesto_radians(d: f64) -> Radian {
    let value = d * (PI / 180.0);
    Radian { value }
}

/// Converts radians to degrees.
///
/// # Arguments
///
/// * `r` - The value in radians.
///
/// # Example
///
/// ```
/// use complexible::complex_numbers::angle::* ;
/// let degree = radianto_degrees(std::f64::consts::PI);
/// assert_eq!(degree.value, 180.0);
/// ```
pub fn radianto_degrees(r: f64) -> Degree {
    let value = r * (180.0 / PI);
    Degree { value }
}
 
