use crate::api_jupiter::{buy_via_quote, create_quote, CreateQuote};
use crate::api_solana::get_all_token_balance_of_account;
use crate::db::create_crypto_data;
use crate::json_config::{
    pull_config, update_config_file_compare_to_main_config, AppStateType, BotGridConfigType,
    BuyOrSell, MainConfig,
};
use crate::model::CreateCryptoData;
use jupiter_swap_api_client::quote::QuoteResponse;
use jupiter_swap_api_client::JupiterSwapApiClient;
use reqwest::Client;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::{pubkey, transaction::VersionedTransaction};
use solana_sdk::{pubkey::Pubkey, signature::NullSigner};
use sqlx::postgres::PgPoolOptions;
use std::borrow::{Borrow, BorrowMut};
use std::env;
use std::error::Error;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub struct BotGrid {
    keypair: Keypair,
    jupiter_swap_api_client: JupiterSwapApiClient,
    config: MainConfig,
    client: Client,
    index_config: usize,
    db: sqlx::Pool<sqlx::Postgres>,
}

impl BotGrid {
    pub async fn new(
        keypair: Keypair,
        jupiter_swap_api_client: JupiterSwapApiClient,
        db: sqlx::Pool<sqlx::Postgres>,
    ) -> Self {
        let config = pull_config("config.json").unwrap();

        println!("{:?}", config);
        let mut s = Self {
            keypair,
            jupiter_swap_api_client,
            config,
            client: Client::new(),
            index_config: 0,
            db,
        };
        s.check_amount_token().await;
        s.edit_json_config(false);

        s
    }

    pub fn edit_json_config(&mut self, force_update: bool) {
        update_config_file_compare_to_main_config("config.json", &self.config, force_update)
            .unwrap();
    }

    pub async fn check_amount_token(&mut self) {
        let res = get_all_token_balance_of_account(&self.keypair.pubkey())
            .await
            .unwrap();
        for coin_balance in res.iter() {
            let coin_balance_info = &coin_balance.account.data.parsed.info;
            let coin_info = self
                .config
                .config
                .iter_mut()
                .find(|config| config.contract_address == coin_balance_info.mint);

            if let Some(coin_info) = coin_info {
                coin_info.decimals = Some(coin_balance_info.token_amount.decimals);
                let multiplier: u64 = 10u64.pow(coin_balance_info.token_amount.decimals as u32);
                coin_info.token_amount =
                    Some((coin_balance_info.token_amount.ui_amount * multiplier as f64) as u64);
            }
        }
    }

    pub fn update_config(&mut self) {
        // import json file and seralise it to specified struct BotGridConfigType
        match pull_config("config.json") {
            Ok(config) => self.config = config,
            Err(e) => println!("Error importing config: {}", e),
        }
    }

    pub async fn run(&mut self) {
        let mut refresh_coin_balance = false;
        loop {
            self.index_config = 0;
            while self.index_config < self.config.config.len() {
                match self.process_coin_info().await {
                    Ok(true) => {
                        if matches!(self.config.config[self.index_config].is_valid, Some(true))
                            && matches!(
                                self.config.config[self.index_config].buy_or_sell,
                                BuyOrSell::Sell
                            )
                        {
                            refresh_coin_balance = true;
                        }
                    }
                    Err(e) => println!(
                        "Error processing {} coin_info: {:?}",
                        self.config.config[self.index_config].name, e
                    ),
                    _ => {}
                }
                self.index_config += 1;
            }
            if refresh_coin_balance {
                self.check_amount_token().await;
                refresh_coin_balance = false;
            }
            self.edit_json_config(true);
            sleep(Duration::from_secs(self.config.interval_second_main_loop)).await;
        }
    }

    // Moved the main logic into a separate async function for readability
    async fn process_coin_info(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let cc_payload = self.prepare_quote_payload()?;
        let quote_response = create_quote(&self.jupiter_swap_api_client, &cc_payload).await?;

        // println!(
        //     "If you sell {} {}, you will receive {} {}",
        //     quote_response.in_amount,
        //     cc_payload.input_address,
        //     quote_response.out_amount,
        //     cc_payload.output_address,
        // );
        if grid_function(
            &quote_response,
            &mut self.config.config[self.index_config],
            Some(&self.db),
        )
        .await
        {
            println!("verifying buy");
            loop {
                sleep(Duration::from_secs(2)).await;
                match create_quote(&self.jupiter_swap_api_client, &cc_payload).await {
                    Ok(quote_response) => {
                        if grid_function(
                            &quote_response,
                            &mut self.config.config[self.index_config],
                            None,
                        )
                        .await
                        {
                            self.try_buy_operation(&cc_payload, quote_response).await?;
                            return Ok(true);
                        }
                        break;
                    }
                    Err(e) => {
                        println!("Error buying: try again {:?}", e);
                    }
                }
            }
        }
        Ok(false)
    }

