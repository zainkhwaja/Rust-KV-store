mod storage;
mod commands;
mod server;

use server::Server;
use anyhow::Result;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    // Create and run server
    let server = Server::new("127.0.0.1:6379".to_string());
    server.run().await?;
    
    Ok(())
}