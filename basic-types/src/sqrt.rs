use std::f32;

pub fn sqrt(v: f32) -> f32 {

    if v == 0.0 {
        return 0.0;
    } else if v < 0.0 {
        return f32::NAN;
    }
    let mut result : f32 = v / 2.0;  //initial guess

    for i in 0..5 {
        result = result - ((result*result - v) / (2.0*result));
    }
    result
}

#[cfg(test)]
mod tests {
    use std::f32;

    use super::sqrt;

    const TOLERANCE: f32 = 1e-4;

    quickcheck! {
        fn sqrt_test(v: f32) -> bool {
            if v < 0f32 {
                sqrt(v).is_nan()
            } else if v.is_infinite() {
                sqrt(v).is_infinite()
            } else {
                // Check against sqrt from the standard library.
                (sqrt(v) - v.sqrt()).abs() < TOLERANCE
            }
        }
    }
}
