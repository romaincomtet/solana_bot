use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey;
use solana_sdk::program_pack::Pack;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use spl_token;
use spl_token::state::Account as TokenAccount;
use spl_token::state::Mint;
use std::borrow::Borrow;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcKeyedAccount {
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub lamports: u64,
    pub data: AccountData,
    pub owner: String,
    pub executable: bool,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
    pub space: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountData {
    pub program: String,
    pub parsed: ParsedAccountData,
    pub space: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedAccountData {
    pub info: AccountInfo,
    #[serde(rename = "type")]
    pub account_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfo {
    pub mint: String,
    pub owner: String,
    pub state: String,
    #[serde(rename = "tokenAmount")]
    pub token_amount: TokenAmount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: u8,
    #[serde(rename = "uiAmount")]
    pub ui_amount: f64,
    #[serde(rename = "uiAmountString")]
    pub ui_amount_string: String,
}

fn deserialize_accounts(
    json_data: &str,
) -> Result<Vec<RpcKeyedAccount>, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str::<Vec<RpcKeyedAccount>>(json_data).unwrap())
}

pub async fn get_all_token_balance_of_account(
    pubkey: &Pubkey,
) -> Result<Vec<RpcKeyedAccount>, Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    let spl_token_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;

    // Fetch all SPL Token accounts for the user's wallet by mint
    let token_accounts: Vec<solana_client::rpc_response::RpcKeyedAccount> = client
        .get_token_accounts_by_owner(pubkey, TokenAccountsFilter::ProgramId(spl_token_program_id))
        .await?;
    let serialized = serde_json::to_string(&token_accounts).unwrap();
    let res: Vec<RpcKeyedAccount> = deserialize_accounts(serialized.as_str()).unwrap();
    Ok(res)
}
