use alloy_consensus::TxLegacy;
use alloy_consensus::TxEnvelope;
use alloy_consensus::SignableTransaction;
use alloy_primitives::{Bytes, Address, TxKind};
use alloy_rlp::Encodable;
use alloy_rpc_types::request::{TransactionInput, TransactionRequest};
use alloy_network::TxSignerSync;
use alloy_sol_types::{
    sol, sol_data::{Array}, 
    SolCall, SolEvent, SolType, SolValue
};
use alloy_signer::{Signer, SignerSync};
use alloy_signer_wallet::LocalWallet;

use std::str::FromStr; // Import the FromStr trait
use hex;

sol! {
    function replicate(
        bytes calldata name,
        bytes calldata initialization,
        bytes calldata erc721Data,
        address implementation
    ) external returns (
        address tba
    );
}

fn main() -> anyhow::Result<()> {

    let data = replicateCall {
        name: Bytes::from("odinsbadeye"),
        initialization: Bytes::from(""),
        erc721Data: Bytes::from(""),
        implementation: Address::from_str("0x6a21cf6AEA5C33666C3FB515ABd44EB49ca2FE50")?,
    }.abi_encode();

    println!("incorrect encoding: {:?}", hex::encode(data));

    println!("correct encoding  : \"0x6fc310f8000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000006a21cf6aea5c33666c3fb515abd44eb49ca2fe50000000000000000000000000000000000000000000000000000000000000000b6f64696e7362616465796500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\"");

    Ok(())
    
}