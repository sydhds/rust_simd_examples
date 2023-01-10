use std::arch::x86_64::*; //import the intel intrinsics

fn main() {

    // Overview
    // According to https://www.felixcloutier.com/x86/roundss
    // There is 2 instructions for rounding:
    // SSE4_2: roundss
    // AVX: vroundss
    //
    // By using compile flags, the intrinsic is turned into different instructions
    // RUSTFLAGS='-C target-feature=+sse' cargo build && objdump -d ../target/debug/simd_03_intrinsics | grep roundss
    // RUSTFLAGS='-C target-feature=+avx' cargo build && objdump -d ../target/debug/simd_03_intrinsics | grep vroundss

    let (res, f1, f2) = unsafe {
        let f1: __m128 = _mm_set1_ps(3.45);
        let f2: __m128 = _mm_set1_ps(1.987);
        const ROUNDING_MODE: i32 = _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC;
        let r = _mm_round_ss::<{ ROUNDING_MODE }>(f1, f2);
        (r, f1, f2)
    };
    println!("res: {:?} {:?} {:?}", res, f1, f2);
}