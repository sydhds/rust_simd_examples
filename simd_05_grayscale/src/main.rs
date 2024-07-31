use std::io::{LineWriter, Write};
use std::path::PathBuf;

use std::arch::x86_64::{__m128, __m128i, _mm_and_si128, _mm_loadu_si128, _mm_mulhi_epu16, _mm_packus_epi32, _mm_set1_epi16, _mm_set1_epi32, _mm_setzero_ps, _mm_setzero_si128, _mm_slli_si128, _mm_srli_si128};
use std::intrinsics::transmute;

// Port from original Cpp code:
// https://github.com/Const-me/SimdIntroArticle/tree/master/Grayscale

fn main() {

    // Example of some SSE functions and there results
    test();
    // Fixed-point arithmetic
    test2();

    /*
    let width = 320;
    let height = 240;
    let path_1 = PathBuf::from("image.ppm");
    // let path_2_1 = PathBuf::from("image_gray_std.ppm");
    // let path_2_2 = PathBuf::from("image_gray_rayon.ppm");

    println!("Generating image data...");
    let mut img = generate_image_data(width, height).expect("Unable to generate image data");

    println!("Writing {:?}...", path_1);
    write_image(width, height, &img, &path_1)
        .unwrap_or_else(|_| panic!("Unable to write image {:?}", path_1));
    let mut img2 = img.clone();
    */
}

