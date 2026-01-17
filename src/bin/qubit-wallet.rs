use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: qubit-wallet [export|show]");
        println!("  export  - Show wallet address in hex format");
        println!("  show    - Show full wallet details (hex address)");
        return;
    }

    let command = &args[1];

    // Load wallet
    let wallet_data = match fs::read("wallet.dat") {
        Ok(data) => data,
        Err(_) => {
            eprintln!("❌ Error: wallet.dat not found. Run the qubit node first to generate a wallet.");
            std::process::exit(1);
        }
    };

    // Deserialize wallet
    let wallet: qubit_core::Wallet = match bincode::deserialize(&wallet_data) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("❌ Error deserializing wallet: {}", e);
            std::process::exit(1);
        }
    };

    match command.as_str() {
        "export" => {
            println!("{}", hex::encode(wallet.address));
        }
        "show" => {
            println!("💳 Qubit Wallet Details");
            println!("=======================");
            println!("Address (hex): {}", hex::encode(wallet.address));
            println!("Address length: {} bytes", wallet.address.len());
            println!("⚠️  KEEP wallet.dat SAFE - it contains your secret key!");
        }
        _ => {
            eprintln!("❌ Unknown command: {}", command);
            eprintln!("Use 'export' or 'show'");
            std::process::exit(1);
        }
    }
}
