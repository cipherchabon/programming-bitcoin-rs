use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Cursor, Read, Write},
};

use crate::transactions::tx::Tx;

/// Fetches transactions from the network
pub struct TxFetcher {
    cache: RefCell<HashMap<String, Tx>>,
    api_url: String,
}

impl TxFetcher {
    /// Creates a new TxFetcher
    pub fn builder() -> TxFetcherBuilder {
        TxFetcherBuilder::new()
    }

    /// Fetches a transaction from the network
    pub fn fetch(&self, tx_id: &str, fresh: bool) -> Result<Tx, Box<dyn std::error::Error>> {
        let mut cache = self.cache.borrow_mut();
        if fresh || !cache.contains_key(tx_id) {
            let url = format!("{}/tx/{}/hex", self.api_url, tx_id);
            let response = reqwest::blocking::get(url)?.text()?;
            let raw = hex::decode(response.trim())?;
            let mut cursor = Cursor::new(raw);
            let tx = Tx::parse(&mut cursor)?;

            if tx.id() != tx_id {
                return Err(format!("not the same id: {} vs {}", tx.id(), tx_id).into());
            }
            cache.insert(tx_id.to_string(), tx);
        }

        Ok(cache.get(tx_id).unwrap().clone())
    }

    /// Loads the cache from a file
    pub fn load_cache(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let disk_cache: HashMap<String, String> = serde_json::from_str(&contents)?;
        for (k, raw_hex) in disk_cache {
            let raw = hex::decode(raw_hex)?;
            let mut cursor = Cursor::new(raw);
            let tx = Tx::parse(&mut cursor)?;
            self.cache.borrow_mut().insert(k, tx);
        }

        Ok(())
    }

    /// Dumps the cache to a file
    pub fn dump_cache(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        let to_dump: HashMap<String, String> = self
            .cache
            .borrow()
            .iter()
            .map(|(k, tx)| (k.clone(), hex::encode(tx.serialize())))
            .collect();

        let serialized = serde_json::to_string_pretty(&to_dump)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }
}

/// Builder for TxFetcher
pub struct TxFetcherBuilder {
    api_url: String,
}

impl Default for TxFetcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TxFetcherBuilder {
    /// Creates a new TxFetcherBuilder
    pub fn new() -> Self {
        Self {
            api_url: "https://blockstream.info/api/".to_string(),
        }
    }

    /// Sets the API URL
    pub fn api_url(mut self, api_url: &str) -> Self {
        self.api_url = api_url.to_string();
        self
    }

    /// Sets the API URL to the testnet
    pub fn is_testnet(mut self) -> Self {
        self.api_url = "https://blockstream.info/testnet/api/".to_string();
        self
    }

    /// Builds the TxFetcher
    pub fn build(self) -> TxFetcher {
        TxFetcher {
            cache: RefCell::new(HashMap::new()),
            api_url: self.api_url,
        }
    }
}
