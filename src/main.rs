#![allow(dead_code)]

use crate::error::Error;
use helium_wallet::wallet::Wallet;
use image::Luma;
use log::debug;
use qr2term::print_qr;
use qrcode::QrCode;
use rustc_serialize::base64::{ToBase64, MIME};
use std::{fs, fs::File, io::Read, path::Path};

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

    // Save QR code image as base64 to file
    let path = format!("./{}", &filename);
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let mut base64 = vec.to_base64(MIME);
    base64 = base64.replace("\r\n", "");

    let filename = format!("{}_qr.base64", pub_key);
    let path = format!("./{}", filename);
    fs::write(&path, base64)?;

    // Display QR Code in terminal
    print_qr(&pub_key)?;

    Ok(())
}
