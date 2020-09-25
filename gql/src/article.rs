use crate::Context;
use chrono::{DateTime, Utc};
use conduit::{chrono, ArticleDto, ArticleService};
use juniper::{graphql_object, FieldResult};

pub mod query {
    use super::*;

    pub async fn get_article(ctx: &Context, slug: String) -> FieldResult<Article> {
        let current_user_id = ctx.user.as_ref().map(|user| user.id);
        Ok(ArticleService::new(&ctx.get_pool().await)
            .get_article(&slug, current_user_id)
            .await?
            .into())
    }
}

pub mod mutation {
    use super::*;
}

pub struct Article {
    slug: String,
    title: String,
    description: String,
    body: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ArticleDto> for Article {
    fn from(dto: ArticleDto) -> Self {
        Article {
            slug: dto.slug,
            title: dto.title,
            description: dto.description,
            body: dto.body,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[graphql_object(Context = Context)]
impl Article {
    fn slug(&self) -> &str {
        &self.slug
    }
    fn title(&self) -> &str {
        &self.title
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn body(&self) -> &str {
        &self.body
    }
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
