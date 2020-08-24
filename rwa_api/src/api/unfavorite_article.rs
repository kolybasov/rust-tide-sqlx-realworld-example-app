use crate::db::{ArticleDto, ArticleResponse, ProfileDto, User};
use crate::State;
use sqlx::query;
use tide::{Body, Request, Response, Result, StatusCode};

pub async fn unfavorite_article(req: Request<State>) -> Result {
    let state = req.state();
    let slug: String = req.param("slug")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    let article = query!(
        r#"
            SELECT a.*, 
                   ARRAY_AGG(at.tag_id) "tag_list!: Vec<Option<String>>",
                   COUNT(DISTINCT af.article_id) "favorites_count!",
                   u.username "author_username!",
                   u.image "author_image",
                   u.bio "author_bio",
                   BOOL_OR(uf.follower_id IS NOT NULL) "author_following!"
            FROM articles a
            LEFT JOIN articles_tags at ON at.article_id = a.id
            LEFT JOIN articles_favorites af ON af.article_id = a.id
            INNER JOIN users u 
                LEFT JOIN users_followers uf ON uf.leader_id = u.id AND uf.follower_id = $2
                ON u.id = a.author_id
            WHERE a.slug = $1
            GROUP BY a.id, u.username, u.image, u.bio
        "#,
        slug,
        current_user_id,
    )
    .fetch_one(&state.db_pool)
    .await?;

    query!(
        "DELETE FROM articles_favorites WHERE article_id = $1 AND user_id = $2",
        article.id,
        current_user_id
    )
    .execute(&state.db_pool)
    .await?;

    let body = ArticleResponse {
        article: ArticleDto {
            slug: &article.slug,
            title: &article.title,
            description: &article.description,
            body: &article.body,
            created_at: article.created_at,
            updated_at: article.updated_at,
            tag_list: article.tag_list.into_iter().filter_map(|tag| tag).collect(),
            favorited: false,
            favorites_count: article.favorites_count as usize - 1,
            author: ProfileDto {
                username: &article.author_username,
                bio: article.author_bio.as_deref(),
                image: article.author_image.as_deref(),
                following: article.author_following,
            },
        },
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&body)?)
        .build())
}
