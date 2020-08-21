use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

impl UserUpdate {
    pub fn update(self, user: User) -> anyhow::Result<Self> {
        let password = if let Some(plain) = self.password {
            bcrypt::hash(&plain, bcrypt::DEFAULT_COST)?
        } else {
            user.password
        };

        Ok(UserUpdate {
            email: self.email.or(Some(user.email)),
            username: self.username.or(Some(user.username)),
            image: self.image.or(user.image),
            bio: self.bio.or(user.bio),
            password: Some(password),
        })
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub user: UserDto,
}

impl UserDto {
    pub fn with_token(user: User, token: String) -> Self {
        UserDto {
            id: user.id,
            email: user.email,
            username: user.username,
            bio: user.bio,
            image: user.image,
            created_at: user.created_at,
            updated_at: user.updated_at,
            token,
        }
    }
}
