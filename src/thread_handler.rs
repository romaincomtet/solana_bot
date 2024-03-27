pub struct TreadHandler {
    pub thread: Option<std::thread::JoinHandle<()>>,
    pub keypair: solana_sdk::signature::Keypair,
    pub stop: bool,
}

impl TreadHandler {
    pub async fn new(keypair: solana_sdk::signature::Keypair) -> Self {
        Self {
            thread: None,
            keypair,
            stop: false,
        }
    }
}