fn generate_image_data(width: u16, height: u16) -> anyhow::Result<Vec<u8>> {
    // let data_size = width * height * 3;
    let data_size = usize::try_from(u32::from(width) * u32::from(height) * 3)?;
    eprintln!("Image size: {}", data_size);
    let mut data: Vec<u8> = Vec::with_capacity(data_size);

    for j in 0..height {
        for i in 0..width {
            let r = f64::from(i) / (f64::from(width) - 1.0);
            let g = f64::from(j) / (f64::from(height) - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            data.push(ir);
            data.push(ig);
            data.push(ib);
        }
    }

    Ok(data)
}

fn write_image(width: u16, height: u16, data: &[u8], path: &PathBuf) -> anyhow::Result<()> {
    let file = std::fs::File::create(path)?;
    let mut writer = LineWriter::new(file);

    let color_count = 3;
    let line_size = u32::from(width) * color_count;

    writer.write_all(b"P3\n")?;
    writeln!(writer, "{} {}", width, height)?;
    writer.write_all(b"255\n")?;

    for j in 0..height {
        for i in 0..width {
            let index_ = (u32::from(j) * line_size) + (u32::from(i) * 3);
            let index = usize::try_from(index_)?;

            let r = data[index];
            let g = data[index + 1];
            let b = data[index + 2];
            writeln!(writer, "{} {} {}", r, g, b)?;
        }
    }

    Ok(())
}

fn test() {

    // 1. Init a __m128i
    let r: __m128i = unsafe { _mm_setzero_si128() }; // r = 0

    // 2. Broadcast (aka set) a i32 in a __m128i
    // https://doc.rust-lang.org/core/arch/x86/fn._mm_set1_epi32.html
    let r2_: __m128i = unsafe {
        _mm_set1_epi32(0x1122AABB)
    };
    // Test
    let r2 = unsafe { transmute::<__m128i, [u8; 16]>(r2_) };
    println!("r2: {:x?}", r2);
    assert_eq!(r2, [
        0xBB, 0xAA, 0x22, 0x11,
        0xBB, 0xAA, 0x22, 0x11,
        0xBB, 0xAA, 0x22, 0x11,
        0xBB, 0xAA, 0x22, 0x11
    ]);

    // 3. _mm_loadu_si128
    // https://doc.rust-lang.org/core/arch/x86/fn._mm_loadu_si128.html
    let v1: [u8; 16] = [
        0, 1, 2, 3,
        4, 5, 6, 7,
        9, 9, 10, 11,
        12, 13, 14, 15
    ];
    let a = unsafe {
        // [u8; 16] -> __m128i
        let v1_ = transmute::<[u8; 16], __m128i>(v1);
        // Get pointer
        let ptr =&v1_ as *const __m128i;
        // Load from array
        _mm_loadu_si128(ptr)
    };
    // Test
    let a2 = unsafe { transmute::<__m128i, [u8; 16]>(a) };
    println!("a2: {:?}", a2);
    assert_eq!(v1, a2);

    // 4.  _mm_packus_epi32
    // https://doc.rust-lang.org/core/arch/x86/fn._mm_packus_epi32.html
    let v1: [u8; 16] = [
        255, 1, 2, 3, // 0xFF, ...
        254, 4, 5, 6, // 0xFE, ...
        255, 7, 8, 9, // 0xFF, ...
        254, 10, 11, 12 // 0xFE, ...
    ];
    let v2: [u8; 16] = [
        253, 1, 2, 3, // 0xFD, ...
        252, 5, 6, 7, // 0xFC, ...
        251, 9, 10, 11, // 0xFB, ...
        250, 13, 14, 15 // 0xFA, ...
    ];
    let packed_ = unsafe {
        let v1_ = transmute::<[u8; 16], __m128i>(v1);
        // Get pointer
        // let ptr1 =&v1_ as *const __m128i;
        let v2_ = transmute::<[u8; 16], __m128i>(v2);
        // Get pointer
        // let ptr2 =&v2_ as *const __m128i;
        let mask = _mm_set1_epi32(0xFF);
        _mm_packus_epi32(
            _mm_and_si128( v1_, mask ),
            _mm_and_si128( v2_, mask ),
        )
    };
    let packed = unsafe { transmute::<__m128i, [u8; 16]>(packed_) };
    println!("packed: {:x?}", packed);
    assert_eq!(packed, [
        0xFF, 0, 0xFE, 0, 0xFF, 0, 0xFE, 0, // from v1
        0xFD, 0, 0xFC, 0, 0xFB, 0, 0xFA, 0, // from v2
    ]);

    // 5. _mm_slli_si128
    // https://doc.rust-lang.org/core/arch/x86/fn._mm_slli_si128.html
    let packed2_ = unsafe {
        _mm_slli_si128::<1>(packed_)
    };
    let packed2 = unsafe { transmute::<__m128i, [u8; 16]>(packed2_) };
    println!("packed2: {:x?}", packed2);
    assert_eq!(packed2, [
        0, 0xFF, 0, 0xFE, 0, 0xFF, 0, 0xFE, // from v1
        0, 0xFD, 0, 0xFC, 0, 0xFB, 0, 0xFA, // from v2
    ]);
}

fn test2() {
    let _mulRedFloat: f32 = 0.29891;
    let _factor = 0x10000;
    let _mulRed_ = _mulRedFloat * (_factor as f32);
    let _mulRed: u16 = _mulRed_ as u16;
    println!("factor: {}, mulRed_: {}, mulRed: {}", _factor, _mulRed_, _mulRed);

    // TODO: compare float multiplication with u16 multiplication
    
    println!("float multiplication: {}", 1.0f32 * _mulRedFloat);
    println!("u16 multiplication: {}", 0x0001 * mulRed);
    
}   

unsafe fn pack_red(a: __m128i, b: __m128i) -> __m128i {
    // mask: 0xFF, 0x00, 0x00, 0x00, ...
    let mask = _mm_set1_epi32(0xFF);
    // Pack a & b as u16 (so in packed, we only have the red values (from RGBA array) as u16
    let packed = _mm_packus_epi32(
        _mm_and_si128( a, mask ), // Keep only red values from a
        _mm_and_si128( b, mask ) // Keep only read values from b
    );
    // Shift
    _mm_slli_si128::<1>(packed)
}

unsafe fn pack_green(a: __m128i, b: __m128i) -> __m128i {
    let mask = _mm_set1_epi32(0xFF00);
    _mm_packus_epi32(
        _mm_and_si128( a, mask ),
        _mm_and_si128( b, mask )
    )
}

unsafe fn pack_blue(a: __m128i, b: __m128i) -> __m128i {
    let mask = _mm_set1_epi32(0xFF00);
    _mm_packus_epi32(
        _mm_and_si128( _mm_srli_si128::<1>(a), mask ),
        _mm_and_si128(  _mm_srli_si128::<1>(b), mask )
    )
}

// The coefficients to produce the gray values
const mulRedFloat: f32 = 0.29891;
const mulGreenFloat: f32 = 0.58661;
const mulBlueFloat: f32 = 0.11448;

const mulRed: u16 = (mulRedFloat * (0x10000 as f32)) as u16;
const mulGreen: u16 = (mulGreenFloat * (0x10000 as f32)) as u16;
const mulBlue: u16 = (mulBlueFloat * (0x10000 as f32)) as u16;

unsafe fn brightness( r: __m128i, g: __m128i, b: __m128i ) -> __m128i {
    // let r_ = _mm_mulhi_epu16( r, _mm_set1_epi16( mulRed ) );
    // let g_ = _mm_mulhi_epu16( g, _mm_set1_epi16( mulGreen ) );
    // let b_ = _mm_mulhi_epu16( b, _mm_set1_epi16( mulBlue ) );
    
    todo!()
}

fn grayscale_sse(image_data: &[u8]) {

    if image_data.len() % 16 != 0 {
        panic!("Invalid size");
    }

    let mut image_data_it = image_data.chunks(16);

    loop {
        unsafe {

            // let r: __m128i = _mm_setzero_si128(); // r = 0
            // let g: __m128i = _mm_setzero_si128(); // g = 0
            // let b: __m128i = _mm_setzero_si128(); // b = 0

            let chk1_ = image_data_it.next();
            let chk2_ = image_data_it.next();

            if chk1_.is_none() {
                // No more data to process
                break;
            }

            let chk1 = chk1_.unwrap();
            let chk2 = chk2_.unwrap();

            // load rgb

            let v1_ = transmute::<&[u8], __m128i>(chk1);
            // Get pointer
            let ptr =&v1_ as *const __m128i;
            // Load from array
            let a = _mm_loadu_si128(ptr);

            let v2_ = transmute::<&[u8], __m128i>(chk1);
            // Get pointer
            let ptr =&v2_ as *const __m128i;
            // Load from array
            let b = _mm_loadu_si128(ptr);

            let c_r = pack_red(a, b);
            let c_g = pack_blue(a, b);
            let c_b = pack_green(a, b);


        }
    }


}

