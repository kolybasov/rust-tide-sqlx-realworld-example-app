use super::ProfileDto;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleDto {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tag_list: Vec<String>,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: ProfileDto,
}

#[derive(Serialize, Debug)]
pub struct ArticleResponse {
    pub article: ArticleDto,
}

#[derive(Serialize, Debug)]
pub struct ArticlesResponse {
    pub articles: Vec<ArticleDto>,
}
