use crate::{article, comment, error::Result, profile, user, Context, OperationResult};
use juniper::graphql_object;

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    async fn register(
        ctx: &Context,
        input: user::mutation::UserRegisterInput,
    ) -> Result<user::User> {
        user::mutation::register(ctx, input).await
    }

    pub async fn login(ctx: &Context, input: user::mutation::UserLoginInput) -> Result<user::User> {
        user::mutation::login(ctx, input).await
    }

    pub async fn update_user(
        ctx: &Context,
        input: user::mutation::UserUpdateInput,
    ) -> Result<user::User> {
        user::mutation::update_user(ctx, input).await
    }

    pub async fn create_article(
        ctx: &Context,
        input: article::mutation::CreateArticleInput,
    ) -> Result<article::Article> {
        article::mutation::create_article(ctx, input).await
    }

    pub async fn update_article(
        ctx: &Context,
        slug: String,
        input: article::mutation::UpdateArticleInput,
    ) -> Result<article::Article> {
        article::mutation::update_article(ctx, slug, input).await
    }

    pub async fn delete_article(ctx: &Context, slug: String) -> Result<OperationResult> {
        article::mutation::delete_article(ctx, slug).await
    }

    pub async fn favorite_article(ctx: &Context, slug: String) -> Result<article::Article> {
        article::mutation::favorite_article(ctx, slug).await
    }

    pub async fn unfavorite_article(ctx: &Context, slug: String) -> Result<article::Article> {
        article::mutation::unfavorite_article(ctx, slug).await
    }

    pub async fn create_comment(
        ctx: &Context,
        slug: String,
        input: comment::mutation::CreateCommentInput,
    ) -> Result<comment::Comment> {
        comment::mutation::create_comment(ctx, slug, input).await
    }

    pub async fn delete_comment(ctx: &Context, comment_id: i32) -> Result<OperationResult> {
        comment::mutation::delete_comment(ctx, comment_id).await
    }

    pub async fn follow_profile(ctx: &Context, username: String) -> Result<profile::Profile> {
        profile::mutation::follow_profile(ctx, username).await
    }

    pub async fn unfollow_profile(ctx: &Context, username: String) -> Result<profile::Profile> {
        profile::mutation::unfollow_profile(ctx, username).await
    }
}
