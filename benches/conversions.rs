#![feature(test)]
extern crate test;

use test::Bencher;

use embedded_semver::prelude::*;

#[bench]
fn bench_to_i32(b: &mut Bencher) {
    let version = Semver::new(1, 1, 5);
    b.iter(|| version.to_i32().unwrap());
}

#[bench]
fn bench_to_i64(b: &mut Bencher) {
    let version = Semver::new(1, 1, 5);
    b.iter(|| version.to_i64().unwrap());
}

#[bench]
fn bench_from_i32(b: &mut Bencher) {
    let val = 83886081;
    b.iter(|| Semver::from_i32(val).unwrap());
}

#[bench]
fn bench_from_i64(b: &mut Bencher) {
    let val = 21474902017;
    b.iter(|| Semver::from_i64(val).unwrap());
}
