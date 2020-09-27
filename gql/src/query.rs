use crate::{article, comment, profile, tag, user, Context};
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    async fn get_user(ctx: &Context) -> FieldResult<user::User> {
        user::query::get_user(ctx).await
    }

    async fn get_article(ctx: &Context, slug: String) -> FieldResult<article::Article> {
        article::query::get_article(ctx, slug).await
    }

    async fn get_articles(
        ctx: &Context,
        input: Option<article::query::GetArticlesInput>,
    ) -> FieldResult<article::ArticleConnection> {
        article::query::get_articles(ctx, input).await
    }

    async fn feed(
        ctx: &Context,
        input: Option<article::query::GetArticlesInput>,
    ) -> FieldResult<article::ArticleConnection> {
        article::query::feed(ctx, input).await
    }

    async fn get_tags(ctx: &Context) -> FieldResult<Vec<String>> {
        tag::query::get_tags(ctx).await
    }

    async fn get_comments(ctx: &Context, slug: String) -> FieldResult<comment::CommentConnection> {
        comment::query::get_comments(ctx, slug).await
    }

    async fn get_profile(ctx: &Context, username: String) -> FieldResult<profile::Profile> {
        profile::query::get_profile(ctx, username).await
    }
}
