use crate::db::{Article, ArticleDto, ArticleResponse, ProfileDto, User};
use crate::State;
use serde::Deserialize;
use slug::slugify;
use sqlx::{query, query_as, Acquire};
use tide::{Body, Request, Response, Result, StatusCode};

#[derive(Debug, Deserialize)]
struct CreateArticlePayload {
    article: CreateArticlePayloadArticle,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateArticlePayloadArticle {
    title: String,
    description: String,
    body: String,
    tag_list: Option<Vec<String>>,
}

pub async fn create_article(mut req: Request<State>) -> Result {
    let payload: CreateArticlePayload = req.body_json().await?;
    let state = req.state();
    let author = req.ext::<User>().unwrap();

    let mut transaction = state.db_pool.begin().await?;

    let article = query_as!(
        Article,
        r#"
            INSERT INTO articles (slug, title, description, body, author_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        "#,
        slugify(&payload.article.title),
        payload.article.title,
        payload.article.description,
        payload.article.body,
        author.id
    )
    .fetch_one(transaction.acquire().await?)
    .await?;

    let tags_count = payload
        .article
        .tag_list
        .as_ref()
        .map(|tag_list| tag_list.len())
        .unwrap_or(0);

    let tags_list = if tags_count > 0 {
        let tags_list: Vec<String> = payload
            .article
            .tag_list
            .unwrap()
            .iter()
            .map(|raw_tag| slugify(raw_tag))
            .collect();

        // Create missing tags
        let placeholders = (1..=tags_count)
            .map(|index| format!("(${})", index))
            .collect::<Vec<String>>()
            .join(",");
        let insert_tags_query = format!(
            "INSERT INTO tags (tag) VALUES {} ON CONFLICT (tag) DO NOTHING",
            placeholders
        );

        tags_list
            .iter()
            .fold(query(&insert_tags_query), |query, tag| query.bind(tag))
            .execute(transaction.acquire().await?)
            .await?;

        // Assign tags to articles
        let placeholders = (1..=(tags_count * 2))
            .step_by(2)
            .map(|index| format!("(${}, ${})", index, index + 1))
            .collect::<Vec<String>>()
            .join(",");

        let insert_articles_tags_query = format!(
            "INSERT INTO articles_tags (tag_id, article_id) VALUES {}",
            placeholders
        );

        tags_list
            .iter()
            .fold(query(&insert_articles_tags_query), |query, tag| {
                query.bind(tag).bind(article.id)
            })
            .execute(transaction.acquire().await?)
            .await?;

        tags_list
    } else {
        vec![]
    };

    transaction.commit().await?;

    let body = ArticleResponse {
        article: ArticleDto {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            created_at: article.created_at,
            updated_at: article.updated_at,
            favorited: false,
            favorites_count: 0,
            tag_list: tags_list,
            author: ProfileDto {
                username: author.username.clone(),
                bio: author.bio.clone(),
                image: author.image.clone(),
                following: false,
            },
        },
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&body)?)
        .build())
}
