use super::ProfileDto;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{query_file, query_file_as, Executor, Postgres, Result};

#[derive(Debug, Deserialize)]
pub struct CreateCommentParams {
    pub body: String,
}

pub struct CommentService<E: Executor<'static, Database = Postgres> + Copy> {
    db: E,
}

impl<E> CommentService<E>
where
    E: Executor<'static, Database = Postgres> + Copy,
{
    pub fn new(executor: E) -> Self {
        CommentService { db: executor }
    }

    pub async fn get_comments(
        &self,
        article_slug: &str,
        current_user_id: Option<i32>,
    ) -> Result<Vec<CommentDto>> {
        let comments = query_file_as!(
            CommentRow,
            "./src/queries/get_comments.sql",
            None::<i32>,
            article_slug,
            current_user_id
        )
        .fetch_all(self.db)
        .await?;

        Ok(comments.into_iter().map(|row| row.into()).collect())
    }

    pub async fn get_comment(
        &self,
        comment_id: i32,
        current_user_id: Option<i32>,
    ) -> Result<CommentDto> {
        let comment = query_file_as!(
            CommentRow,
            "./src/queries/get_comments.sql",
            comment_id,
            None::<String>,
            current_user_id
        )
        .fetch_one(self.db)
        .await?;

        Ok(comment.into())
    }

    pub async fn create_comment(
        &self,
        params: &CreateCommentParams,
        slug: &str,
        current_user_id: i32,
    ) -> Result<CommentDto> {
        let comment = query_file!(
            "./src/queries/create_comment.sql",
            slug,
            params.body,
            current_user_id
        )
        .fetch_one(self.db)
        .await?;

        self.get_comment(comment.id, Some(current_user_id)).await
    }

    pub async fn delete_comment(&self, comment_id: i32, current_user_id: i32) -> Result<()> {
        query_file!(
            "./src/queries/delete_comment.sql",
            comment_id,
            current_user_id
        )
        .execute(self.db)
        .await?;

        Ok(())
    }
}

struct CommentRow {
    id: i32,
    body: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    author_username: String,
    author_bio: Option<String>,
    author_image: Option<String>,
    author_following: bool,
    #[allow(dead_code)]
    article_id: i32,
    #[allow(dead_code)]
    author_id: i32,
}

impl From<CommentRow> for CommentDto {
    fn from(comment: CommentRow) -> Self {
        CommentDto {
            id: comment.id,
            body: comment.body,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            author: ProfileDto {
                username: comment.author_username,
                bio: comment.author_bio,
                image: comment.author_image,
                following: comment.author_following,
            },
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommentDto {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub body: String,
    pub author: ProfileDto,
}
