use hex;
use sha2::{Digest, Sha512};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    hash_password(password) == hashed_password
}
