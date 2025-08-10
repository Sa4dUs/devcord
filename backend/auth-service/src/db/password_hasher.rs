use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub async fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    //Salt is the way the random part is done
    let salt = SaltString::generate(&mut OsRng);

    // If we want to change the way the password is hashed
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

//When the String is Hashed the salt is store, so we have to take the hash to get that data
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    if parsed_hash.is_err() {
        return false;
    }
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash.unwrap())
        .is_ok()
}
