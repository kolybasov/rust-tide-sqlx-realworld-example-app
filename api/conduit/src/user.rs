use crate::error::{Error, Result};
use crate::jwt::JWT;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{query_file_as, Executor, Postgres};

#[derive(Debug, Deserialize)]
pub struct RegisterParams {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserParams {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

pub struct UserService<E: Executor<'static, Database = Postgres> + Copy> {
    db: E,
}

impl<E> UserService<E>
where
    E: Executor<'static, Database = Postgres> + Copy,
{
    pub fn new(executor: E) -> Self {
        UserService { db: executor }
    }

    pub async fn register(&self, params: &RegisterParams, jwt: &JWT) -> Result<UserDto> {
        let password = bcrypt::hash(&params.password, bcrypt::DEFAULT_COST)?;
        let user = query_file_as!(
            User,
            "./src/queries/create_user.sql",
            params.email,
            password,
            params.username
        )
        .fetch_one(self.db)
        .await?;

        let token = jwt.sign(&user)?;
        Ok(UserDto::with_token(user, token))
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<User> {
        let user = query_file_as!(User, "./src/queries/get_user.sql", id)
            .fetch_one(self.db)
            .await?;
        Ok(user)
    }

    pub async fn login(&self, params: &LoginParams, jwt: &JWT) -> Result<UserDto> {
        let user: User = query_file_as!(User, "./src/queries/get_user_by_email.sql", params.email)
            .fetch_one(self.db)
            .await?;

        if bcrypt::verify(&params.password, &user.password)? {
            let token = jwt.sign(&user)?;
            Ok(UserDto::with_token(user, token))
        } else {
            Err(Error::InvalidPassword)
        }
    }

    pub async fn update_user(
        &self,
        params: &UpdateUserParams,
        user: &User,
        jwt: &JWT,
    ) -> Result<UserDto> {
        let token = jwt.sign(&user)?;
        let password = if let Some(new_password) = &params.password {
            let hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)?;
            Some(hash)
        } else {
            None
        };

        let updated_user = query_file_as!(
            User,
            "./src/queries/update_user.sql",
            user.id,
            params.email,
            params.username,
            password,
            params.image.as_deref().or(user.image.as_deref()),
            params.bio.as_deref().or(user.bio.as_deref())
        )
        .fetch_one(self.db)
        .await?;

        Ok(UserDto::with_token(updated_user, token))
    }
}

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
