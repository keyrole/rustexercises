use rand::prelude::*;

pub fn gen_password() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = thread_rng();

    let password: String = (0..PASSWORD_LEN).map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    }).collect();

    password
}

pub fn gen_alphanumeric(len: usize) -> String {
    use rand::distributions::{Alphanumeric, DistString};
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), len);
    string
}