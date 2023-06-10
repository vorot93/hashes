#![feature(test)]
extern crate test;

use blake::simd::{x86_64, Machine};
use test::Bencher;

macro_rules! mach_bench {
    ($MachName:ident, $feature:expr, $enable:expr) => {
        #[allow(non_snake_case)]
        #[allow(non_snake_case)]
        #[bench]
        pub fn $MachName(b: &mut Bencher) {
            if !$enable {
                return;
            }
            let m = unsafe { x86_64::$MachName::instance() };
            let mut state = blake::Compressor::default();
            let input = [0; 128];
            #[target_feature(enable = $feature)]
            unsafe fn runner<M: Machine>(m: M, state: &mut blake::Compressor, input: &[u8; 128]) {
                for _ in 0..10240 / (std::mem::size_of::<blake::Compressor>() * 4) {
                    blake::u32x4::put_block(m, state, std::mem::transmute(input), (0, 0));
                }
            }
            b.iter(|| unsafe { runner(m, &mut state, &input) });
            b.bytes = 10240;
        }
    };
}

mach_bench!(SSE2, "sse2", is_x86_feature_detected!("sse2"));
mach_bench!(SSSE3, "ssse3", is_x86_feature_detected!("ssse3"));
mach_bench!(SSE41, "sse4.1", is_x86_feature_detected!("sse4.1"));
mach_bench!(AVX, "avx", is_x86_feature_detected!("avx"));
mach_bench!(AVX2, "avx2", is_x86_feature_detected!("avx2"));
