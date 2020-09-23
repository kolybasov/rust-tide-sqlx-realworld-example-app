use crate::Context;
use conduit::{UserDto, UserService};
use juniper::{graphql_object, FieldResult};

pub mod queries {
    use super::*;

    pub async fn get_user(ctx: &Context) -> FieldResult<Option<User>> {
        let user = if let Some(user) = ctx.user.clone() {
            let state = ctx.state.read().await;
            let token = state.jwt.sign(&user)?;

            Some(User {
                email: user.email,
                username: user.username,
                token,
                bio: user.bio,
                image: user.image,
            })
        } else {
            None
        };

        Ok(user)
    }
}

pub mod mutations {
    use super::*;
    use conduit::{LoginParams, RegisterParams, UpdateUserParams};
    use juniper::GraphQLInputObject;

    #[derive(GraphQLInputObject)]
    pub struct UserRegisterInput {
        email: String,
        password: String,
        username: String,
    }
    impl From<UserRegisterInput> for RegisterParams {
        fn from(input: UserRegisterInput) -> Self {
            RegisterParams {
                email: input.email,
                password: input.password,
                username: input.username,
            }
        }
    }

    pub async fn register(ctx: &Context, input: UserRegisterInput) -> FieldResult<User> {
        let state = ctx.state.read().await;
        Ok(UserService::new(&state.db_pool)
            .register(&input.into(), |user| state.jwt.sign(user))
            .await?
            .into())
    }

    #[derive(GraphQLInputObject)]
    pub struct UserLoginInput {
        email: String,
        password: String,
    }
    impl From<UserLoginInput> for LoginParams {
        fn from(input: UserLoginInput) -> Self {
            LoginParams {
                email: input.email,
                password: input.password,
            }
        }
    }

    pub async fn login(ctx: &Context, input: UserLoginInput) -> FieldResult<User> {
        let state = ctx.state.read().await;
        Ok(UserService::new(&state.db_pool)
            .login(&input.into(), |user| state.jwt.sign(user))
            .await?
            .into())
    }

    #[derive(GraphQLInputObject)]
    pub struct UserUpdateInput {
        email: Option<String>,
        password: Option<String>,
        username: Option<String>,
        bio: Option<String>,
        image: Option<String>,
    }
    impl From<UserUpdateInput> for UpdateUserParams {
        fn from(input: UserUpdateInput) -> Self {
            UpdateUserParams {
                email: input.email,
                username: input.username,
                password: input.password,
                bio: input.bio,
                image: input.image,
            }
        }
    }

    pub async fn update_user(ctx: &Context, input: UserUpdateInput) -> FieldResult<User> {
        let state = ctx.state.read().await;
        Ok(UserService::new(&state.db_pool)
            .update_user(&input.into(), ctx.get_user()?, |user| state.jwt.sign(user))
            .await?
            .into())
    }
}

pub struct User {
    email: String,
    token: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

impl From<UserDto> for User {
    fn from(dto: UserDto) -> Self {
        User {
            email: dto.email,
            token: dto.token,
            username: dto.username,
            bio: dto.bio,
            image: dto.image,
        }
    }
}

#[graphql_object(Context = Context)]
impl User {
    fn email(&self) -> &str {
        &self.email
    }

    fn token(&self) -> &str {
        &self.token
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn bio(&self) -> Option<&str> {
        self.bio.as_deref()
    }

    fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }
}
