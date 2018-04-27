pub fn mul(u: &[f32], v: &[f32]) -> Vec<f32> {
    assert_eq!(u.len(), v.len());

    let mut w : Vec<f32> = vec![];

    for x in 0..u.len() {
        w.push(u[x]*v[x]);
    }
    w
}

pub fn mul_inplace(u: &mut [f32], v: &[f32]) {
    assert_eq!(u.len(), v.len());

    for x in 0..u.len() {
        u[x] = u[x] * v[x];
    }
}

#[cfg(test)]
mod tests {
    use super::{mul, mul_inplace};

    #[test]
    fn mul_test() {
        let u = [1f32, -2f32, -3f32];
        let v = [2f32, 4f32, -2f32];

        assert_eq!(&mul(&u, &v), &[2f32, -8f32, 6f32]);
    }

    #[test]
    fn mul_inplace_test() {
        let mut u = [1f32, -2f32, -3f32];
        let v = [2f32, 4f32, -2f32];

        mul_inplace(&mut u, &v);

        assert_eq!(&u, &[2f32, -8f32, 6f32]);
    }
}
