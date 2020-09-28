use crate::{error::Result, profile::Profile, Context, OperationResult};
use chrono::{DateTime, Utc};
use conduit::{chrono, CommentDto, CommentService, CreateCommentParams};
use juniper::{graphql_object, GraphQLInputObject};

pub mod query {
    use super::*;

    pub async fn get_comments(ctx: &Context, slug: String) -> Result<CommentConnection> {
        Ok(CommentService::new(&ctx.get_pool().await)
            .get_comments(&slug, ctx.get_user_id())
            .await?
            .into())
    }
}

pub mod mutation {
    use super::*;

    #[derive(GraphQLInputObject)]
    pub struct CreateCommentInput {
        body: String,
    }
    impl From<CreateCommentInput> for CreateCommentParams {
        fn from(input: CreateCommentInput) -> Self {
            CreateCommentParams { body: input.body }
        }
    }

    pub async fn create_comment(
        ctx: &Context,
        slug: String,
        input: CreateCommentInput,
    ) -> Result<Comment> {
        Ok(CommentService::new(&ctx.get_pool().await)
            .create_comment(&input.into(), &slug, ctx.get_user()?.id)
            .await?
            .into())
    }

    pub async fn delete_comment(ctx: &Context, comment_id: i32) -> Result<OperationResult> {
        Ok(CommentService::new(&ctx.get_pool().await)
            .delete_comment(comment_id, ctx.get_user()?.id)
            .await?
            .into())
    }
}

pub struct Comment {
    id: i32,
    body: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    author: Profile,
}
impl From<CommentDto> for Comment {
    fn from(dto: CommentDto) -> Self {
        Comment {
            id: dto.id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            body: dto.body,
            author: dto.author.into(),
        }
    }
}

#[graphql_object(Context =Context)]
impl Comment {
    fn id(&self) -> i32 {
        self.id
    }
    fn body(&self) -> &str {
        &self.body
    }
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    fn author(&self) -> &Profile {
        &self.author
    }
}

pub struct CommentConnection {
    nodes: Vec<Comment>,
}
impl From<Vec<CommentDto>> for CommentConnection {
    fn from(comments: Vec<CommentDto>) -> Self {
        CommentConnection {
            nodes: comments.into_iter().map(Comment::from).collect(),
        }
    }
}

#[graphql_object(Context = Context)]
impl CommentConnection {
    fn nodes(&self) -> &[Comment] {
        &self.nodes
    }
    fn total_count(&self) -> i32 {
        self.nodes.len() as i32
    }
}
