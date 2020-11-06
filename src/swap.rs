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
    //In the vari