    // Prepare payload based on coin_info
    fn prepare_quote_payload(&mut self) -> Result<CreateQuote, Box<dyn std::error::Error>> {
        let coin_info = &self.config.config[self.index_config];
        let amount = match coin_info.buy_or_sell {
            BuyOrSell::Buy => coin_info.sol_amount.ok_or("Missing sol_amount")?,
            BuyOrSell::Sell => coin_info.token_amount.ok_or("Missing token_amount")?,
        };

        let input_address = match coin_info.buy_or_sell {
            BuyOrSell::Buy => NATIVE_MINT,
            BuyOrSell::Sell => pubkey::Pubkey::from_str(&coin_info.contract_address)?,
        };
        let output_address = match coin_info.buy_or_sell {
            BuyOrSell::Buy => pubkey::Pubkey::from_str(&coin_info.contract_address)?,
            BuyOrSell::Sell => NATIVE_MINT,
        };

        Ok(CreateQuote {
            amount,
            input_address,
            output_address,
            slippage_bps: 100,
        })
    }

    async fn try_buy_operation(
        &mut self,
        cc_payload: &CreateQuote,
        quote_response: QuoteResponse,
    ) -> Result<(), Box<dyn Error>> {
        let mut attempt = 0;
        let coin_info = &self.config.config[self.index_config];
        if coin_info.is_valid == Some(false) {
            return Ok(());
        }
        let mut quote_response = Ok(quote_response);
        while attempt < 5 {
            match quote_response {
                Ok(quote_response) => {
                    match buy_via_quote(
                        &self.jupiter_swap_api_client,
                        &quote_response,
                        &self.keypair,
                        &coin_info.buy_or_sell,
                        attempt,
                    )
                    .await
                    {
                        Ok(signature) => {
                            let coin_info = &mut self.config.config[self.index_config];
                            coin_info.buy_or_sell = match coin_info.buy_or_sell {
                                BuyOrSell::Buy => {
                                    coin_info.sol_amount = Some(0);
                                    coin_info.token_amount = Some(quote_response.out_amount);
                                    coin_info.state = None;
                                    BuyOrSell::Sell
                                }
                                BuyOrSell::Sell => {
                                    // minus 2 dollar for transaction fee
                                    coin_info.sol_amount = Some(quote_response.out_amount - 105000);
                                    coin_info.token_amount = Some(0);
                                    coin_info.state = None;

                                    BuyOrSell::Buy
                                }
                            };
                            println!(
                                "{} successful. Signature: {}, token: {}",
                                match coin_info.buy_or_sell {
                                    BuyOrSell::Buy => "Purchase",
                                    BuyOrSell::Sell => "Sale",
                                },
                                signature,
                                coin_info.name
                            );
                            return Ok(());
                        }
                        Err(buy_error) => {
                            sleep(Duration::from_millis(200)).await;
                            attempt += 1;
                            if attempt >= 5 {
                                self.check_amount_token().await;
                                self.edit_json_config(true);
                                let coin_info = &mut self.config.config[self.index_config];
                                coin_info.buy_or_sell = match coin_info.buy_or_sell {
                                    BuyOrSell::Buy => {
                                        if coin_info.token_amount == Some(0)
                                            || coin_info.token_amount == None
                                        {
                                            coin_info.sol_amount =
                                                Some(quote_response.in_amount - 155000);
                                            BuyOrSell::Buy
                                        } else {
                                            coin_info.sol_amount = Some(0);
                                            coin_info.state = None;
                                            BuyOrSell::Sell
                                        }
                                    }
                                    BuyOrSell::Sell => {
                                        if coin_info.token_amount == Some(0)
                                            || coin_info.token_amount == None
                                        {
                                            if coin_info.sol_amount == Some(0)
                                                || coin_info.sol_amount == None
                                            {
                                                coin_info.sol_amount = Some(
                                                    quote_response
                                                        .route_plan
                                                        .get(0)
                                                        .unwrap()
                                                        .swap_info
                                                        .out_amount
                                                        - 105000,
                                                );
                                                coin_info.state = None;
                                                BuyOrSell::Buy
                                            } else {
                                                BuyOrSell::Sell
                                            }
                                        } else {
                                            BuyOrSell::Sell
                                        }
                                    }
                                };
                                println!("All attempts to buy_via_quote failed after {} tries. Last error: {:?}", attempt, buy_error);
                                // After 3 failed attempts, return the last error
                                return Err(buy_error.into());
                            }
                        }
                    }
                }
                Err(create_quote_error) => {
                    println!(
                        "Error creating quote on attempt {}: {:?}",
                        attempt + 1,
                        create_quote_error
                    );
                    attempt += 1;
                    sleep(Duration::from_millis(500)).await;
                }
            }
            quote_response = create_quote(&self.jupiter_swap_api_client, &cc_payload).await
        }
        // If all attempts fail, return a generic error or consider custom error handling
        Err("All attempts to buy_via_quote failed".into())
    }
}

