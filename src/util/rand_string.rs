use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn random_string(len: u64) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len as usize)
        .collect();
    // println!("{}", rand_string);
    rand_string
}