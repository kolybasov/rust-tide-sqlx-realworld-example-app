use serde::Deserialize;

pub mod article;
pub mod comment;
pub mod profile;
pub mod tag;
pub mod user;

pub use article::{
    ArticleDto, ArticleResponse, ArticleService, ArticlesResponse, CreateArticleParams,
    GetArticlesParams, UpdateArticleParams,
};
pub use comment::{
    CommentDto, CommentResponse, CommentService, CommentsResponse, CreateCommentParams,
};
pub use profile::{ProfileDto, ProfileResponse, ProfileService};
pub use tag::TagService;
pub use user::{
    LoginParams, RegisterParams, UpdateUserParams, User, UserDto, UserResponse, UserService,
};

#[derive(Debug, Deserialize)]
pub struct PageOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
