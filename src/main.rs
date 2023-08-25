use ring::digest::{Context, SHA256};
use rand::Rng;

fn main() {
    let code_verifier = generate_random_code_verifier();

    let code_challenge = calculate_code_challenge(&code_verifier);

    println!("Code verifier: {}", code_verifier);
    println!("Code challenge: {}", code_challenge);

}

fn generate_random_code_verifier() -> String {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~";
    const VERIFIER_LEN: usize = 64;

    (0..VERIFIER_LEN)
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
    base64::encode_config(digest.as_ref(), base64::URL_SAFE_NO_PAD)
}