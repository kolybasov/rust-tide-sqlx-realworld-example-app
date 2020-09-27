use crate::{profile::Profile, Context, OperationResult};
use chrono::{DateTime, Utc};
use conduit::{
    chrono, ArticleDto, ArticleService, CreateArticleParams, GetArticlesParams, UpdateArticleParams,
};
use juniper::{graphql_object, FieldResult, GraphQLInputObject};

pub mod query {
    use super::*;

    pub async fn get_article(ctx: &Context, slug: String) -> FieldResult<Article> {
        Ok(ArticleService::new(&ctx.get_pool().await)
            .get_article(&slug, ctx.get_user_id())
            .await?
            .into())
    }

    #[derive(GraphQLInputObject, Default)]
    pub struct GetArticlesInput {
        tag: Option<String>,
        author: Option<String>,
        favorited: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    }
    impl From<GetArticlesInput> for GetArticlesParams {
        fn from(input: GetArticlesInput) -> Self {
            GetArticlesParams {
                tag: input.tag,
                author: input.author,
                favorited: input.favorited,
                limit: input.limit.map(|limit| limit as i64),
                offset: input.offset.map(|offset| offset as i64),
                feed: None,
            }
        }
    }

    pub async fn get_articles(
        ctx: &Context,
        input: Option<GetArticlesInput>,
    ) -> FieldResult<ArticleConnection> {
        let params: GetArticlesParams = input.unwrap_or_else(|| GetArticlesInput::default()).into();
        Ok(ArticleService::new(&ctx.get_pool().await)
            .get_articles(ctx.get_user_id(), &params)
            .await?
            .into())
    }

    pub async fn feed(
        ctx: &Context,
        input: Option<GetArticlesInput>,
    ) -> FieldResult<ArticleConnection> {
        let user = ctx.get_user()?;
        let params = GetArticlesParams::from(input.unwrap_or_else(|| GetArticlesInput::default()))
            .feed(Some(true));

        Ok(ArticleService::new(&ctx.get_pool().await)
            .get_articles(Some(user.id), &params)
            .await?
            .into())
    }
}

pub mod mutation {
    use super::*;

    #[derive(GraphQLInputObject)]
    pub struct CreateArticleInput {
        title: String,
        description: String,
        body: String,
        tag_list: Option<Vec<String>>,
    }
    impl From<CreateArticleInput> for CreateArticleParams {
        fn from(input: CreateArticleInput) -> Self {
            CreateArticleParams {
                title: input.title,
                description: input.description,
                body: input.body,
                tag_list: input.tag_list,
            }
        }
    }

    pub async fn create_article(ctx: &Context, input: CreateArticleInput) -> FieldResult<Article> {
        let user = ctx.get_user()?;
        Ok(ArticleService::new(&ctx.get_pool().await)
            .create_article(&input.into(), user.id)
            .await?
            .into())
    }

    #[derive(GraphQLInputObject)]
    pub struct UpdateArticleInput {
        title: Option<String>,
        description: Option<String>,
        body: Option<String>,
    }
    impl From<UpdateArticleInput> for UpdateArticleParams {
        fn from(input: UpdateArticleInput) -> Self {
            UpdateArticleParams {
                title: input.title,
                description: input.description,
                body: input.body,
            }
        }
    }

    pub async fn update_article(
        ctx: &Context,
        slug: String,
        input: UpdateArticleInput,
    ) -> FieldResult<Article> {
        let user = ctx.get_user()?;
        Ok(ArticleService::new(&ctx.get_pool().await)
            .update_article(&slug, user.id, &input.into())
            .await?
            .into())
    }

    pub async fn delete_article(ctx: &Context, slug: String) -> FieldResult<OperationResult> {
        Ok(ArticleService::new(&ctx.get_pool().await)
            .delete_article(&slug, ctx.get_user()?.id)
            .await?
            .into())
    }

    pub async fn favorite_article(ctx: &Context, slug: String) -> FieldResult<Article> {
        Ok(ArticleService::new(&ctx.get_pool().await)
            .favorite_article(&slug, ctx.get_user()?.id)
            .await?
            .into())
    }

    pub async fn unfavorite_article(ctx: &Context, slug: String) -> FieldResult<Article> {
        Ok(ArticleService::new(&ctx.get_pool().await)
            .unfavorite_article(&slug, ctx.get_user()?.id)
            .await?
            .into())
    }
}

pub struct Article {
    slug: String,
    title: String,
    description: String,
    body: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    tag_list: Vec<String>,
    favorited: bool,
    favorites_count: usize,
    author: Profile,
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
            tag_list: dto.tag_list,
            favorited: dto.favorited,
            favorites_count: dto.favorites_count,
            author: dto.author.into(),
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
    fn tag_list(&self) -> &[String] {
        &self.tag_list
    }
    fn favorited(&self) -> bool {
        self.favorited
    }
    fn favorites_count(&self) -> i32 {
        self.favorites_count as i32
    }
    fn author(&self) -> &Profile {
        &self.author
    }
}

pub struct ArticleConnection {
    nodes: Vec<Article>,
}
impl From<Vec<ArticleDto>> for ArticleConnection {
    fn from(articles: Vec<ArticleDto>) -> Self {
        ArticleConnection {
            nodes: articles.into_iter().map(Article::from).collect(),
        }
    }
}

#[graphql_object(Context = Context)]
impl ArticleConnection {
    fn nodes(&self) -> &[Article] {
        &self.nodes
    }
    fn total_count(&self) -> i32 {
        self.nodes.len() as i32
    }
}
