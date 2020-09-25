use crate::{article, user, Context};
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
}
