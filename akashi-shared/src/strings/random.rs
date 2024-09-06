use rand::distributions::Alphanumeric;
use rand::Rng;

/// Generate a string with random characters with the specified length
pub fn random_string(len: usize) -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    rand_string
}
