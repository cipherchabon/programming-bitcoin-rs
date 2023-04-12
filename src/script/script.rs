use core::fmt;
use std::io::{Cursor, Error, Read};

use super::op::create_op_code_names;
use crate::utils::varint::read_varint;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Script {
    cmds: Vec<Vec<u8>>,
}

impl Script {
    /// Parses a script from a byte vector
    pub fn parse(reader: &mut Cursor<Vec<u8>>) -> Result<Script, Error> {
        let mut cmds = vec![];
        let mut count = 0;
        let length = read_varint(reader)?;
        while count < length {
            let mut current = [0u8; 1];
            reader.read_exact(&mut current)?;
            count += 1;
            let current_byte = current[0];
            if current_byte >= 1 && current_byte <= 75 {
                let n = current_byte;
                let mut cmd = vec![0u8; n as usize];
                reader.read_exact(&mut cmd)?;
                cmds.push(cmd);
                count += n as u64;
            } else if current_byte == 76 {
                let data_length = read_varint(reader)?;
                let mut cmd = vec![0u8; data_length as usize];
                reader.read_exact(&mut cmd)?;
                cmds.push(cmd);
                count += data_length as u64 + 1;
            } else if current_byte == 77 {
                let data_length = read_varint(reader)?;
                let mut cmd = vec![0u8; data_length as usize];
                reader.read_exact(&mut cmd)?;
                cmds.push(cmd);
                count += data_length as u64 + 2;
            } else {
                let op_code = current_byte;
                cmds.push(vec![op_code]);
            }
        }
        if count != length {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "parsing script failed",
            ));
        }
        Ok(Script { cmds })
    }

    fn raw_serialize(&self) -> Vec<u8> {
        let mut result = vec![];
        for cmd in &self.cmds {
            if cmd.len() == 1 {
                let op_code = cmd[0];
                result.push(op_code);
            } else {
                let length = cmd.len();
                if length < 76 {
                    result.push(length as u8);
                } else if length <= 0xff {
                    result.push(76);
                    result.push(length as u8);
                } else if length <= 520 {
                    result.push(77);
                    result.extend_from_slice(&length.to_le_bytes()[..2]);
                } else {
                    panic!("too long a cmd");
                }
                result.extend_from_slice(&cmd);
            }
        }
        result
    }

    /// Serializes the script into a byte vector
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = self.raw_serialize();
        let total = result.len();
        let mut length_bytes = vec![];
        if total < 0xfd {
            length_bytes.push(total as u8);
        } else if total <= 0xffff {
            length_bytes.push(0xfd);
            length_bytes.extend_from_slice(&(total as u16).to_le_bytes());
        } else {
            length_bytes.push(0xfe);
            length_bytes.extend_from_slice(&(total as u32).to_le_bytes());
        }
        length_bytes.append(&mut result);
        length_bytes
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_code_names = create_op_code_names();
        let mut result = String::new();

        for cmd in &self.cmds {
            if cmd.len() == 1 {
                let op_code = cmd[0];
                result.push_str(&op_code_names[&op_code]);
            } else {
                result.push_str(
                    &cmd.iter()
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<String>(),
                );
            }
            result.push(' ');
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let script_pubkey = hex::decode("6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937").unwrap();
        // cursor
        let mut script_pubkey = Cursor::new(script_pubkey);
        let script = Script::parse(&mut script_pubkey).unwrap();
        let want = hex::decode("304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a71601").unwrap();
        assert_eq!(script.cmds[0], want);

        let want =
            hex::decode("035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937")
                .unwrap();
        assert_eq!(script.cmds[1], want);
    }

    #[test]
    fn test_serialize() {
        let want = "6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937";
        let script_pubkey = hex::decode(want).unwrap();
        let mut script_pubkey = Cursor::new(script_pubkey);
        let script = Script::parse(&mut script_pubkey).unwrap();
        assert_eq!(hex::encode(script.serialize()), want);
    }
}
