use crate::db::{ArticleDto, ArticleResponse, ProfileDto, User};
use crate::State;
use sqlx::{query, query_as};
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn get_article(req: Request<State>) -> Result {
    let state = req.state();
    let current_user_id = req.ext::<User>().map(|user| user.id).or(None);
    let slug: String = req.param("slug")?;

    let article = query!(
        r#"
            SELECT a.*, 
                   ARRAY_AGG(at.tag_id) "tag_list!",
                   COUNT(DISTINCT af.article_id) "favorites_count!",
                   af2.user_id IS NOT NULL "favorited!"
            FROM articles a
            LEFT JOIN articles_tags at ON at.article_id = a.id
            LEFT JOIN articles_favorites af ON af.article_id = a.id
            LEFT JOIN articles_favorites af2 ON af2.article_id = a.id AND af2.user_id = $2
            WHERE a.slug = $1
            GROUP BY a.id, af2.user_id
        "#,
        slug,
        current_user_id,
    )
    .fetch_one(&state.db_pool)
    .await?;

    let author = query_as!(
        ProfileDto,
        r#"
            SELECT username, bio, image, (uf.following_id IS NOT NULL) "following!"  FROM users u
            LEFT JOIN users_followers uf ON uf.following_id = u.id AND uf.follower_id = $2
            WHERE u.id = $1
        "#,
        article.author_id,
        current_user_id
    )
    .fetch_one(&state.db_pool)
    .await?;

    let body = ArticleResponse {
        article: ArticleDto {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            created_at: article.created_at,
            updated_at: article.updated_at,
            tag_list: article.tag_list,
            favorited: article.favorited,
            favorites_count: article.favorites_count as usize,
            author,
        },
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&body)?)
        .build())
}
