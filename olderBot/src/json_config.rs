use std::fs::File;

use serde::{Deserialize, Serialize};
use std::error;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum StrategyType {
    Grid,
    MeanReversion,
    Momentum,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BuyOrSell {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RangeLimitType {
    pub limit: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppStateType {
    pub lowest: f64, // Directly represents the lowest out_amount for selling tokens for SOL
    pub highest: f64, // Directly represents the highest out_amount for selling tokens for SOL
    pub current_price: f64, // Last observed out_amount
    pub nb_output_if_i_sell: f64, // Additional field as per your existing logic
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotGridConfigType {
    pub is_valid: Option<bool>,
    pub name: String,
    pub decimals: Option<u8>,
    pub token_amount: Option<u64>,
    pub sol_amount: Option<u64>,
    pub contract_address: String,
    pub strategy: StrategyType,
    pub buy_or_sell: BuyOrSell,
    pub range: f64,
    pub state: Option<AppStateType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainConfig {
    pub config: Vec<BotGridConfigType>,
    pub interval_second_main_loop: u64,
}

fn get_json_config(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn pull_config(file_path: &str) -> Result<MainConfig, Box<dyn error::Error>> {
    let config = get_json_config(file_path).unwrap();
    let config: MainConfig = serde_json::from_str(config.as_ref())?;
    Ok(config)
}

pub fn update_config_file_compare_to_main_config(
    file_path: &str,
    current_config: &MainConfig,
    force_update: bool,
) -> Result<(), Box<dyn error::Error>> {
    // Attempt to load the existing configuration
    let existing_config = pull_config(file_path);

    match existing_config {
        Ok(existing_config) => {
            // Compare the existing configuration with the current configuration
            if force_update
                || serde_json::to_string(&existing_config)?
                    != serde_json::to_string(current_config)?
            {
                // If different, write the current configuration to the file
                let updated_config_json = serde_json::to_string_pretty(current_config)?;
                let mut file = File::create(file_path)?;
                file.write_all(updated_config_json.as_bytes())?;
                // print!("Config updated. ");
            } else {
                println!("No changes detected. Configuration file not updated.");
            }
        }
        Err(_) => {
            println!("Configuration cannot be loaded. there is a problem with the file.");
        }
    }

    Ok(())
}
