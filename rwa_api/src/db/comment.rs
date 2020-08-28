use crate::db::ProfileDto;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub author_id: i32,
    pub article_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

#[derive(Serialize, Debug)]
pub struct CommentResponse {
    pub comment: CommentDto,
}

impl From<CommentDto> for CommentResponse {
    fn from(comment: CommentDto) -> Self {
        CommentResponse { comment }
    }
}

#[derive(Serialize, Debug)]
pub struct CommentsResponse {
    pub comments: Vec<CommentDto>,
}

impl From<Vec<CommentDto>> for CommentsResponse {
    fn from(comments: Vec<CommentDto>) -> Self {
        CommentsResponse { comments }
    }
}
