use crate::user;
use crate::Context;
use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    async fn register(
        ctx: &Context,
        input: user::mutation::UserRegisterInput,
    ) -> FieldResult<user::User> {
        user::mutation::register(ctx, input).await
    }

    pub async fn login(
        ctx: &Context,
        input: user::mutation::UserLoginInput,
    ) -> FieldResult<user::User> {
        user::mutation::login(ctx, input).await
    }

    pub async fn update_user(
        ctx: &Context,
        input: user::mutation::UserUpdateInput,
    ) -> FieldResult<user::User> {
        user::mutation::update_user(ctx, input).await
    }
}
