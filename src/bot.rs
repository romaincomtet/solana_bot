use std::{error::Error, time::{Duration, SystemTime}};

use tokio::time::sleep;

use crate::{api_jupiter::fetch_price, types::{Coin, CoinHistory, ThreadMod}};

pub struct Bot {

}

impl Bot {
    pub async fn monitor_prices(&self, thread: &mut ThreadMod) -> Result<(), Box<dyn Error>> {
        let mut previous_price = fetch_price(&thread.coin.mint_address).await.unwrap();
        let entry_time = SystemTime::now();

        while !thread.discord_settings.muted {
            sleep(Duration::from_secs(60)).await;

            let current_time = SystemTime::now();

            let current_price = fetch_price(&thread.coin.mint_address).await.unwrap();

            if let Ok(duration) = current_time.duration_since(entry_time) {
                let seconds = duration.as_secs();

                if seconds >= 300 && seconds <= 350 {
                    thread.coin.history.push(CoinHistory {
                        price: current_price,
                        price_sol: 0.0,
                        time: current_time
                    })
                }
            }

            let difference = (current_price - previous_price) / 100.0;

            if (difference >= 5.0) {
                // Send message
            }

            previous_price = current_price;
        }

        Ok(())
    }
}
