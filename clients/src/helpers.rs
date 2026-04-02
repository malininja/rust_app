use rand::seq::IndexedRandom;

pub fn generate_random_string(length: u8) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();

    (0..length)
        .map(|_| *CHARSET.choose(&mut rng).unwrap() as char)
        .collect()
}