pub async fn grid_function(
    response: &QuoteResponse,
    config_coin: &mut BotGridConfigType,
    db: Option<&sqlx::Pool<sqlx::Postgres>>,
) -> bool {
    // Define the current price based on the transaction type
    let is_selling_tokens_for_sol = matches!(config_coin.buy_or_sell, BuyOrSell::Sell);
    if (response.in_amount == 0) || (response.out_amount == 0) {
        return false;
    }
    let mut current_price;
    if is_selling_tokens_for_sol {
        current_price = (response.out_amount as f64) / (response.in_amount as f64);
    } else {
        current_price = (response.in_amount as f64) / (response.out_amount as f64);
    }
    if let Some(db) = db {
        create_crypto_data(
            db,
            &CreateCryptoData {
                name: config_coin.name.clone(),
                price: current_price,
                slipage: response.slippage_bps.to_string(),
                fee_amount: response
                    .route_plan
                    .get(0)
                    .unwrap()
                    .swap_info
                    .fee_amount
                    .to_string(),
                price_impact_pct: response.price_impact_pct.parse().unwrap(),
            },
        )
        .await;
    }

    match &mut config_coin.state {
        Some(state) => {
            if is_selling_tokens_for_sol {
                if current_price > state.highest {
                    println!(
                        "sell-\t{}\t- highest:\t{} SOL.",
                        config_coin.name, current_price,
                    );
                    state.highest = current_price;
                }
                if current_price < state.lowest {
                    println!(
                        "sell-\t{}\t- lowest:\t{} SOL. sell at {}",
                        config_coin.name,
                        current_price,
                        state.highest * (1f64 - config_coin.range)
                    );
                    state.lowest = current_price;
                }
            } else {
                if current_price > state.highest {
                    println!(
                        "buy-\t{}\t- highest:\t{} SOL. at {}",
                        config_coin.name,
                        current_price,
                        state.highest * (1f64 - config_coin.range)
                    );
                    state.highest = current_price;
                }
                if current_price < state.lowest {
                    println!(
                        "buy-\t-{}\t- lowest:\t{} SOL. buy",
                        config_coin.name, current_price,
                    );
                    state.lowest = current_price;
                }
            }
            state.current_price = current_price;
            state.nb_output_if_i_sell = current_price;
        }
        None => {
            config_coin.state = Some(AppStateType {
                lowest: current_price,
                highest: current_price,
                current_price,
                nb_output_if_i_sell: current_price,
            });
        }
    }
    if matches!(config_coin.is_valid, Some(true)) {
        match config_coin.buy_or_sell {
            BuyOrSell::Buy => {
                if buy_condition(config_coin) {
                    if let Some(state) = &config_coin.state {
                        println!(
                            "Token: {} Decision: BUY. Current price: {:.4}, Lowest price: {:.4}",
                            config_coin.name, current_price, state.lowest
                        );
                    }
                    return true;
                }
            }
            BuyOrSell::Sell => {
                if sell_condition(config_coin) {
                    if let Some(state) = &config_coin.state {
                        println!(
                            "Token: {} Decision: SELL. Current price: {}, Highest price: {}",
                            config_coin.name, state.current_price, state.highest
                        );
                    }
                    return true;
                }
            }
        }
    }
    false
}

fn buy_condition(config_coin: &BotGridConfigType) -> bool {
    if let Some(state) = &config_coin.state {
        // println!(
        //     "buy_condition {} > {}",
        //     state.current_price,
        //     state.lowest * (1f64 + config_coin.range)
        // );
        if config_coin.name == "sol-happy" {
            return false;
        }
        if config_coin.name == "sol-hammi" {
            return false;
        }
        if state.current_price > state.lowest * (1f64 + config_coin.range) {
            return true;
        }
    }
    false
}

fn sell_condition(config_coin: &BotGridConfigType) -> bool {
    if let Some(state) = &config_coin.state {
        if config_coin.name == "sol-happy"
            && state.highest > 0.02500
            && state.current_price < state.highest * (1f64 - 0.05)
        {
            return true;
        }
        if config_coin.name == "sol-happy" && state.highest > 0.02500 {
            return true;
        }
        if config_coin.name == "sol-hammi"
            && state.highest > 0.0028200
            && state.current_price < state.highest * (1f64 - 0.05)
        {
            return true;
        }
        if config_coin.name == "sol-hammi"
            && state.highest > 0.0028200
            && state.current_price < 0.0028200
        {
            return true;
        }
        let is_selling_tokens_for_sol = matches!(config_coin.buy_or_sell, BuyOrSell::Sell);
        // println!(
        //     "sell_condition {} < {}",
        //     state.current_price,
        //     state.highest * (1f64 - config_coin.range)
        // );
        if (state.current_price as f64) < state.highest * (1f64 - config_coin.range) {
            return true;
        }
    }
    false
}
