use crate::db::User;
use chrono::Utc;
use jsonwebtoken::{decode, encode, errors, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct JWT {
    secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    data: JWTData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTData {
    id: Uuid,
}

impl JWT {
    pub fn new(secret: String) -> Self {
        JWT { secret }
    }

    pub fn sign(&self, user: &User) -> errors::Result<String> {
        let claims = Claims {
            data: JWTData { id: user.id },
            exp: Utc::now().timestamp(),
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn verify(&self, token: String) -> errors::Result<Claims> {
        let data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(data.claims)
    }
}
