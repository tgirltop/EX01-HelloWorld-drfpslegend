//use reqwest::Error;
//************************** Sending a single transaction ************************************** 

use web3::types::{TransactionRequest};
use web3::transports::Http;
use web3::{Web3};


async fn send_eth_transaction(
    _metamask_address: &str,
    _metamask_private_key: &str,
    _recipient_address: &str,
    _amount: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    //In the variable below, you just have to add your RPC provider Key.
    //You can choose testnet (goerli, Sepolia...etc), and your provider (Alchemy, Infura for example)
    //Note that the method can change because of your provider, for example Infura can struggle with
    //web3::{Web3} crate because of .web3.personal() method.
    let transport = Http::new("...").unwrap();
    let web3 = Web3::new(transport);

    //Note that u can use the "serde_JSON" crate to get sensitives informations from JSON file, a good practice
    //to protect Adress and private key.
    let transaction = TransactionRequest{
        from: "ENTER YOUR ADRESS HERE".parse().unwrap(),
        to: Some("ENTER RECIPIENT ADRESS HERE".parse().unwrap()),
        gas: None,
        gas_price: Some(1000000000.into()),
        value: Some(12345.into()),
        nonce: None,
        data: None,
        access_list: None,
        condition: None,
        max_priority_fee_per_gas: None,
        max_fee_per_gas: None,
        transaction_type: None,
    };
    
    let signed_transaction = web3.personal().sign_transaction(transaction, _metamask_private_key).await?;
    let transaction_hash = web3.eth().send_raw_transaction(signed_transaction.raw).await?;
    
    
    //Wait for the transaction to be confirmed on the testnet
    let receipt = loop {
        match web3.eth().transaction_receipt(transaction_hash).await {
            Ok(receipt) => break receipt,
            Err(_) => continue,
        }
    };

    println!("Transaction confirmed! Receipt: {:?}", receipt);

    Ok(())
}

#[tokio::main]
async fn main() {
    let metamask_address = "...";
    let metamask_private_key = "...";
    let recipient_address = "...";
    let amount = 100000000000000000; // 0.1 ETH int this example

    match