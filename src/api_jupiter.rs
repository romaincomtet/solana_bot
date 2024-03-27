use crate::types::Coin;
use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    swap::SwapRequest,
    transaction_config::{ComputeUnitPriceMicroLamports, TransactionConfig},
    JupiterSwapApiClient,
};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::{signature::Keypair, transaction::VersionedTransaction};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub data: HashMap<String, TokenInfo>,
    #[serde(rename = "timeTaken")]
    pub time_taken: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub id: String,
    #[serde(rename = "mintSymbol")]
    pub mint_symbol: String,
    #[serde(rename = "vsToken")]
    pub vs_token: String,
    #[serde(rename = "vsTokenSymbol")]
    pub vs_token_symbol: String,
    pub price: f64
}


pub struct CreateQuote {
    pub amount: u64,
    pub input_address: Pubkey,
    pub output_address: Pubkey,
    pub slippage_bps: u16,
}

pub fn deserialize_token(json_data: &str)-> Result<Token, Box<dyn Error>> {
    Ok(serde_json::from_str::<Token>(json_data).unwrap())
}

pub async fn create_quote(
    jupiter_swap_api_client: &JupiterSwapApiClient,
    create_quote: &CreateQuote,
) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
    // Assuming it might return an error
    let quote_request = QuoteRequest {
        amount: create_quote.amount,
        input_mint: create_quote.input_address,
        output_mint: create_quote.output_address,
        slippage_bps: create_quote.slippage_bps,
        ..QuoteRequest::default()
    };

    // GET /quote
    let quote_response: jupiter_swap_api_client::quote::QuoteResponse =
        jupiter_swap_api_client.quote(&quote_request).await?;
    Ok(quote_response)
}


pub async fn fetch_price(mint_address: &String) -> Result<f64, Box<dyn Error>> {
    let jupiter_endoint : String = std::env::var("JUPITER_PRICE_URL").expect("JUPITER_PRICE_URL must be set");

    let url = format!("{}?ids={}", jupiter_endoint, mint_address);
    // println!("url: {}", url);
    let response = reqwest::get(&url).await?.json::<Value>().await?;

    let serialized = serde_json::to_string(&response).unwrap();

    let deserialized_token = deserialize_token(serialized.as_str()).unwrap();

    let price: f64 = deserialized_token.data.get(mint_address).unwrap().price;

    Ok(price)
}

// pub async fn buy_via_quote(
//     jupiter_swap_api_client: &JupiterSwapApiClient,
//     quote_response: &QuoteResponse,
//     keypair: &Keypair,
//     is_sell: &BuyOrSell,
//     nb_of_try: u8,
// ) -> Result<solana_sdk::signature::Signature, Box<dyn std::error::Error>> {
//     // setting compute_unit_price_micro_lamports to Auto will prioritize the transaction
//     let mut default_transaction_config: TransactionConfig = TransactionConfig::default();
//     if nb_of_try > 0 {
//         default_transaction_config.compute_unit_price_micro_lamports =
//             Some(ComputeUnitPriceMicroLamports::Auto);
//     } else if let BuyOrSell::Sell = is_sell {
//         default_transaction_config.compute_unit_price_micro_lamports =
//             Some(ComputeUnitPriceMicroLamports::Auto);
//     }
//     // POST /swap
//     let swap_response = jupiter_swap_api_client
//         .swap(&SwapRequest {
//             user_public_key: keypair.pubkey(),
//             quote_response: quote_response.clone(),
//             config: default_transaction_config,
//         })
//         .await?;

//     println!("Raw tx len: {}", swap_response.swap_transaction.len());

//     // Perform further actions as needed...

//     // // POST /swap-instructions
//     // let swap_instructions = jupiter_swap_api_client
//     //     .swap_instructions(&SwapRequest {
//     //         user_public_key: keypair.pubkey(),
//     //         quote_response: quote_response.clone(),
//     //         config: TransactionConfig::default(),
//     //     })
//     //     .await?;

//     println!("Raw tx len: {}", swap_response.swap_transaction.len());
//     // send with rpc client...
//     let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".into());

//     let mut versioned_transaction: VersionedTransaction =
//         bincode::deserialize(&swap_response.swap_transaction).unwrap();
//     let recent_blockhash = rpc_client.get_latest_blockhash().await?;
//     versioned_transaction
//         .message
//         .set_recent_blockhash(recent_blockhash);

//     // Replace with a keypair or other struct implementing signer
//     let signed_versioned_transaction =
//         VersionedTransaction::try_new(versioned_transaction.message, &[&keypair])?;

//     // This will fail with "Transaction signature verification failure" as we did not really sign
//     match rpc_client
//         .send_and_confirm_transaction(&signed_versioned_transaction)
//         .await
//     {
//         Ok(signature) => Ok(signature),
//         Err(e) => {
//             println!("Error: {:?}", e);
//             Err(e.into())
//         }
//     }
// }
//
//
