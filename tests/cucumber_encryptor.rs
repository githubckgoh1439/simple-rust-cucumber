
use async_trait::async_trait;
use encrsypter_lib::{decryptor, encryptor};
use std::borrow::Cow;
use std::convert::Infallible;

pub struct EncrsypterTestWorld {
    encryptor: encryptor::Encryptor<'static>,
    decryptor: decryptor::Decryptor<'static>,
    encrypted_base64: String,
    decrypt_result: String,
}

#[async_trait(?Send)]
impl cucumber::World for EncrsypterTestWorld {
    type Error = Infallible;

    // Much more straightforward than the Default Trait before. :)
    async fn new() -> Result<Self, Infallible> {
        let key = &[1; 32];
        let nonce = &[3; 12];
        //let nonce = &[3; 20];

        Ok(Self {
            encryptor: encryptor::Encryptor {
                input: Cow::Borrowed(""),
                key,
                nonce,
            },
            decryptor: decryptor::Decryptor {
                file_path: "./testfile.txt",
                key,
                nonce,
            },
            encrypted_base64: "".to_string(),
            decrypt_result: "".to_string(),
        })
    }
}



mod example_steps {
    use cucumber::{Steps, t};
    use std::fs;
    use std::path::Path;
    use std::borrow::Cow;

    
pub fn steps() -> Steps<crate::EncrsypterTestWorld> {
    let mut builder: Steps<crate::EncrsypterTestWorld> = Steps::new();

    builder
        .given_regex_async(
            r#"^I have an encryptor initialized with input "([\w\s!]+)"$"#,
            t!(|mut world, texts_to_encrypt, _step| {
                world.encryptor.input = Cow::Owned(texts_to_encrypt[1].to_owned());
                world
            }),
        )
        .then_regex_async(
            r#"^I should see "([\w\s!]+)" in the encryptor's input field$"#,
            t!(|world, expected_texts, _step| {
                assert_eq!(expected_texts[1], world.encryptor.input);
                world
            }),
        )
        .when_async(
            "I encrypt the encryptor's input",
            t!(|world, _step| {
                world.encryptor.write_encrypted();
                world
            }),
        )
        .then_async(
            "testfile.txt exists",
            t!(|_world, _step| {
                let testfile_path = Path::new("./testfile.txt");
                assert_eq!(testfile_path.exists(), true);
                _world
            }),
        )
        .then_async(
            "testfile.txt is not empty",
            t!(|mut world, _step| {
                let enc_message = fs::read("./testfile.txt").expect("Could not read test file.");
                world.encrypted_base64 = base64::encode(&enc_message);

                assert_eq!(world.encrypted_base64.len() > (0 as usize), true);
                world
            }),
        )
        .when_async(
            "I decrypt testfile.txt",
            t!(|mut world, _step| {
                world.decrypt_result = world.decryptor.read_decrypted();
                world
            }),
        )
        .then_regex_async(
            r#"^the decrypted result should be "([\w\s!]+)"$"#,
            t!(|mut world, expected_texts, _step| {
                assert_eq!(expected_texts[1], world.decrypt_result);
                world
            }),
        );

        builder
    }
}


fn main() {
    // Do any setup you need to do before running the Cucumber runner.
    // e.g. setup_some_db_thing()?;
    let runner = cucumber::Cucumber::<EncrsypterTestWorld>::new()
        .features(&["./tests/features/example_encrypto/"])
        .steps(example_steps::steps());

    // You may choose any executor you like (Tokio, async-std, etc)
    // You may even have an async main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(runner.run());
}
