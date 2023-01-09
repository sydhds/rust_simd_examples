use std::arch::x86_64::*; //import the intel intrinsics

fn main() {

    // Overview
    // https://en.wikipedia.org/wiki/Advanced_Vector_Extensions


    // AVX2
    let ret = unsafe {
        let avx: __m256i = _mm256_set1_epi32(2);
        // avx is now [2,2,2,2,2,2,2,2];
        avx
    };

    println!("ret: {:?}", ret);
}