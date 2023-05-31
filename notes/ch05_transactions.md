# Transactions

## Components
1. Version 
2. Inputs
3. Outputs 
4. Locktime

### Version
The version indicates what addi‐ tional features the transaction uses, inputs define what bitcoins are being spent, out‐ puts define where the bitcoins are going, and locktime defines when this transaction starts being valid. 

### Inputs
Each input contains four fields. The first two fields point to the previous transaction output and the last two fields define how the previous transaction output can be spent. The fields are as follows:

• Previous transaction ID
• Previous transaction index 
• ScriptSig
• Sequence

The previous transaction ID is the hash256 of the previous transaction’s contents. 
Each transaction has to have at least one output, but may have many. Thus, we need to define exactly which output within a transaction we’re spending, which is captured in the previous transaction index.

Note that the previous transaction ID is 32 bytes and that the previous transaction index is 4 bytes. Both are in little-endian.

The ScriptSig field is a variable-length field, not a fixed-length field like most of what we’ve seen so far. A variable-length field requires us to define exactly how long the field will be, which is why the field is preceded by a varint telling us how long it is.

The sequence is also in little-endian and takes up 4 bytes.

### Outputs
Outputs define where the bitcoins are going. Each transaction must have one or more outputs.
Like with inputs, output serialization starts with how many outputs there are as a varint.
Each output has two fields: amount and ScriptPubKey. 
The amount is the amount of bitcoins being assigned and is specified in satoshis, or 1/100,000,000ths of a bitcoin. The absolute maximum for the amount is the asymptotic limit of 21 million bitcoins in satoshis, which is 2,100,000,000,000,000 (2,100 trillion) satoshis. This number is greater than 232 (4.3 billion or so) and is thus stored in 64 bits, or 8 bytes. The amount is serialized in little-endian.
The ScriptPubKey, like the ScriptSig, has to do with Bitcoin’s smart contract language, Script. Think of the ScriptPubKey as the locked box that can only be opened by the holder of the key. It’s like a one-way safe that can receive deposits from anyone, but can only be opened by the owner of the safe. Like ScriptSig, ScriptPubKey is a variable-length field and is preceded by the length of the field in a varint.

### Locktime
Locktime is a way to time-delay a transaction. A transaction with a locktime of 600,000 cannot go into the blockchain until block 600,001. 

Note that locktime is ignored if the sequence numbers for every input are ffffffff.

The serialization is in little-endian and 4 bytes.

## Transaction Fee
One of the consensus rules of Bitcoin is that for any non-coinbase transactions, the sum of the inputs has to be greater than or equal to the sum of the outputs. 

The transaction fee is simply the sum of the inputs minus the sum of the outputs. As inputs don’t have an amount field, we have to look up the amount. This requires access to the blockchain, specifically the UTXO set. If you are not running a full node, this can be tricky, as you now need to trust some other entity to provide you with this information.