use crate::user;
use crate::Context;
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    async fn get_user(ctx: &Context) -> FieldResult<Option<user::User>> {
        user::queries::get_user(ctx).await
    }
}
