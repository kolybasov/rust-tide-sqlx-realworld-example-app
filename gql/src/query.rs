use crate::{article, comment, error::Result, profile, tag, user, Context};
use juniper::graphql_object;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    async fn get_user(ctx: &Context) -> Result<user::User> {
        user::query::get_user(ctx).await
    }

    async fn get_article(ctx: &Context, slug: String) -> Result<article::Article> {
        article::query::get_article(ctx, slug).await
    }

    async fn get_articles(
        ctx: &Context,
        input: Option<article::query::GetArticlesInput>,
    ) -> Result<article::ArticleConnection> {
        article::query::get_articles(ctx, input).await
    }

    async fn feed(
        ctx: &Context,
        input: Option<article::query::GetArticlesInput>,
    ) -> Result<article::ArticleConnection> {
        article::query::feed(ctx, input).await
    }

    async fn get_tags(ctx: &Context) -> Result<Vec<String>> {
        tag::query::get_tags(ctx).await
    }

    async fn get_comments(ctx: &Context, slug: String) -> Result<comment::CommentConnection> {
        comment::query::get_comments(ctx, slug).await
    }

    async fn get_profile(ctx: &Context, username: String) -> Result<profile::Profile> {
        profile::query::get_profile(ctx, username).await
    }
}
