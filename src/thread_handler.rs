use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::types::ThreadMod;

pub struct TreadHandler {
    pub threads: Option<Vec<JoinHandle<()>>>,
    pub keypair: solana_sdk::signature::Keypair,
    pub stop: bool,
    pub thread_mods: Vec<Arc<Mutex<ThreadMod>>>,
}

impl TreadHandler {
    pub async fn new(keypair: solana_sdk::signature::Keypair) -> Self {
        Self {
            threads: None,
            keypair,
            stop: false,
            thread_mods: vec![],
        }
    }

    // pub async fn init() {
    //     // Example data - replace with actual data initialization.
    //     let mods = vec![
    //         ThreadMod {
    //             jupiter_api_client: JupiterSwapApiClient {},
    //             coin: Coin {},
    //             discord_settings: DiscordSettings {},
    //             strategy: None,
    //         },
    //         // Add more ThreadMod instances as needed.
    //     ];
    // }

    pub async fn run(mut self) {
        // // Wrap each ThreadMod in Arc<Mutex<T>> for shared and mutable access.
        // let shared_mods: Vec<Arc<Mutex<ThreadMod>>> = self.thread_mods
        //     .into_iter()
        //     .map(|mod_instance| Arc::new(Mutex::new(mod_instance)))
        //     .collect();

        // Create a thread for each ThreadMod instance.
        // let threads: Vec<JoinHandle<()>> = shared_mods
        //     .iter()
        //     .map(|shared_mod| {
        //         let shared_mod_clone = Arc::clone(shared_mod);
        //         thread::spawn(move || {
        //             // Example processing.
        //             let mut mod_guard = shared_mod_clone.lock().unwrap();
        //             // Modify mod_guard as needed.
        //             // For instance, you might call a method on jupiter_api_client.
        //         })
        //     })
        //     .collect();
    }

    pub async fn finish(&mut self) {
        // self.stop = true;
        // if let Some(threads) = &self.threads {
        //     for thread in threads.iter() {
        //         thread.join().unwrap();
        //     }
        // }
    }
}
