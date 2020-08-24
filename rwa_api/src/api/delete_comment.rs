use crate::db::User;
use crate::State;
use sqlx::query;
use tide::{Request, Response, Result, StatusCode};

pub async fn delete_comment(req: Request<State>) -> Result {
    let state = req.state();
    let comment_id: i32 = req.param("id")?;
    let current_user_id = req.ext::<User>().unwrap().id;

    query!(
        "DELETE FROM comments WHERE id = $1 AND author_id = $2",
        comment_id,
        current_user_id
    )
    .execute(&state.db_pool)
    .await?;

    Ok(Response::new(StatusCode::NoContent))
}
