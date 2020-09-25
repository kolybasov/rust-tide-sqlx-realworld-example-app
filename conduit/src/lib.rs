pub mod article;
pub mod comment;
pub mod error;
pub mod profile;
pub mod query;
pub mod tag;
pub mod user;

pub use article::{
    ArticleDto, ArticleService, CreateArticleParams, GetArticlesParams, UpdateArticleParams,
};
pub use chrono;
pub use comment::{CommentDto, CommentService, CreateCommentParams};
pub use profile::{ProfileDto, ProfileService};
use serde::Deserialize;
pub use sqlx::{postgres::PgPoolOptions, PgPool};
pub use tag::TagService;
pub use user::{LoginParams, RegisterParams, UpdateUserParams, User, UserDto, UserService};

#[derive(Debug, Deserialize)]
pub struct PageOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
