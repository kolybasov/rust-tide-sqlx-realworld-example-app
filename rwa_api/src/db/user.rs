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
pub struct UserResponse {
    pub user: UserDto,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileDto {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl From<User> for ProfileDto {
    fn from(user: User) -> Self {
        ProfileDto {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following: false,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ProfileResponse {
    pub profile: ProfileDto,
}

impl UserDto {
    pub fn with_token(user: User, token: String) -> Self {
        UserDto {
            email: user.email,
            username: user.username,
            bio: user.bio,
            image: user.image,
            token,
        }
    }
}
