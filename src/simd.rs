use std::simd::Simd;

pub fn convert_simd_u8_to_usize(u8_simd: Simd<u8, 16>) -> Simd<usize, 16> {
    // Initialize an array to hold the converted values
    let mut usize_array = [0_usize; 16];

    // Use a loop to convert each element
    for i in 0..16 {
        usize_array[i] = u8_simd[i] as usize;
    }

    // Create a Simd<usize, 16> from the array
    Simd::from_array(usize_array)
}
