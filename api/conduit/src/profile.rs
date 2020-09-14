use crate::error::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_file, query_file_as, Executor, Postgres};

pub struct ProfileService<E: Executor<'static, Database = Postgres> + Copy> {
    db: E,
}

impl<E> ProfileService<E>
where
    E: Executor<'static, Database = Postgres> + Copy,
{
    pub fn new(executor: E) -> Self {
        ProfileService { db: executor }
    }

    pub async fn get_profile(
        &self,
        username: &str,
        current_user_id: Option<i32>,
    ) -> Result<ProfileDto> {
        let profile = query_file_as!(
            ProfileDto,
            "./src/queries/get_profile.sql",
            username,
            current_user_id
        )
        .fetch_one(self.db)
        .await?;
        Ok(profile)
    }

    pub async fn follow_profile(&self, username: &str, current_user_id: i32) -> Result<ProfileDto> {
        query_file!(
            "./src/queries/follow_profile.sql",
            current_user_id,
            username
        )
        .execute(self.db)
        .await?;

        self.get_profile(username, Some(current_user_id)).await
    }

    pub async fn unfollow_profile(
        &self,
        username: &str,
        current_user_id: i32,
    ) -> Result<ProfileDto> {
        query_file!(
            "./src/queries/unfollow_profile.sql",
            username,
            current_user_id
        )
        .execute(self.db)
        .await?;

        self.get_profile(username, Some(current_user_id)).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileDto {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
