use super::ProfileDto;
use super::TagService;
use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slug::slugify;
use sqlx::{query_file, query_file_as, Executor, Postgres};

#[derive(Debug, Deserialize, Default)]
pub struct GetArticlesParams {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub feed: Option<bool>,
}

impl GetArticlesParams {
    pub fn limit(mut self, limit: Option<i64>) -> Self {
        self.limit = limit;
        self
    }

    pub fn offset(mut self, offset: Option<i64>) -> Self {
        self.offset = offset;
        self
    }

    pub fn feed(mut self, feed: Option<bool>) -> Self {
        self.feed = feed;
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticleParams {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleParams {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

pub struct ArticleService<E: Executor<'static, Database = Postgres> + Copy> {
    db: E,
}

impl<E> ArticleService<E>
where
    E: Executor<'static, Database = Postgres> + Copy,
{
    pub fn new(executor: E) -> Self {
        ArticleService { db: executor }
    }

    pub async fn get_articles(
        &self,
        current_user_id: Option<i32>,
        options: &GetArticlesParams,
    ) -> Result<Vec<ArticleDto>> {
        let articles = query_file_as!(
            ArticleRow,
            "./src/queries/get_articles.sql",
            current_user_id,
            None::<String>, // slug
            options.tag,
            options.author,
            options.favorited,
            options.feed,
            options.limit.unwrap_or(20),
            options.offset.unwrap_or(0)
        )
        .fetch_all(self.db)
        .await?;

        Ok(articles.into_iter().map(ArticleRow::into).collect())
    }

    pub async fn get_article(
        &self,
        slug: &str,
        current_user_id: Option<i32>,
    ) -> Result<ArticleDto> {
        let article = query_file_as!(
            ArticleRow,
            "./src/queries/get_articles.sql",
            current_user_id,
            slug,
            None::<String>,
            None::<String>,
            None::<String>,
            None::<bool>,
            1,
            0
        )
        .fetch_one(self.db)
        .await?;

        Ok(article.into())
    }

    pub async fn favorite_article(&self, slug: &str, current_user_id: i32) -> Result<ArticleDto> {
        query_file!("./src/queries/favorite_article.sql", slug, current_user_id)
            .execute(self.db)
            .await?;

        self.get_article(slug, Some(current_user_id)).await
    }

    pub async fn unfavorite_article(&self, slug: &str, current_user_id: i32) -> Result<ArticleDto> {
        query_file!(
            "./src/queries/unfavorite_article.sql",
            slug,
            current_user_id
        )
        .execute(self.db)
        .await?;

        self.get_article(slug, Some(current_user_id)).await
    }

    pub async fn create_article(
        &self,
        params: &CreateArticleParams,
        current_user_id: i32,
    ) -> Result<ArticleDto> {
        let slug = slugify(&params.title);
        let article = query_file!(
            "./src/queries/create_article.sql",
            slug,
            params.title,
            params.description,
            params.body,
            current_user_id
        )
        .fetch_one(self.db)
        .await?;

        let tags_count = params
            .tag_list
            .as_ref()
            .map(|tag_list| tag_list.len())
            .unwrap_or(0);

        if tags_count > 0 {
            let tag_service = TagService::new(self.db);

            let tag_list = tag_service
                .create_tags(&params.tag_list.clone().unwrap())
                .await?;
            tag_service
                .assign_tags_to_article(&tag_list, article.id)
                .await?;
        }

        self.get_article(&slug, Some(current_user_id)).await
    }

    pub async fn update_article(
        &self,
        slug: &str,
        current_user_id: i32,
        params: &UpdateArticleParams,
    ) -> Result<ArticleDto> {
        let new_slug = if let Some(new_title) = &params.title {
            Some(slugify(new_title))
        } else {
            None
        };

        query_file!(
            "./src/queries/update_article.sql",
            new_slug,
            params.title,
            params.description,
            params.body,
            slug,
            current_user_id
        )
        .execute(self.db)
        .await?;

        self.get_article(&new_slug.as_deref().unwrap_or(slug), Some(current_user_id))
            .await
    }

    pub async fn delete_article(&self, slug: &str, current_user_id: i32) -> Result<()> {
        query_file!("./src/queries/delete_article.sql", slug, current_user_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
struct ArticleRow {
    // article
    slug: String,
    title: String,
    description: String,
    body: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    // tags
    tag_list: Vec<String>,
    // author
    author_username: String,
    author_image: Option<String>,
    author_bio: Option<String>,
    author_following: bool,
    // favorites
    favorited: Option<bool>,
    favorites_count: i64,
    // unneeded fields
    #[allow(dead_code)]
    id: i32,
    #[allow(dead_code)]
    author_id: i32,
}

impl From<ArticleRow> for ArticleDto {
    fn from(article: ArticleRow) -> ArticleDto {
        ArticleDto {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            created_at: article.created_at,
            updated_at: article.updated_at,
            tag_list: article.tag_list,
            favorited: article.favorited.unwrap_or(false),
            favorites_count: article.favorites_count as usize,
            author: ProfileDto {
                username: article.author_username,
                bio: article.author_bio,
                image: article.author_image,
                following: article.author_following,
            },
        }
    }
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
