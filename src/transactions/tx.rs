use std::{
    fmt,
    io::{Cursor, Error, ErrorKind, Read},
};

use crate::utils::{
    hash256::hash256,
    varint::{encode_varint, read_varint},
};

use super::{input::TxInput, output::TxOutput};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tx {
    version: u32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: Locktime,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Locktime {
    BlockHeight(u32),
    UnixTimestamp(u32),
}

impl Tx {
    /// Parses a transaction from a byte stream
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut version = vec![0; 4];

        stream
            .read_exact(&mut version)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Invalid version: {}", e)))?;

        let version = u32::from_le_bytes(
            version
                .try_into()
                .map_err(|_| Error::new(ErrorKind::Other, "Invalid version"))?,
        );

        let mut inputs = vec![];
        if let Ok(num_inputs) = read_varint(stream) {
            for _ in 0..num_inputs {
                let input = TxInput::parse(stream)
                    .map_err(|_| Error::new(ErrorKind::Other, "Invalid input"))?;
                inputs.push(input);
            }
        }

        let mut outputs = vec![];
        if let Ok(num_outputs) = read_varint(stream) {
            for _ in 0..num_outputs {
                let output = TxOutput::parse(stream)
                    .map_err(|_| Error::new(ErrorKind::Other, "Invalid output"))?;

                outputs.push(output);
            }
        }

        let mut locktime_bytes = vec![0; 4];
        stream.read_exact(&mut locktime_bytes).unwrap();
        let locktime_value = u32::from_le_bytes(locktime_bytes.try_into().unwrap());

        let locktime = if locktime_value >= 500_000_000 {
            Locktime::UnixTimestamp(locktime_value)
        } else {
            Locktime::BlockHeight(locktime_value)
        };

        Ok(Tx {
            version,
            inputs,
            outputs,
            locktime,
        })
    }

    /// Returns the byte serialization of the transaction
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Serialize version in little endian
        let version_le = self.version.to_le_bytes().to_vec();
        result.extend(version_le);

        // Serialize inputs
        let inputs = self.inputs.clone();
        result.extend_from_slice(&encode_varint(inputs.len() as u64).unwrap());

        for input in inputs {
            result.extend(input.serialize());
        }

        // Serialize outputs
        let outputs = self.outputs.clone();
        result.extend_from_slice(&encode_varint(outputs.len() as u64).unwrap());
        for output in outputs {
            result.extend(output.serialize());
        }

        // Serialize locktime in little endian
        let locktime_le = match self.locktime {
            Locktime::BlockHeight(value) => value.to_le_bytes().to_vec(),
            Locktime::UnixTimestamp(value) => value.to_le_bytes().to_vec(),
        };
        result.extend(locktime_le);

        result
    }

    /// Returns the transaction id
    pub fn id(&self) -> String {
        hex::encode(self.hash())
    }

    fn hash(&self) -> Vec<u8> {
        let bytes = self.serialize();
        let mut hash = hash256(&bytes);
        hash.reverse();
        hash.to_vec()
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_inputs(&self) -> Vec<TxInput> {
        self.inputs.clone()
    }

    pub fn get_outputs(&self) -> Vec<TxOutput> {
        self.outputs.clone()
    }

    pub fn get_locktime(&self) -> Locktime {
        self.locktime.clone()
    }
}

impl fmt::Display for Tx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "version: {}, inputs: {}, outputs: {}, locktime: {}",
            self.version,
            self.inputs.len(),
            self.outputs.len(),
            match self.locktime {
                Locktime::BlockHeight(value) => format!("BlockHeight({})", value),
                Locktime::UnixTimestamp(value) => format!("UnixTimestamp({})", value),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_parse_version() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream).unwrap();
        assert_eq!(tx.get_version(), 1);
    }

    #[test]
    fn test_parse_inputs() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream).unwrap();
        assert_eq!(tx.get_inputs().len(), 1);

        let input = tx.get_inputs().get(0).unwrap().clone();
        let want = String::from("d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81");
        assert_eq!(input.get_prev_tx(), want);
        assert_eq!(input.get_prev_index(), 0);

        let want = hex::decode("6b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a").unwrap();
        assert_eq!(input.get_script_sig().serialize(), want);
        assert_eq!(input.get_sequence(), 0xfffffffe);
    }

    #[test]
    fn test_parse_outputs() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream).unwrap();
        assert_eq!(tx.get_outputs().len(), 2);

        let output = tx.get_outputs().get(0).unwrap().clone();
        assert_eq!(output.get_amount(), 32454049);
        let want = hex::decode("1976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac").unwrap();
        assert_eq!(output.get_script_pubkey().serialize(), want);

        let output = tx.get_outputs().get(1).unwrap().clone();
        assert_eq!(output.get_amount(), 10011545);
        let want = hex::decode("1976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac").unwrap();
        assert_eq!(output.get_script_pubkey().serialize(), want);
    }

    #[test]
    fn test_parse_locktime() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream).unwrap();
        assert_eq!(tx.get_locktime(), Locktime::BlockHeight(410393));
    }
}
