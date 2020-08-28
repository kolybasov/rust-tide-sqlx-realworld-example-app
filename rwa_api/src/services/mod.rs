use serde::Deserialize;

pub mod article;
pub mod comment;
pub mod profile;
pub mod tag;
pub mod user;

pub use article::{ArticleService, CreateArticleParams, GetArticlesParams, UpdateArticleParams};
pub use comment::{CommentService, CreateCommentParams};
pub use profile::ProfileService;
pub use tag::TagService;
pub use user::{LoginParams, RegisterParams, UpdateUserParams, UserService};

#[derive(Debug, Deserialize)]
pub struct PageOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
