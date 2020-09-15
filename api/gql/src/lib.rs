use conduit::{PgPool, ProfileService};
use juniper::{EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

#[derive(Clone)]
pub struct Context {
    pub db_pool: PgPool,
}

impl juniper::Context for Context {}

pub struct Query;
#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn get_profile(context: &Context, username: String) -> FieldResult<Profile> {
        let profile = ProfileService::new(&context.db_pool)
            .get_profile(&username, None)
            .await?;

        Ok(Profile {
            username: profile.username,
            bio: profile.bio,
            image: profile.image,
            following: profile.following,
        })
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}
