mod api_jupiter;
mod api_solana;
#[allow(unused)]
mod bot_grid_refacto;
mod db;
mod json_config;
mod keypair;
mod model;
use crate::db::establish_connection;
use crate::keypair::load_keypair_from_json_file;
use anyhow::Result;
use dotenv::dotenv;
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::Signer;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let mint_address = "26KMQVgDUoB6rEfnJ51yAABWWJND8uMtpnQgsHQ64Udr"; // Example: SOL mint address

    let file_path = "secret_sol.json";
    let keypair: solana_sdk::signature::Keypair = load_keypair_from_json_file(file_path).unwrap();
    let api_base_url = env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v6".into());
    println!("pubkey: {:?}", keypair.pubkey().to_string());
    let jupiter_swap_api_client: JupiterSwapApiClient = JupiterSwapApiClient::new(api_base_url);
    let pool: sqlx::Pool<sqlx::Postgres> = establish_connection().await;
    let mut bot_grid = bot_grid_refacto::BotGrid::new(keypair, jupiter_swap_api_client, pool).await;

    bot_grid.run().await;

    anyhow::Ok(())
}
