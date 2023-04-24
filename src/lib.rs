//! # `complex_numbers` Module
//!
//! This module provides a Complex struct for working with complex numbers in Rust.
//! Complex numbers are numbers that can be expressed in the form a + bi, where a and b are real numbers
//! and i is the imaginary unit (i² = -1). Complex numbers are useful in a wide range of mathematical
//! and engineering applications, including signal processing, control systems, and quantum mechanics.
//!
//! The Complex struct provides methods for common operations on complex numbers, such as addition,
//! subtraction, multiplication, and division. It also includes methods for computing the magnitude,
//! phase (or angle), and conjugate of a complex number.
//!
//! This module aims to provide a comprehensive and efficient solution for working with complex numbers in Rust
//! while maintaining readability and ease of use. Happy number crunching!
//! 
//! # Usage 💡
//! Here's a quick example to get you started:
//! ```
//! use complexible::complex_numbers::*;
//! fn main() {
//!     let a = ComplexNumber::from_cartesian(3.0, 4.0);
//! 
//!     let angle = Angle::from_radians(0.927);
//!     let b = ComplexNumber::from_polar(5.0, angle);
//! 
//!     let sum = a.add(&b);
//!     let product = a.mul(&b); 
//!     
//!     println!("Sum: {}", sum);   
//!     println!("Product: {}", product);
//! }
//! ```
pub mod complex_numbers;
