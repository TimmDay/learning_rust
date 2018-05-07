pub fn mul(u: &[f32], v: &[f32]) -> Vec<f32> {
    assert_eq!(u.len(), v.len());


    let mut w : Vec<f32> = vec![0f32; u.len()]; //vec![0f32, u.len()] sets the length correctly and initialises to 0
    //let mut product = Vec::with_capacity(u.len()); //slower way
    // the length of product keeps getting modified as we push to it
    // why is this expensive? speculative execution, it cannot go ahead and do iterations in parrallel because there is
    //


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

pub fn mul_fast(u: &mut [f32], v: &[f32]){
    assert_eq!(u.len(), v.len());

    for i in 0..u.len() {
        u[i] *= v[i];
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
