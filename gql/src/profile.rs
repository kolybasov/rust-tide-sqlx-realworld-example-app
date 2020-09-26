use crate::Context;
use conduit::{ProfileDto, ProfileService};
use juniper::{graphql_object, FieldResult};

pub mod query {
    use super::*;

    pub async fn get_profile(ctx: &Context, username: String) -> FieldResult<Profile> {
        Ok(ProfileService::new(&ctx.get_pool().await)
            .get_profile(&username, ctx.get_user_id())
            .await?
            .into())
    }
}

pub mod mutation {
    use super::*;

    pub async fn follow_profile(ctx: &Context, username: String) -> FieldResult<Profile> {
        Ok(ProfileService::new(&ctx.get_pool().await)
            .follow_profile(&username, ctx.get_user()?.id)
            .await?
            .into())
    }

    pub async fn unfollow_profile(ctx: &Context, username: String) -> FieldResult<Profile> {
        Ok(ProfileService::new(&ctx.get_pool().await)
            .unfollow_profile(&username, ctx.get_user()?.id)
            .await?
            .into())
    }
}

pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}
impl From<ProfileDto> for Profile {
    fn from(dto: ProfileDto) -> Profile {
        Profile {
            username: dto.username,
            bio: dto.bio,
            image: dto.image,
            following: dto.following,
        }
    }
}

#[graphql_object(Context = Context)]
impl Profile {
    fn username(&self) -> &str {
        &self.username
    }
    fn bio(&self) -> Option<&str> {
        self.bio.as_deref()
    }
    fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }
    fn following(&self) -> bool {
        self.following
    }
}
