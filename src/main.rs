use ring::digest::{Context, SHA256};
use rand::Rng;
use clap::Parser;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "")]
    verifier: String,

    #[arg(short, long, default_value = "64")]
    length: usize,
}

fn main() {

    let args = Args::parse();

    let the_given_verifier = if args.verifier.is_empty() {
        let verifier_length = args.length;
        generate_random_code_verifier(verifier_length)
    } else {
        args.verifier
    };

    let code_challenge = calculate_code_challenge(&the_given_verifier );

    let print_aligned = |key: &str, value: &str| {
        println!("{:<15}: {}", key, value);
    };

    print_aligned("code_verifier", &the_given_verifier);
    print_aligned("code_challenge",& code_challenge);

}

fn generate_random_code_verifier(verifier_length: usize) -> String {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~";
    (0..verifier_length)
    .map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    })
    .collect()
}

fn calculate_code_challenge(code_verifier: &str) -> String {
    let mut context = Context::new(&SHA256);
    context.update(code_verifier.as_bytes());
    let digest = context.finish();
    const CUSTOM_ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    let challenge:String = CUSTOM_ENGINE.encode(digest.as_ref());
    challenge
}