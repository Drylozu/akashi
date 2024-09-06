use rand::distributions::Alphanumeric;
use rand::Rng;

/// Generate a string with random characters with the specified length
pub fn random_str<'a>(len: usize) -> &'a str {
    let rand_string = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    rand_string
}