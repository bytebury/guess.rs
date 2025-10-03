use std::env;

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Serialize, de::DeserializeOwned};

pub mod user_claims;

pub struct JwtService {}

impl JwtService {
    pub fn generate<T: Serialize>(claims: &T) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::new(Algorithm::HS256),
            claims,
            &EncodingKey::from_secret(
                env::var("JWT_SECRET")
                    .expect("JWT_SECRET is not defined")
                    .as_bytes(),
            ),
        )
    }

    pub fn verify<T: DeserializeOwned>(
        token: &str,
    ) -> Result<TokenData<T>, jsonwebtoken::errors::Error> {
        decode::<T>(
            token,
            &DecodingKey::from_secret(
                env::var("JWT_SECRET")
                    .expect("JWT_SECRET is not defined")
                    .as_bytes(),
            ),
            &Validation::new(Algorithm::HS256),
        )
    }
}
