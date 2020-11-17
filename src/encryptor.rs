use crate::constants;
use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;

use std::borrow::Cow;
use std::fs::File;
use std::io::Write;

pub struct Encryptor<'a> {
    pub input: Cow<'a, str>,
    pub key: &'a [u8; constants::key_length_bytes()],
    pub nonce: &'a [u8; constants::nonce_length_bytes()], // We add the nonce as a field to stay consistent with the component model.
                                                          // In order to reach our goal not to touch the interface during refactoring.
                                                          // We could have handled the nonce within the encrypt/decrypt functions, but that would be unnecessarily ugly in our case.
}

impl<'a> Encryptor<'a> {
    pub fn write_encrypted(&self) {
        let cipher_key = GenericArray::from_slice(self.key);
        let cipher = Aes256Gcm::new(cipher_key);
        let nonce = GenericArray::from_slice(self.nonce);

        let encrypted = cipher
            .encrypt(nonce, self.input.as_bytes())
            .expect("Encryption failed.");

        let mut file = File::create("./testfile.txt").expect("Could not create the test file.");
        file.write(encrypted.as_slice())
            .expect("Could not write the encrypted data to testfile.txt.");

        println!("testfile.txt written.")
    }
}
