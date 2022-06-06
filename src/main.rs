#![allow(dead_code)]

use crate::error::Error;
use models::Wallet;
use prettytable::{cell, row, Table};

mod error;
pub mod models;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let mut wallet_obj = Wallet::new();
    let wallet = wallet_obj.create().await?;
    let default_value = String::default();

    let pub_key = wallet.pub_key.as_ref().unwrap_or(&default_value);
    let mut table = Table::new();
    table.add_row(row!["Public key", &pub_key]);
    table.printstd();

    Ok(())
}
