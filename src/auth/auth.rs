use chrono::Utc;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
struct Claim {
    sub: String,
    exp: u64,
    iat: u64,
}

fn user_encode(claim: &Claim) -> String {
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("Key".as_ref()),
    )
    .unwrap()
}

#[test]
fn test_user_encode() {
    let token = user_encode(&Claim {
        sub: "arash".to_owned(),
        exp: jsonwebtoken::get_current_timestamp() + 3600,
        iat: jsonwebtoken::get_current_timestamp(),
    });
    println!("{}", token);
    println!("%%%%%%%%%%%%%%%%");
    println!(
        "{:#?}",
        decode::<Claim>(
            &token,
            &DecodingKey::from_secret("Key".as_ref()),
            &Validation::default(),
        )
        .unwrap()
    );
}
