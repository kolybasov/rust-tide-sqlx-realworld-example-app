use crate::db::User;
use chrono::Utc;
use jsonwebtoken::{decode, encode, errors, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct JWT {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey<'static>,
    headers: Header,
    validations: Validation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    pub data: JWTData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTData {
    pub id: i32,
}

impl JWT {
    pub fn new(secret: &str) -> Self {
        let bytes = secret.as_bytes();

        JWT {
            encoding_key: EncodingKey::from_secret(bytes),
            decoding_key: DecodingKey::from_secret(bytes).into_static(),
            headers: Header::default(),
            validations: Validation::default(),
        }
    }

    pub fn sign(&self, user: &User) -> errors::Result<String> {
        let claims = Claims {
            data: JWTData { id: user.id },
            exp: Utc::now().timestamp() + 604800,
        };
        encode(&self.headers, &claims, &self.encoding_key)
    }

    pub fn verify(&self, token: &str) -> errors::Result<Claims> {
        let data = decode::<Claims>(token, &self.decoding_key, &self.validations)?;
        Ok(data.claims)
    }
}
