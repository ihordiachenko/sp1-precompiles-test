//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th hash
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_primitives::FixedBytes;
use alloy_sol_types::SolValue;
use hasher_lib::{sha256, sha256_with_precompile, PublicValuesStruct};
use sp1_zkvm::io;

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let use_precompile_flag: u8 = io::read();
    let rounds = io::read::<u32>() as usize;
    let message = io::read::<Vec<u8>>();

    let use_precompile = use_precompile_flag != 0;

    // Run hashing 100 times
    let mut current_hash = message.clone();
    let mut final_hash = [0u8; 32];

    for _ in 0..rounds {
        final_hash = if use_precompile {
            sha256_with_precompile(&current_hash)
        } else {
            sha256(&current_hash)
        };
        current_hash = final_hash.to_vec();
    }

    // Encode the public values of the program.
    let public = PublicValuesStruct {
        message: message.into(),
        use_precompile,
        hash: FixedBytes(final_hash),
    };
    let bytes = public.abi_encode();

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    io::commit_slice(&bytes);
}
