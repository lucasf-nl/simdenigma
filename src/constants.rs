pub const REFLECTOR: &[u8; 26] = &[
    24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 0, 19,
]; //"yruhqsldpxngokmiebfzcwvjat".as_bytes();
pub const CHARACTERS8: u8 = 26;

// abcdefghijklmnopqrstuvwxyz
// ekmflgdqvzntowyhxuspaibrcj
// ajdksiruxblhwtmcqgznpyfvoe
// bdfhjlcprtxvznyeiwgakmusqo

pub const ROTOR_I: &[u8; 26] = &[
    4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9,
]; //"ekmflgdqvzntowyhxuspaibrcj".as_bytes();
   // ekmflgdqvzntowyhxuspaibrcj
   // ....a..................... translate e -> a
   // ..........b............... translate k -> b
   // ............c............. translate m -> c
   // .....d.................... translate f -> d
   // ...........e.............. translate l -> e
   // uwygadfpvzbeckmthxslrinqoj
   // abcdefghijklmnopqrstuvwxyz
pub const ROTOR_I_REVERSE: &[u8] = &[
    20, 22, 24, 6, 0, 3, 5, 15, 21, 25, 1, 4, 2, 10, 12, 19, 7, 23, 18, 11, 17, 8, 13, 16, 14, 9,
]; //"uwygadfpvzbeckmthxslrinqoj".as_bytes();
pub const ROTOR_I_TURNOVERS: char = 'q';
pub const ROTOR_II: &[u8] = &[
    0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4,
]; //"ajdksiruxblhwtmcqgznpyfvoe".as_bytes();
pub const ROTOR_II_REVERSE: &[u8] = &[
    0, 9, 15, 2, 25, 22, 17, 11, 5, 1, 3, 10, 14, 19, 24, 20, 16, 6, 4, 13, 7, 23, 12, 8, 21, 18,
]; //"ajpczwrlfbdkotyuqgenhxmivs".as_bytes();
pub const ROTOR_II_TURNOVERS: char = 'e';
pub const ROTOR_III: &[u8] = &[
    1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18, 16, 14,
]; //"bdfhjlcprtxvznyeiwgakmusqo".as_bytes();
pub const ROTOR_III_REVERSE: &[u8] = &[
    19, 0, 6, 1, 15, 2, 18, 3, 16, 4, 20, 5, 21, 13, 25, 7, 24, 8, 23, 9, 22, 11, 17, 10, 14, 12,
]; //"tagbpcsdqeufvnzhyixjwlrkom".as_bytes();
pub const ROTOR_III_TURNOVERS: char = 'v';
