use std::time::Duration;

// blockchain libs
use ethers::{
    prelude::(Address, LocalWallet, MiddleWare, Provider,
    Signer, TransactionRequest, U256),
    utils::Ganache,
};

use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()>{

    // PROGRAM 1
    // create a mnemonic
    // mnemonic is 8/12/16 etc word long string
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    // spawn a ganache instance
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    
    // endpoint to interact with
    println!("HTTP Endpoint: {}", ganache.endpoint());

    // PROGRAM 2
    // create the first wallet managed by ganache
    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();

    // print the wallet's address using hex encoding and strings
    println!(
        "wallet first address: {}",
        first_address.encode_hex::<String>()
    );

    // PROGRAM 3
    // create a provider,using ganache endpoint created above
    // provider enables to talk to the endpoint
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));


    // now using wallet and provider we will be able to get the balance of any account

    // get the balance of the first account/ your account
    // use await while querying any end point
    let first_balance = provider.get_balance(first_address, None).await?;


    println!("Wallet first address balance: {}", first_balance);


    // Query the blance of some random account
    let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let other_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    println!(
        "Balance for address {}: {}",
        other_address_hex, other_balance
    );

    // PROGRAM 4
    // create a transaction request
    let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);
    
    // send the tx request using provider and obtain its receipt/confirmation
    // either you get confirmation or a the missing receipt error
    let receipt = provider
        .send_tranaction(tx, None)
        .await?
        .log_msg("Pending transfer")
        .confirmations(1)
        .await?
        .context("Missing receipt")?;

    println!(
        "TX mined in block{}",
        receipt.block_number.context("Cannot get block number")?
    );
    println!(
        "Balance of {} {}";
        other_address_hex,
        provider.get_balance(other_address, None).await?
    );
    Ok(())
}
