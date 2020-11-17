extern crate rand;

use encrsypter_lib::constants;
use encrsypter_lib::decryptor::Decryptor;
use encrsypter_lib::encryptor::Encryptor;
use rand::rngs::OsRng;
use rand::Rng;
use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let sub_cmd = cli_args
        .get(1)
        .expect("Could not get the first CLI-Argument!");

    if !File::open("./keyfile").is_ok() {
        let mut key_file = File::create("./keyfile").expect("Could not create the key file.");
        let key: [u8; constants::key_length_bytes()] = OsRng.gen();
        key_file
            .write_all(&key)
            .expect("Could not write the key to the key file.");
    }

    if !File::open("./nonce").is_ok() {
        let mut nonce_file =
            File::create("./nonce").expect("Could not create the nonce data file.");
        let nonce: [u8; constants::nonce_length_bytes()] = OsRng.gen();
        nonce_file
            .write_all(&nonce)
            .expect("Could not write the key to the key file.");
    }

    let mut key: [u8; constants::key_length_bytes()] = [0; constants::key_length_bytes()];
    File::open("./keyfile")
        .expect("Could not open the key file")
        .read(&mut key)
        .expect("Could not read from the key file.");

    let mut nonce: [u8; constants::nonce_length_bytes()] = [0; constants::nonce_length_bytes()];
    File::open("./nonce")
        .expect("Could not open the nonce file")
        .read(&mut nonce)
        .expect("Could not read from the nonce file.");

    match sub_cmd.as_ref() {
        "encrypt" => Encryptor {
            input: Cow::Borrowed("Hello Rust!"),
            //input: Cow::Borrowed("hi!"),
            key: &key,
            nonce: &nonce,
        }
        .write_encrypted(),

        "decrypt" => println!(
            "{}",
            Decryptor {
                file_path: "./testfile.txt",
                key: &key,
                nonce: &nonce
            }
            .read_decrypted()
        ),
        _ => panic!("Unknown sub command: {}", sub_cmd),
    }
}
