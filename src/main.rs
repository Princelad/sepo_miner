mod argon2;
mod messages;
mod websocket;

use anyhow::Result;
use websocket::WebSocketClient;

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: Parse wallet address from CLI args
    let wallet_address = std::env::var("WALLET_ADDRESS")
        .unwrap_or_else(|_| {
            eprintln!("[Error] WALLET_ADDRESS environment variable not set");
            eprintln!("Usage: WALLET_ADDRESS=0xYourAddress cargo run");
            std::process::exit(1);
        });

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║            SepoMiner v{:<27} ║", env!("CARGO_PKG_VERSION"));
    println!("║     High-Performance Sepolia Faucet Miner (Rust)     ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();

    let client = WebSocketClient::new(wallet_address);
    
    // Run the WebSocket event loop
    // This will block until the connection is closed or an error occurs
    client.run().await?;

    Ok(())
}
