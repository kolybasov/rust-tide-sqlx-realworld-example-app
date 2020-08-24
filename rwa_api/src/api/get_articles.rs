use crate::db::{ArticleDto, ArticlesResponse, ProfileDto, User};
use crate::State;
use serde::Deserialize;
use sqlx::query;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Debug, Deserialize)]
pub struct GetArticlesParams {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn get_articles(req: Request<State>) -> Result {
    let state = req.state();
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);
    let params: GetArticlesParams = req.query()?;

    let articles = query!(
        r#"
            SELECT a.*, 
                   ARRAY_AGG(at.tag_id) "tag_list!: Vec<Option<String>>",
                   COUNT(DISTINCT af.article_id) "favorites_count!",
                   BOOL_OR(af2.user_id IS NOT NULL) "favorited!",
                   u.username "author_username",
                   u.bio "author_bio",
                   u.image "author_image",
                   BOOL_OR(uf.follower_id IS NOT NULL) "author_following!"
            FROM articles a
            LEFT JOIN articles_tags at ON at.article_id = a.id
            LEFT JOIN articles_favorites af 
                INNER JOIN users u2 ON u2.id = af.user_id
                ON af.article_id = a.id
            LEFT JOIN articles_favorites af2 ON af2.article_id = a.id AND af2.user_id = $1
            INNER JOIN users u 
                LEFT JOIN users_followers uf ON uf.leader_id = u.id AND uf.follower_id = $1
                ON u.id = a.author_id
            WHERE ($2::VARCHAR IS NULL OR at.tag_id = $2::VARCHAR) AND
                  ($3::VARCHAR IS NULL OR u.username = $3::VARCHAR) AND
                  ($4::VARCHAR IS NULL OR u2.username = $4::VARCHAR)
            GROUP BY a.id, u.username, u.bio, u.image
            ORDER BY a.id DESC
            LIMIT $6
            OFFSET $5
        "#,
        current_user_id,
        params.tag,
        params.author,
        params.favorited,
        params.offset.unwrap_or(0),
        params.limit.unwrap_or(20),
    )
    .fetch_all(&state.db_pool)
    .await?;

    let articles_dtos: Vec<ArticleDto> = articles
        .iter()
        .map(|article| ArticleDto {
            slug: &article.slug,
            title: &article.title,
            description: &article.description,
            body: &article.body,
            created_at: article.created_at,
            updated_at: article.updated_at,
            tag_list: article.tag_list.iter().filter_map(|tag| tag.as_deref()).collect(),
            favorited: article.favorited,
            favorites_count: article.favorites_count as usize,
            author: ProfileDto {
                username: &article.author_username,
                bio: article.author_bio.as_deref(),
                image: article.author_image.as_deref(),
                following: article.author_following,
            },
        })
        .collect();
    let articles_count = articles_dtos.len();
    let body = ArticlesResponse {
        articles: articles_dtos,
        articles_count,
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&body)?)
        .build())
}
