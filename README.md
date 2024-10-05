# Simdenigma

An Enigma machine simulator written in Rust, accelerated using portable SIMD.
Building this project requires a nightly version of Rust because portable SIMD
has not yet been stabilized (at time of writing). The project is fully single-threaded,
adding multi-threading is probably not worth the additional complexity.

The machine encodes 100 million randomly generated characters.
It does this in 1,6 seconds on an M1 Macbook Air (24B5055e).
This averages to 16,67 nanoseconds per character, or 60 million characters per second.
Most of the time is spent randomly generating the characters, so the actual encoding is much faster.

## Enigma settings

- Model: Enigma M3
- Reflector: UKW B
- Rotors: I, II, III
- Rotors positions: 1, 1, 1
- Ring settings: A, A, A
- Plugboard: None

These settings are harded coded in `constants.rs` and `enigma.rs`, they can be entered into
[cryptii](https://cryptii.com/pipes/enigma-machine) to verify the correctness of the implementation.
To view output, enable the `loginputandoutput` feature flag.

## Building

Settings are configured at compile time using feature flags. These are available:

- `loginputandoutput`: Logs the input and output of the machine, this significantly slows down the machine.

## Benchmarking

Benchmarking uses the hyperfine cli tool. To run benchmarks, run these commands
(macOS, but should work on Linux):

```sh
cargo build --release
hyperfine --warmup 3 'target/release/simdenigma'
```

## Optimizations

The following optimizations have been applied:
- Constants are defined as `&[u8]` instead of `&str` to avoid safety checks in the byte/string to char conversion.
  - Speedup of 46%
- Avoiding reverse rotor search by using a lookup table.
  - Further speedup of 48%
  - ```
    // ekmflgdqvzntowyhxuspaibrcj Original rotor
    // ....a..................... translate e -> a
    // ..........b............... translate k -> b
    // ............c............. translate m -> c
    // .....d.................... translate f -> d
    // ...........e.............. translate l -> e
    // uwygadfpvzbeckmthxslrinqoj Inverted rotor
    // abcdefghijklmnopqrstuvwxyz Alphabet
    ```
- Avoiding char to u8 conversion by using the alphabet indices.
  - Further speedup of 87%
- Adding SIMD to apply rotors concurrently.
  - Further speedup of 36%

## Scripts

- `./scripts/chartoalphabetarray.js`: Turns a string of characters into an array of alphabet indices.
- `./scripts/rotor_invertor.js`: Creates a reverse rotor lookup table from a normal rotor.
