use std::time::SystemTime;

use jupiter_swap_api_client::JupiterSwapApiClient;

pub struct TreadHandler {
    pub thread: Option<std::thread::JoinHandle<()>>,
    pub keypair: solana_sdk::signature::Keypair,
    pub stop: bool,
}

pub enum Strategy {
    Nothing,
    Sell(Sell),
    Buy(Buy),
    // BuySell(BuySell),
}

pub enum ConfigBuyOrSell {
    OrderBook,
    PercentageOf(PercentageOf),
    Grid(f64),
}

pub struct PercentageOf {
    pub percentage: f64,
    pub price_of: f64, // if not set, it will be the actual price
}

pub enum Amount {
    All,
    Amount(f64),
    Percentage(f64),
}

pub struct Sell {
    config: ConfigBuyOrSell,
    amount: Amount,
    pub slippage: f64,
}

pub struct Buy {
    pub config: ConfigBuyOrSell,
    pub amount: Amount,
    pub slippage: f64,
}

pub struct ThreadMod {
    pub jupiter_api_client: JupiterSwapApiClient,
    pub coin: Coin,
    pub discord_settings: DiscordSettings,
    pub strategy: Option<Strategy>,
}

pub struct Coin {
    pub mint_address: String,
    pub mint_symbol: String,
    pub token_amount: u64,
    pub price: f64,
    pub price_sol: f64,
    pub history: Vec<CoinHistory>,
}

pub struct CoinHistory {
    pub price: f64,
    pub price_sol: f64,
    pub time: SystemTime,
}

pub struct DiscordSettings {
    pub chanel_id: String,
    pub muted: bool,
}

// fn te() {
//     match strategy {
//         Strategy::Nothing => {
//             // Do nothing
//         }
//         Strategy::Sell(sell) => {
//             // Sell
//         }
//         Strategy::Buy(buy) => {
//             // Buy
//         }
//         Strategy::BuySell(buy_sell) => {
//             // Buy and Sell
//         }
//     }
// }
