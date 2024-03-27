mod discord;
#[allow(unused)]
mod keypair;
mod thread_handler;
mod types;
use crate::keypair::load_keypair_from_json_file;
use dotenv::dotenv;
use jupiter_swap_api_client::JupiterSwapApiClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::{env, error::Error};
use tokio;

pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let file_path = "secret_sol.json";
    let keypair: solana_sdk::signature::Keypair = load_keypair_from_json_file(file_path).unwrap();
    let api_base_url = env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v6".into());
    println!("pubkey: {:?}", keypair.pubkey().to_string());
    let jupiter_swap_api_client: JupiterSwapApiClient = JupiterSwapApiClient::new(api_base_url);
    // let pool: sqlx::Pool<sqlx::Postgres> = establish_connection().await;
    // let mut bot_grid = bot_grid_refacto::BotGrid::new(keypair, jupiter_swap_api_client, pool).await;

    // bot_grid.run().await;

    // anyhow::Ok(())
    Ok(())
}
