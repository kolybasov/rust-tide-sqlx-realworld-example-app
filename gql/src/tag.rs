use crate::{error::Result, Context};
use conduit::TagService;

pub mod query {
    use super::*;

    pub async fn get_tags(ctx: &Context) -> Result<Vec<String>> {
        Ok(TagService::new(&ctx.get_pool().await).get_tags().await?)
    }
}
