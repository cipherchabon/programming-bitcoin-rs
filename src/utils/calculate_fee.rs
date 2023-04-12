use crate::transactions::{input::TxInput, tx::Tx};

use super::tx_fetcher::TxFetcher;

/// Calculates the fee of a transaction
pub fn calculate_fee(tx: &Tx, fetcher: &TxFetcher) -> Result<u64, Box<dyn std::error::Error>> {
    let mut input_sum = 0;
    for input in tx.get_inputs() {
        input_sum += get_input_value(&input, fetcher)?;
    }

    let mut output_sum = 0;
    for output in tx.get_outputs() {
        output_sum += output.get_amount();
    }

    if input_sum < output_sum {
        return Err("Invalid transaction: output sum is greater than input sum".into());
    }

    Ok(input_sum - output_sum)
}

/// Returns the value of the input
fn get_input_value(
    input: &TxInput,
    fetcher: &TxFetcher,
) -> Result<u64, Box<dyn std::error::Error>> {
    let prev_tx_hex = input.get_prev_tx();

    let prev_tx = fetcher.fetch(&prev_tx_hex, false)?;

    let prev_outputs = prev_tx.get_outputs();

    let prev_index = input.get_prev_index();

    let prev_output = prev_outputs
        .get(prev_index as usize)
        .ok_or("Previous output not found")?;

    Ok(prev_output.get_amount())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_fee() {
        let fetcher = TxFetcher::builder().build();

        let values = vec![
            (
                "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff\
                192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f6780\
                1c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02\
                a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5\
                423e332166702cb75f40df79fea1288ac19430600",
                40000,
            ),
            (
                "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcde\
                f85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4\
                e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f\
                51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179\
                928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071\
                c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a\
                3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb\
                5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158\
                b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf73\
                6c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f\
                6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353\
                a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d47\
                50b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600",
                140500,
            ),
        ];

        for (raw_tx, fee) in values {
            let mut stream = Cursor::new(hex::decode(raw_tx).unwrap());
            let tx = Tx::parse(&mut stream).unwrap();

            let value = calculate_fee(&tx, &fetcher).unwrap();
            assert_eq!(fee, value);
        }
    }
}
