use crate::db::{ArticleDto, ArticleResponse, ProfileDto, User};
use crate::State;
use serde::Deserialize;
use slug::slugify;
use sqlx::query;
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Debug, Deserialize)]
struct UpdateArticlePayload {
    article: UpdateArticlePayloadArticle,
}

#[derive(Debug, Deserialize)]
struct UpdateArticlePayloadArticle {
    title: Option<String>,
    description: Option<String>,
    body: Option<String>,
}

pub async fn update_article(mut req: Request<State>) -> Result {
    let payload: UpdateArticlePayload = req.body_json().await?;
    let state = req.state();
    let slug: String = req.param("slug")?;
    let author_id = req.ext::<User>().unwrap().id;

    let new_slug = if let Some(new_title) = &payload.article.title {
        Some(slugify(new_title))
    } else {
        None
    };

    let updated_article = query!(
        r#"
            UPDATE articles a
            SET slug = COALESCE($1, a.slug),
                title = COALESCE($2, a.title),
                description = COALESCE($3, a.description),
                body = COALESCE($4, a.body)
            WHERE slug = $5 AND author_id = $6
            RETURNING id
        "#,
        new_slug,
        payload.article.title,
        payload.article.description,
        payload.article.body,
        slug,
        author_id
    )
    .fetch_one(&state.db_pool)
    .await?;

    let article = query!(
        r#"
            SELECT a.*, 
                   ARRAY_AGG(at.tag_id) "tag_list!: Vec<Option<String>>",
                   COUNT(DISTINCT af.article_id) "favorites_count!",
                   BOOL_OR(af2.user_id IS NOT NULL) "favorited!",
                   u.username "author_username!",
                   u.image "author_image",
                   u.bio "author_bio",
                   BOOL_OR(uf.follower_id IS NOT NULL) "author_following!"
            FROM articles a
            LEFT JOIN articles_tags at ON at.article_id = a.id
            LEFT JOIN articles_favorites af ON af.article_id = a.id
            LEFT JOIN articles_favorites af2 ON af2.article_id = a.id AND af2.user_id = $2
            INNER JOIN users u 
                LEFT JOIN users_followers uf ON uf.leader_id = u.id AND uf.follower_id = $2
                ON u.id = a.author_id
            WHERE a.id = $1
            GROUP BY a.id, u.username, u.image, u.bio
        "#,
        updated_article.id,
        author_id,
    )
    .fetch_one(&state.db_pool)
    .await?;

    let body = ArticleResponse {
        article: ArticleDto {
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
        },
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&body)?)
        .build())
}
