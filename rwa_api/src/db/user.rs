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
pub struct UserResponse<'a> {
    pub user: UserDto<'a>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDto<'a> {
    pub email: &'a str,
    pub token: &'a str,
    pub username: &'a str,
    pub bio: Option<&'a str>,
    pub image: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileDto<'a> {
    pub username: &'a str,
    pub bio: Option<&'a str>,
    pub image: Option<&'a str>,
    pub following: bool,
}

impl<'a> From<&'a User> for ProfileDto<'a> {
    fn from(user: &'a User) -> Self {
        ProfileDto {
            username: &user.username,
            bio: user.bio.as_deref(),
            image: user.image.as_deref(),
            following: false,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ProfileResponse<'a> {
    pub profile: ProfileDto<'a>,
}

impl<'a> UserDto<'a> {
    pub fn with_token(user: &'a User, token: &'a str) -> Self {
        UserDto {
            email: &user.email,
            username: &user.username,
            bio: user.bio.as_deref(),
            image: user.image.as_deref(),
            token,
        }
    }
}
