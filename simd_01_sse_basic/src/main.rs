#![feature(portable_simd)]

use std::arch::x86_64::*;
use core::simd::Simd;
use std::mem::transmute;

fn main() {

    // Overview SSE
    // https://en.wikipedia.org/wiki/Streaming_SIMD_Extensions

    // Rust doc
    // https://doc.rust-lang.org/core/arch/x86_64/index.html#functions
    // Specific:
    // https://doc.rust-lang.org/core/arch/x86_64/fn._mm_add_ps.html
    // Intel documentation:
    // https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_add_ps

    let s = unsafe {

        // __m128: Four f32 types

        let a0: __m128 = _mm_setzero_ps();
        // a0 is now: [0.0, 0.0, 0.0, 0.0]
        let a: __m128 = _mm_set1_ps(2.0);
        // a is now [ 2.0, 2.0, 2.0, 2.0 ]
        let b = _mm_set_ps(1.0, 2.0, 3.0, 4.0);
        // b is now [ 4.0, 3.0, 2.0, 1.0 ] reverse!
        let r = _mm_add_ps(a, b);
        // r is now [ 6.0, 5.0, 4.0, 3.0]
        r
    };

    println!("s: {:?}", s);

    // https://doc.rust-lang.org/core/simd/struct.Simd.html
    let all_floats: Simd<f32, 4> = s.into();
    println!("s: {:?} {:?} {:?} {:?}", all_floats[0], all_floats[1], all_floats[2], all_floats[3]);

    let all_floats_2: &[f32; 4] = all_floats.as_array();
    println!("s: {:?}", all_floats_2);

    let all_floats_3 = unsafe {
        transmute::<__m128, [f32;4]>(s);
    };

    println!("s: {:?}", all_floats_3);
}