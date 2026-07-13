use clap::{Parser, Subcommand};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "amun")]
#[command(about = "AmunChain CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Keygen {
        #[arg(long, default_value = "amun_wallet.json")]
        output: String,
    },
    Sign {
        #[arg(long)]
        keyfile: String,
        #[arg(long)]
        message: String,
    },
    Verify {
        #[arg(long)]
        pubkey: String,
        #[arg(long)]
        message: String,
        #[arg(long)]
        signature: String,
    },
    Info,
}

fn prompt_password(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().ok();
    let mut password = String::new();
    io::stdin().read_line(&mut password).ok();
    password.trim().to_string()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen { output } => {
            let password = if let Ok(pw) = std::env::var("AMUN_KEYSTORE_PASSWORD") {
                pw
            } else {
                let pw1 = prompt_password("Enter keystore password");
                let pw2 = prompt_password("Confirm keystore password");
                if pw1 != pw2 {
                    eprintln!("Passwords do not match");
                    std::process::exit(1);
                }
                if pw1.len() < 12 {
                    eprintln!("Password must be at least 12 characters");
                    std::process::exit(1);
                }
                pw1
            };

            let signer = amun_crypto::Ed25519Signer::generate();
            let keystore = amun_keystore::KeyStore::create(
                &signer.to_bytes(),
                &signer.public_bytes(),
                &hex::encode(&signer.public_bytes()[..20]),
                &password,
                1,
            )
            .expect("keystore creation failed");

            std::fs::write(&output, keystore.to_json()).expect("write failed");
            println!("Key written to {}", output);
            println!("Address: {}", hex::encode(&signer.public_bytes()[..20]));
        }
        Commands::Sign { keyfile, message } => {
            let password = if let Ok(pw) = std::env::var("AMUN_KEYSTORE_PASSWORD") {
                pw
            } else {
                prompt_password("Enter keystore password")
            };

            let content = std::fs::read_to_string(&keyfile).expect("read failed");
            let keystore = amun_keystore::KeyStore::from_json(&content).expect("parse failed");
            let secret = keystore.decrypt(&password).expect("decrypt failed");
            let signer = amun_crypto::Ed25519Signer::from_seed(
                &secret[..32].try_into().expect("invalid seed"),
            );
            match signer.sign(message.as_bytes(), b"AMUN_CLI", 1) {
                Ok(sig) => println!("Signature: {}", hex::encode(sig)),
                Err(e) => eprintln!("Signing failed: {:?}", e),
            }
        }
        Commands::Verify {
            pubkey,
            message,
            signature,
        } => {
            let pk = hex::decode(&pubkey).expect("invalid pubkey hex");
            let sig = hex::decode(&signature).expect("invalid signature hex");
            match amun_crypto::Ed25519Signer::verify(
                &pk[..32].try_into().expect("invalid pubkey length"),
                message.as_bytes(),
                &sig[..64].try_into().expect("invalid sig length"),
                b"AMUN_CLI",
                1,
            ) {
                Ok(()) => println!("Valid"),
                Err(e) => println!("Invalid: {:?}", e),
            }
        }
        Commands::Info => {
            println!("AmunChain CLI v1.0.0");
            println!("Network: sovereign testnet");
            println!("Chain ID: 1");
        }
    }
}
