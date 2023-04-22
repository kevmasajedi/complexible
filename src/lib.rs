pub mod complex_numbers;

mod tests {
    use super::complex_numbers::* ; 
    use std::f64::consts::PI ;

    #[test] 
    fn test_angle_constructors() {
        let radianAngle = Angle::fromRadians(PI / 4.0) ; 
        let degreeAngle = Angle::fromDegrees(45.0); 
        assert_eq!(
            radianAngle.d.value, 
            degreeAngle.d.value
        );
        assert_eq!(
            radianAngle.r.value,
            degreeAngle.r.value
        );
    }

    #[test]
    fn test_complex_number_constructors() {
        let cart = ComplexNumber::fromCartesian(1.0, 1.0); 
        let pol = ComplexNumber::fromPolar((2.0_f64).sqrt(), Angle::fromDegrees(45.0));
        assert_eq!(
            cart.abs(),
            pol.abs()
        );
        assert_eq!(
            cart.angleInDegs(),
            pol.angleInDegs(),
        );
        assert_eq!(
            cart.angleInRads(),
            pol.angleInRads()
        );
        assert_eq!(
            cart.real(),
            round_to_one_in_billion(pol.real())
        );
        assert_eq!(
            cart.imag(),
            round_to_one_in_billion(pol.imag())
        )
    }

    #[test]
    fn test_complex_add() {
        assert_eq!(
            zc1().add(&zc2()),
            ComplexNumber::fromCartesian(3.0, 5.0)
        );
        assert_eq!(
            zp1().add(&zp2()),
            ComplexNumber::fromCartesian(3.0, 5.0)
        );
    }

    #[test]
    fn test_complex_sub() {
        assert_eq!(
            zc1().sub(&zc2()),
            ComplexNumber::fromCartesian(1.0, 1.0)
        );
        assert_eq!(
            zp1().sub(&zp2()),
            ComplexNumber::fromCartesian(1.0, 1.0)
        );
    }

    #[test]
    fn test_complex_mul() {
        assert_eq!(
            zc1().mul(&zc2()),
            ComplexNumber::fromPolar(8.06225774829855, Angle::fromDegrees(119.74488129694))
        );
        assert_eq!(
            zp1().mul(&zp2()),
            ComplexNumber::fromPolar(8.06225774829855, Angle::fromDegrees(119.74488129694))
        );
    }

    
    #[test]
    fn test_complex_div() {
        assert_eq!(
            zc1().div(&zc2()),
            ComplexNumber::fromPolar(1.61245154965971, Angle::fromDegrees(-7.12501634890201))
        );
        assert_eq!(
            zp1().div(&zp2()),
            ComplexNumber::fromPolar(1.61245154965971, Angle::fromDegrees(-7.12501634890201))
        );
    }

    #[test]
    fn test_complex_pow() {
        assert_eq!(
            zc1().pow(3.0), 
            ComplexNumber::fromPolar(46.872166581031, Angle::fromDegrees(168.92979742206))
        );
        assert_eq!(
            zp2().pow(2.0), 
            ComplexNumber::fromPolar(5.0, Angle::fromDegrees(126.86989764584))
        )
    }

    #[test]
    fn test_complex_nth_root() {
        assert_eq!(
            zc1().nth_root(3.0), 
            ComplexNumber::fromPolar(1.53340623702, Angle::fromDegrees(18.7699774913))
        );
        assert_eq!(
            zp2().nth_root(2.0), 
            ComplexNumber::fromPolar(1.49534878122, Angle::fromDegrees(31.7174744115))
        )
    }

    #[test]
    fn test_complex_ln() {
        assert_eq!(
            zc1().ln(), 
            ComplexNumber::fromPolar(1.28247467873, Angle::fromDegrees(56.309932474020215))
        );
        assert_eq!(
            zp2().ln(), 
            ComplexNumber::fromPolar(0.80471895621, Angle::fromDegrees(63.43494882292201))
        )
    }

    #[test]
    fn test_complex_log10() {
        assert_eq!(
            zc1().log10(), 
            ComplexNumber::fromPolar(0.55697167615, Angle::fromDegrees(56.309932474020215))
        );
        assert_eq!(
            zp2().log10(), 
            ComplexNumber::fromPolar(0.34948500216, Angle::fromDegrees(63.43494882292201))
        )
    }

    #[test]
    fn test_complex_log() {
        assert_eq!(
            zc1().log(3.0), 
            ComplexNumber::fromPolar(1.1673587597364, Angle::fromDegrees(56.309932474020215))
        );
        assert_eq!(
            zp2().log(5.0), 
            ComplexNumber::fromPolar(0.5, Angle::fromDegrees(63.43494882292201))
        )
    }

    fn round_to_one_in_billion(n:f64) -> f64 {
        let billion = 1_000_000_000_f64 ; 
        (n * billion).round() / billion
    }
    fn zc1() -> ComplexNumber {
        ComplexNumber::fromCartesian(2.0, 3.0)
    }
    fn zc2() -> ComplexNumber {
        ComplexNumber::fromCartesian(1.0, 2.0)
    }
    fn zp1() -> ComplexNumber {
        ComplexNumber::fromPolar(3.605551275463989, Angle::fromDegrees(56.309932474020215))
    }
    fn zp2() -> ComplexNumber {
        ComplexNumber::fromPolar(2.23606797749979, Angle::fromDegrees(63.43494882292201))
    }
}