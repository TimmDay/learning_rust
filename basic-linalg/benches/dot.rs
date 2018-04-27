#![feature(test)]

extern crate basic_linalg;
extern crate rand;
extern crate test;

use std::iter;

use basic_linalg::dot::dot;
use rand::{weak_rng, Rng};
use test::{black_box, Bencher};

fn random_vec<R>(rng: &mut R, len: usize) -> Vec<f32>
where
    R: Rng,
{
    iter::repeat(()).map(|()| rng.gen()).take(len).collect()
}

#[bench]
fn dot_bench(b: &mut Bencher) {
    let mut rng = weak_rng();
    let v1 = black_box(random_vec(&mut rng, 500));
    let v2 = black_box(random_vec(&mut rng, 500));

    b.iter(|| {
        black_box(dot(&v1, &v2));
    })
}
