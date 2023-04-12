use std::{
    fmt,
    io::{Cursor, Read},
};

use crate::script::script::Script;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TxOutput {
    amount: u64,
    script_pubkey: Script,
}

impl TxOutput {
    /// Parses a transaction output from a byte vector
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, std::io::Error> {
        let mut value = [0; 8];
        cursor.read_exact(&mut value)?;
        let script_pubkey = Script::parse(cursor)?;
        Ok(TxOutput {
            amount: u64::from_le_bytes(value),
            script_pubkey,
        })
    }

    /// Serializes the transaction output into a byte vector
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Serialize amount, 8 bytes, little endian
        result.extend_from_slice(&self.amount.to_le_bytes());

        // Serialize the script_pubkey
        result.extend(self.script_pubkey.serialize());

        result
    }

    /// Returns the amount of the transaction output
    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    /// Returns the script pubkey of the transaction output
    pub fn get_script_pubkey(&self) -> Script {
        self.script_pubkey.clone()
    }
}

impl fmt::Display for TxOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TxOutput {{ value: {}, script_pubkey: {} }}",
            self.get_amount(),
            self.get_script_pubkey()
        )
    }
}
