# Complexible: A Rust Library for Complex-Number Operations
Complexible is a minimal and user-friendly Rust library for complex-number operations, bringing the joy of complex mathematics to Rustaceans everywhere! Designed with simplicity and ease-of-use in mind, Complexible offers a range of functionality to help you work with complex numbers in your Rust projects.

## Features
* Complex numbers support:
    * Representation of complex numbers in Cartesian (`CartesianComplexNumber`) and polar (`PolarComplexNumber`) forms
    * Conversion between Cartesian and polar forms
    * Creation of complex numbers from real numbers
* Angle representation:
    * Angle representation using `Angle`, `Radian`, and `Degree` structs
* Basic arithmetic operations on complex numbers:
    * Addition
    * Subtraction
    * Multiplication (complex and scalar)
    * Division
* Complex number properties and operations:
    * Real part
    * Imaginary part
    * Magnitude (absolute value)
    * Angle (argument) in radians and degrees
    * Natural logarithm (ln)
    * Logarithm with arbitrary base
    * Logarithm base 10
    * Power (exponentiation)
    * Nth root

## Installation
Add Complexible to your Cargo.toml:

```
[dependencies]
complexible = "0.2.0"
```

## Usage
Here's a quick example to get you started:

```
use complexible::complex_numbers::*;

fn main() {
    let a = ComplexNumber::from_cartesian(3.0, 4.0);
    let b = ComplexNumber::from_polar(5.0, 0.927);

    let sum = a + b;
    let product = a * b;

    println!("Sum: {}", sum);
    println!("Product: {}", product);
}
```
For more examples and detailed documentation, please refer to the [API documentation](https://docs.rs/complexible).

## Contributing
We welcome contributions! If you'd like to help improve Complexible, feel free to submit a pull request on __GitHub__. Please make sure to follow the existing coding style and add tests for any new features or bug fixes.

## License
Complexible is released under the MIT license.