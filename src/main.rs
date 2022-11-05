// https://tms-dev-blog.com/how-to-use-jwt-with-rust-learn-the-basics/

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{thread, time};

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
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
        exp: my_exp as usize,
        iat: my_iat as usize,
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
    println!("wait ...");
    thread::sleep(time::Duration::from_secs(7));

    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::default(),

    ) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("err: {:?}", err.kind());
            panic!()
        }
    };

    println!("token data: {:?}", token_data);
}
