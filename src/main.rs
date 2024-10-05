#![feature(portable_simd)]

use rand::Rng;

mod constants;
mod enigma;
mod simd;

use enigma::*;

fn main() {
    let num_chars = 100_000_000;
    let mut rng = rand::thread_rng();

    let mut rotor_positions: [u8; 3] = [0, 0, 0]; // Corresponds to AAA

    let mut queue: [u8; 16] = [0; 16];

    for i in 0..num_chars {
        let idx = rng.gen_range(0..26);

        let ii = i % 16;

        queue[ii] = idx;

        if ii == 15 {
            #[cfg(feature = "loginputandoutput")]
            println!("Pre-processed: {:#?}", simd_num_to_char(&queue));

            handle_queue(&mut queue, &mut rotor_positions);

            #[cfg(feature = "loginputandoutput")]
            println!("Processed: {:#?}", simd_num_to_char(&queue));
        }
    }
}

#[cfg(feature = "loginputandoutput")]
fn simd_num_to_char(num: &[u8; 16]) -> [char; 16] {
    let num_simd: Simd<u8, 16> = Simd::from_slice(num);
    let num_simd: Simd<u32, 16> = num_simd.cast();
    let chars_simd: Simd<u32, 16> = Simd::from_slice(&['a' as u32; 16]);

    let result = num_simd + chars_simd;

    result
        .to_array()
        .iter()
        .map(|&x| x as u8 as char)
        .collect::<Vec<char>>()
        .as_slice()
        .try_into()
        .unwrap()
}
