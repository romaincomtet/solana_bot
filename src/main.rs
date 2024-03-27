mod discord;
#[allow(unused)]
mod keypair;
mod types;

use dotenv;
use solana_program::pubkey;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::error::Error;
use tokio;

pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // dotenv().ok();

    // let file_path = "secret_sol.json";
    // let keypair: solana_sdk::signature::Keypair = load_keypair_from_json_file(file_path).unwrap();
    // let api_base_url = env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v6".into());
    // println!("pubkey: {:?}", keypair.pubkey().to_string());
    // let jupiter_swap_api_client: JupiterSwapApiClient = JupiterSwapApiClient::new(api_base_url);
    // let pool: sqlx::Pool<sqlx::Postgres> = establish_connection().await;
    // let mut bot_grid = bot_grid_refacto::BotGrid::new(keypair, jupiter_swap_api_client, pool).await;

    // bot_grid.run().await;

    // anyhow::Ok(())
    Ok(())
}
