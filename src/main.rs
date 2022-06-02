#![allow(dead_code)]

use crate::error::Error;
use helium_wallet::wallet::Wallet;
use image::Luma;
use log::debug;
use qrcode::QrCode;
use std::path::Path;

mod error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let password = "pass123";
    let filename = Path::new("wallet.key");

    debug!("Starting...");

    let wallet = Wallet::builder()
        .password(password)
        .output(&filename)
        .force(true)
        .create()
        .expect("it should have created a wallet");
    debug!("Created wallet");

    let keypair = wallet
        .decrypt(password.as_bytes())
        .expect("it should decrypt the wallet");
    dbg!(&keypair);

    let pub_key: String = keypair.public_key().to_string();
    debug!("Wallet public key: {}", &pub_key);

    let code = QrCode::new(&pub_key)?;
    let image = code.render::<Luma<u8>>().build();
    let filename = format!("{}_qr.png", pub_key);
    image.save(&filename)?;

    Ok(())
}
