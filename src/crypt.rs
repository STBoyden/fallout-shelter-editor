use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

use aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit},
    Aes256,
};
use base64::{engine::general_purpose, Engine as _};
use sha1::Sha1;

use crate::data::Save;

const PASSWORD: &[u8] = b"UGxheWVy";
const IV: &[u8] = b"tu89geji340t89u2";

type AesDec = cbc::Decryptor<Aes256>;
type AesEnc = cbc::Encryptor<Aes256>;

pub fn decrypt(path: PathBuf) -> anyhow::Result<Save> {
    let file = File::open(&path)?;

    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = vec![];

    reader.read_to_end(&mut buffer)?;

    let data = String::from_utf8(buffer.clone())?;

    let backup_file = File::create(format!("{}.bak", path.display()))?;
    let mut backup_writer = BufWriter::new(backup_file);
    backup_writer.write_all(&buffer)?;

    let mut decoded = general_purpose::STANDARD.decode(&data)?;

    let key = pbkdf2::pbkdf2_hmac_array::<Sha1, 32>(PASSWORD, IV, 1000);
    let decryptor = AesDec::new(&key.into(), IV.into());

    let bytes = match decryptor.decrypt_padded_mut::<Pkcs7>(&mut decoded) {
        Ok(bytes) => bytes,
        Err(e) => panic!("Could not decrypt file: {}", e),
    };

    let json_string = String::from_utf8(bytes.to_vec())?;

    let dump_file = File::create(format!("{}.dump.json", path.display()))?;
    let mut dump_file_writer = BufWriter::new(dump_file);
    dump_file_writer.write_all(json_string.as_bytes())?;

    let vault_data: Save = serde_json::from_str(&json_string)?;

    Ok(vault_data)
}
pub fn encrypt(save_data: &Save, path: PathBuf) -> anyhow::Result<()> {
    let save_data_str = serde_json::to_string(&save_data)?;
    let save_data_bytes = save_data_str.as_bytes();
    let data_length = save_data_bytes.len().clone();

    let key = pbkdf2::pbkdf2_hmac_array::<Sha1, 32>(PASSWORD, IV, 1000);
    let encryptor = AesEnc::new(&key.into(), IV.into());

    let mut buffer = vec![0u8; data_length * 2];
    buffer[..data_length].copy_from_slice(save_data_bytes);

    let bytes = match encryptor.encrypt_padded_mut::<Pkcs7>(&mut buffer, data_length) {
        Ok(bytes) => bytes,
        Err(e) => panic!("Could not encrypt data: {}", e),
    };

    let mut buffer = String::new();
    buffer = buffer.replace("\0", "\n");
    general_purpose::STANDARD.encode_string(&bytes, &mut buffer);

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(buffer.as_bytes())?;

    Ok(())
}
