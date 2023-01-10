# rust_simd_examples

## Build & run

* cargo build
* cargo run --bin simd_01_sse_basic

## Examples

* simd_01_sse_basic: basic vector addition
* simd_02_avx_basic: basic avx instruction
* simd_03_intrinsics:
  * cd simd_03_intrinsics
    * RUSTFLAGS='-C target-feature=+sse' cargo build && objdump -d ../target/debug/simd_03_intrinsics | grep roundss
    * RUSTFLAGS='-C target-feature=+avx' cargo build && objdump -d ../target/debug/simd_03_intrinsics | grep vroundss
