#![feature(test)]
extern crate test;

use blake::Blake512;
use digest::bench_update;
use test::Bencher;

bench_update!(
    Blake512::default();
    blake_512_10 10;
    blake_512_100 100;
    blake_512_1000 1000;
    blake_512_10000 10000;
);
