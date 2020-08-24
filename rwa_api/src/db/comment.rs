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
pub struct CommentDto<'a> {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub body: &'a str,
    pub author: ProfileDto<'a>,
}

#[derive(Serialize, Debug)]
pub struct CommentResponse<'a> {
    pub comment: CommentDto<'a>,
}

#[derive(Serialize, Debug)]
pub struct CommentsResponse<'a> {
    pub comments: Vec<CommentDto<'a>>,
}
