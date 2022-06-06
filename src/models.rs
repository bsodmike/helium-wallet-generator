use crate::error::Error;
use helium_wallet::wallet::Wallet as HeliumWallet;
use image::Luma;
use log::debug;
use qr2term::print_qr;
use qrcode::QrCode;
use rustc_serialize::base64::{ToBase64, MIME};
use std::{fs, fs::File, io::Read, path::Path};

pub struct Wallet {
    pub pub_key: Option<String>,
    pub qrcode_base64: Option<String>,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            pub_key: None,
            qrcode_base64: None,
        }
    }

    pub async fn create(&mut self) -> Result<&mut Wallet, Error> {
        let password = "pass123";
        let filename = Path::new("wallet.key");

        debug!("Starting...");

        let wallet = HeliumWallet::builder()
            .password(password)
            .output(&filename)
            .force(true)
            .create()
            .expect("it should have created a wallet");

        let keypair = wallet
            .decrypt(password.as_bytes())
            .expect("it should decrypt the wallet");
        debug!("Created wallet");

        let pub_key: String = keypair.public_key().to_string();

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
        fs::write(&path, &base64)?;
        debug!("Generated QR Code and persisted image data as base64");

        // Display QR Code in terminal
        print_qr(&pub_key)?;

        self.pub_key = Some(pub_key);
        self.qrcode_base64 = Some(base64);

        Ok(self)
    }
}
