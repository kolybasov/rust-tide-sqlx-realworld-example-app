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
pub struct ArticleDto<'a> {
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tag_list: Vec<String>,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: ProfileDto<'a>,
}

#[derive(Serialize, Debug)]
pub struct ArticleResponse<'a> {
    pub article: ArticleDto<'a>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesResponse<'a> {
    pub articles: Vec<ArticleDto<'a>>,
    pub articles_count: usize,
}
