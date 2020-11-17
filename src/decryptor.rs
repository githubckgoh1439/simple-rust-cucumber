use crate::constants;

use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;
use std::fs;

pub struct Decryptor<'a> {
    pub file_path: &'a str,
    pub key: &'a [u8; constants::key_length_bytes()],
    pub nonce: &'a [u8; constants::nonce_length_bytes()], // We add the nonce as a field to stay consistent with the component model.
                                                          // In order to reach our goal not to touch the interface during refactoring.
                                                          // We could have handled the nonce within the encrypt/decrypt functions, but that would be unnecessarily ugly in our case.
}

impl<'a> Decryptor<'a> {
    pub fn read_decrypted(&self) -> String {
        let cipher_key = GenericArray::from_slice(self.key);
        let cipher = Aes256Gcm::new(cipher_key);
        let nonce = GenericArray::from_slice(self.nonce);

        let encrypted = fs::read("./testfile.txt").expect("Could not encrypted message file.");

        let decrypted = cipher
            .decrypt(nonce, encrypted.as_ref())
            .expect("Could not decrypt the encrypted message.");
        String::from_utf8(decrypted).expect("Could not convert the decrypted bytes into String.")
    }
}
