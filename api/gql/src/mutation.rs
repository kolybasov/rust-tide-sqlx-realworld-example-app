use crate::user;
use crate::Context;
use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    async fn register(
        ctx: &Context,
        input: user::mutations::UserRegisterInput,
    ) -> FieldResult<user::User> {
        user::mutations::register(ctx, input).await
    }

    pub async fn login(
        ctx: &Context,
        input: user::mutations::UserLoginInput,
    ) -> FieldResult<user::User> {
        user::mutations::login(ctx, input).await
    }

    pub async fn update_user(
        ctx: &Context,
        input: user::mutations::UserUpdateInput,
    ) -> FieldResult<user::User> {
        user::mutations::update_user(ctx, input).await
    }
}
