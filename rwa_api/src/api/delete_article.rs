use crate::db::User;
use crate::State;
use sqlx::query;
use tide::{Request, Response, Result, StatusCode};

pub async fn delete_article(req: Request<State>) -> Result {
    let state = req.state();
    let current_user_id = req.ext::<User>().unwrap().id;
    let slug: String = req.param("slug")?;

    query!(
        "DELETE FROM articles WHERE slug = $1 AND author_id = $2",
        slug,
        current_user_id
    )
    .execute(&state.db_pool)
    .await?;

    Ok(Response::new(StatusCode::NoContent))
}
