use std::{
    fmt,
    io::{Cursor, Error, Read},
};

use crate::script::script::Script;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TxInput {
    prev_tx: Vec<u8>,
    prev_index: Vec<u8>,
    script_sig: Script,
    sequence: Vec<u8>,
}

impl TxInput {
    /// Parses a transaction input from a byte stream
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut prev_tx = vec![0; 32];
        cursor.read_exact(&mut prev_tx)?;
        let mut prev_index = vec![0; 4];
        cursor.read_exact(&mut prev_index)?;
        let script_sig = Script::parse(cursor)?;
        let mut sequence = vec![0; 4];
        cursor.read_exact(&mut sequence)?;

        Ok(TxInput {
            prev_tx,
            prev_index,
            script_sig,
            sequence,
        })
    }

    /// Returns the byte serialization of the transaction input
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Serialize prev_tx (no need to reverse)
        result.extend(&self.prev_tx);

        // Serialize prev_index in little endian
        let prev_index_le = self.prev_index.clone();
        result.extend(prev_index_le);

        // Serialize script_sig
        result.extend(self.script_sig.serialize());

        // Serialize sequence in little endian
        let sequence = self.sequence.clone();
        result.extend(sequence);

        result
    }

    /// Returns the previous transaction hash
    pub fn get_prev_tx(&self) -> String {
        let mut value = self.prev_tx.clone();
        value.reverse();
        hex::encode(value)
    }

    /// Returns the previous transaction index
    pub fn get_prev_index(&self) -> u32 {
        u32::from_le_bytes(self.prev_index.as_slice().try_into().unwrap())
    }

    /// Returns the script signature
    pub fn get_script_sig(&self) -> Script {
        self.script_sig.clone()
    }

    /// Returns the sequence number
    pub fn get_sequence(&self) -> u32 {
        u32::from_le_bytes(self.sequence.as_slice().try_into().unwrap())
    }
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TxInput {{ prev_tx: {}, prev_index: {}, script_sig: {}, sequence: {} }}",
            self.get_prev_tx(),
            self.get_prev_index(),
            self.get_script_sig(),
            self.get_sequence()
        )
    }
}
