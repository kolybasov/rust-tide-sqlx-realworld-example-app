use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub user: UserDto,
}

impl From<UserDto> for UserResponse {
    fn from(user: UserDto) -> Self {
        UserResponse { user }
    }
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

impl From<ProfileDto> for ProfileResponse {
    fn from(profile: ProfileDto) -> Self {
        ProfileResponse { profile }
    }
}
