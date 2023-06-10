#![feature(test)]
extern crate test;

use blake::Blake256;
use digest::bench_update;
use test::Bencher;

bench_update!(
    Blake256::default();
    blake_256_10 10;
    blake_256_100 100;
    blake_256_1000 1000;
    blake_256_10000 10000;
);
