//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::SolType;
use clap::Parser;
use hasher_lib::PublicValuesStruct;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const HASHER_ELF: &[u8] = include_elf!("hasher-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long)]
    use_precompile: bool,

    #[arg(long, default_value = "100")]
    rounds: usize,

    #[arg(long, default_value = "Hello, world!")]
    message: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }
    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let use_precompile_flag = if args.use_precompile { 1 } else { 0 };

    let mut stdin = SP1Stdin::new();
    stdin.write(&[use_precompile_flag]);
    stdin.write(&(args.rounds as u32).to_le_bytes());
    stdin.write(&args.message.as_bytes());

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(HASHER_ELF, &stdin).run().unwrap();

        // Read the output.
        let decoded = PublicValuesStruct::abi_decode(output.as_slice()).unwrap();
        let PublicValuesStruct {
            message,
            use_precompile: _,
            hash,
        } = decoded;
        println!("Message: {}", String::from_utf8_lossy(&message));
        println!("Hash: {:?}", hash);
        println!("Rounds: {}", args.rounds);
        println!(
            "Total instruction count: {}",
            report.total_instruction_count()
        );
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(HASHER_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
