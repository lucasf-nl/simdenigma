use std::simd::Simd;

use crate::constants::*;
use crate::simd::*;

pub fn handle_queue(mut queue: &mut [u8; 16], positions: &mut [u8; 3]) {
    let mut rotors: [[u8; 3]; 16] = [[0; 3]; 16];
    let mut transposed_rotors: [[u8; 16]; 3] = [[0; 16]; 3];

    // Pre-calculate rotor positions
    for i in 0..16 {
        move_rotors(positions);
        rotors[i] = *positions;
    }

    // Transpose rotor positions for more efficient SIMD
    for i in 0..16 {
        for j in 0..3 {
            transposed_rotors[j][i] = rotors[i][j];
        }
    }

    // Apply rotors
    apply_rotor_simd(&mut queue, ROTOR_III, transposed_rotors[2]);
    apply_rotor_simd(&mut queue, ROTOR_II, transposed_rotors[1]);
    apply_rotor_simd(&mut queue, ROTOR_I, transposed_rotors[0]);

    // Apply reflector
    apply_reflector_simd(&mut queue);

    // Apply rotors in reverse
    apply_rotor_simd(&mut queue, ROTOR_I_REVERSE, transposed_rotors[0]);
    apply_rotor_simd(&mut queue, ROTOR_II_REVERSE, transposed_rotors[1]);
    apply_rotor_simd(&mut queue, ROTOR_III_REVERSE, transposed_rotors[2]);
}

fn apply_rotor_simd(queue: &mut [u8; 16], rotor: &'static [u8], positions: [u8; 16]) {
    let queue_simd: Simd<u8, 16> = Simd::from_slice(queue);
    let position_simd = Simd::from_slice(&positions);
    let characters_simd = Simd::splat(CHARACTERS8);

    // Index in the rotor
    let index = (queue_simd + position_simd) % characters_simd;

    // Value on the rotor
    let encrypted_char = Simd::gather_or(&rotor, convert_simd_u8_to_usize(index), Simd::splat(0));

    // Final index
    let final_index = (encrypted_char + characters_simd - position_simd) % characters_simd;

    // Update the queue
    *queue = final_index.to_array();
}

fn apply_reflector_simd(queue: &mut [u8; 16]) {
    let queue_simd: Simd<u8, 16> = Simd::from_slice(queue);

    // Index in the reflector
    let index = queue_simd;
    let encrypted_char =
        Simd::gather_or(REFLECTOR, convert_simd_u8_to_usize(index), Simd::splat(0));

    // Update the queue
    *queue = encrypted_char.to_array();
}

fn check_notches(rotors: &[u8; 3]) -> [bool; 3] {
    let mut matches_notches = [false, false, false];
    let notches = [
        ROTOR_I_TURNOVERS as u8 - 'a' as u8,
        ROTOR_II_TURNOVERS as u8 - 'a' as u8,
        ROTOR_III_TURNOVERS as u8 - 'a' as u8 + 1,
    ];

    for i in 0..3 {
        if rotors[i] == notches[i] {
            matches_notches[i] = true;
        }
    }

    matches_notches
}

pub fn move_rotors(positions: &mut [u8; 3]) -> [u8; 3] {
    // Precalculate notches
    let mut notches = check_notches(positions);

    // Step the middle motor if it's at the notch, this handles double stepping
    if notches[1] {
        positions[1] = (positions[1] + 1) % 26;
        positions[0] = (positions[0] + 1) % 26;
    }

    // Step the right rotor
    positions[2] = (positions[2] + 1) % 26;

    // Recalculate notches
    notches = check_notches(positions);

    // Double step
    if notches[2] {
        // Step the middle rotor
        positions[1] = (positions[1] + 1) % 26;
    }

    *positions
}
