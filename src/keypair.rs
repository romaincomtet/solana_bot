use anyhow::Result;
use solana_sdk::signature::{read_keypair_file, Keypair};

pub fn load_keypair_from_json_file(file_path: &str) -> Result<Keypair> {
    // Attempt to read the keypair from the specified JSON file
    let keypair = read_keypair_file(file_path)
        .map_err(|err| anyhow::anyhow!("Failed to read keypair from file: {}", err))?;

    Ok(keypair)
}
