use crate::Context;
use conduit::ProfileService;
use juniper::{graphql_object, FieldResult, GraphQLObject};

#[derive(GraphQLObject)]
struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

pub struct Query;
#[graphql_object(Context = Context)]
impl Query {
    async fn get_profile(context: &Context, username: String) -> FieldResult<Profile> {
        let profile = ProfileService::new(&context.get_pool().await)
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
