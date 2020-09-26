use crate::Context;
use conduit::TagService;
use juniper::FieldResult;

pub mod query {
    use super::*;

    pub async fn get_tags(ctx: &Context) -> FieldResult<Vec<String>> {
        Ok(TagService::new(&ctx.get_pool().await).get_tags().await?)
    }
}
