pub fn dot(u: &[f32], v: &[f32]) -> f32 {
    assert_eq!(u.len(), v.len());

    let mut dp : f32 = 0.0;
    for x in 0..u.len() {
        dp += u[x] * v[x];
    }
    dp
}

#[cfg(test)]
mod tests {
    use super::dot;

    #[test]
    fn dot_test() {
        let u = [1f32, -2f32, -3f32];
        let v = [2f32, 4f32, -2f32];
        let w = [-1f32, 3f32, 2.5f32];

        assert_approx_eq!(dot(&u, &v), 0f32);
        assert_approx_eq!(dot(&u, &w), -14.5f32);
        assert_approx_eq!(dot(&v, &w), 5f32);
    }
}
