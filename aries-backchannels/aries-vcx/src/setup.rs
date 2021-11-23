use aries_vcx::init::{open_main_pool, PoolConfig, open_as_main_wallet, init_issuer_config}; // TODO: Should we move all Config arguments to a single module?
use aries_vcx::libindy::utils::wallet::{create_wallet, configure_issuer_wallet, close_main_wallet, WalletConfig};
use aries_vcx::utils::provision::{provision_cloud_agent, AgentProvisionConfig};
use aries_vcx::utils::plugins::init_plugin;
use aries_vcx::libindy::utils::pool;
use aries_vcx::settings;
use std::io::prelude::*;
use crate::AgentConfig;
use rand::{thread_rng, Rng};
use uuid;

#[derive(Debug, Deserialize)]
struct SeedResponse {
    did: String,
    seed: String,
    verkey: String
}

async fn get_trustee_seed() -> String {
    if let Some(ledger_url) = std::env::var("LEDGER_URL").ok() {
        let url = format!("{}/register", ledger_url);
        let mut rng = thread_rng();
        let client = reqwest::Client::new();
        let body = json!({
            "role": "TRUST_ANCHOR",
            "seed": format!("my_seed_000000000000000000{}", rng.gen_range(100000..1000000))
        }).to_string();
        client.post(&url).body(body).send().expect("Failed to send message").json::<SeedResponse>().expect("Failed to deserializ response").seed
    } else {
        "000000000000000000000000Trustee1".to_string()
    }
}

async fn download_genesis_file() -> std::result::Result<String, String> {
    match std::env::var("GENESIS_FILE").ok() {
        Some(genesis_file) => {
            if !std::path::Path::new(&genesis_file).exists() {
                Err(format!("The file {} does not exist", genesis_file))
            } else {
                info!("Using genesis file {}", genesis_file);
                Ok(genesis_file)
            }
        }
        None => match std::env::var("LEDGER_URL").ok() {
            Some(ledger_url) => {
                info!("Downloading genesis file");
                let genesis_url = format!("{}/genesis", ledger_url);
                let body = reqwest::get(&genesis_url)
                    .expect("Failed to get genesis file from ledger")
                    .text()
                    .expect("Failed to get the response text");
                let path = std::env::current_dir().expect("Failed to obtain the current directory path").join("resource").join("genesis_file.txn");
                let mut f = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path.clone())
                    .expect("Unable to open file");
                f.write_all(body.as_bytes()).expect("Unable to write data");
                debug!("Genesis file downloaded and saved to {:?}", path);
                path.to_str().map(|s| s.to_string()).ok_or("Failed to convert genesis file path to string".to_string())
            }
            None => {
                std::env::current_dir().expect("Failed to obtain the current directory path").join("resource").join("indypool.txn").to_str().map(|s| s.to_string()).ok_or("Failed to convert genesis file path to string".to_string())
            }
        }
    }
}

pub async fn initialize() -> AgentConfig {
    info!("Initializing vcx");
    let genesis_path = download_genesis_file().await.expect("Failed to download the genesis file");
    let agency_endpoint = std::env::var("CLOUD_AGENCY_URL").unwrap_or("http://localhost:8000".to_string());
    init_plugin(settings::DEFAULT_PAYMENT_PLUGIN, settings::DEFAULT_PAYMENT_INIT_FUNCTION);
    let pool_config = PoolConfig {
        genesis_path,
        pool_config: None,
        pool_name: None
    };
    let agency_config = AgentProvisionConfig {
        agency_endpoint,
        agency_did: "VsKV7grR1BUE29mG2Fm2kX".to_string(),
        agency_verkey: "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR".to_string(),
        agent_seed: None
    };
    let wallet_config = WalletConfig {
        wallet_name: format!("rust_agent_{}", uuid::Uuid::new_v4().to_string()),
        wallet_key: "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY".to_string(),
        wallet_key_derivation: "RAW".to_string(),
        rekey: None,
        storage_config: None,
        rekey_derivation_method: None,
        storage_credentials: None,
        wallet_type: None
    };
    
    create_wallet(&wallet_config).expect("Failed to create wallet");
    let _wh = open_as_main_wallet(&wallet_config).expect("Failed to open the main wallet");
    let _ph = open_main_pool(&pool_config).expect("Failed to open the main pool");

    let enterprise_seed = get_trustee_seed().await;
    let issuer_config = configure_issuer_wallet(&enterprise_seed).expect("Failed to configure the issuer wallet");
    init_issuer_config(&issuer_config).expect("Failed to init issuer config");
    let _agency_config = provision_cloud_agent(&agency_config).expect("Failed to provision the cloud agent");

    debug!("Initialization finished");
    AgentConfig { did: issuer_config.institution_did }
}

pub fn shutdown() {
    close_main_wallet().expect("Failed to close the main wallet");
    pool::close().expect("Failed to close the main pool");
}
