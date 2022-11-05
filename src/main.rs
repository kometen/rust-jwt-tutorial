// https://tms-dev-blog.com/how-to-use-jwt-with-rust-learn-the-basics/

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    test: String,
}

fn main() {
    let key = b"this is my secret";
    let my_iat = Utc::now().timestamp();
    let my_exp = Utc::now()
        .checked_add_signed(Duration::seconds(5))
        .expect("invalid timestamp")
        .timestamp();
    let my_claims = Claims {
        sub: "claus@gnome.no".to_owned(),
        iat: my_iat as usize,
        exp: my_exp as usize,
        test: "jeg æder blåbærsyltetøj".to_owned(),
    };

    let token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    println!("token: {:?}", token);
}