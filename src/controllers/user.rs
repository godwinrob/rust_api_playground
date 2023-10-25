use crate::{error::AppError, models, models::auth::Claims};
use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

pub async fn user_profile(claims: Claims) -> Result<axum::Json<serde_json::Value>, AppError> {
    Ok(axum::Json(serde_json::json!({"email": claims.email})))
}

pub async fn delete_user(
    claims: Claims,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    // check if email or password is a blank string
    if claims.email.is_empty() {
        return Err(AppError::InvalidToken);
    }

    // get the user for the email from database
    let query_str = "SELECT email, password FROM users where email = $1 LIMIT 1";
    let user = sqlx::query_as::<_, models::auth::User>(query_str)
        .bind(&claims.email)
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;

    if let None = user {
        //if a user with email does not exist return error
        return Err(AppError::UserDoesNotExist);
    }

    let mut email_for_deletion = "".to_string();

    if let Some(u) = user {
        if u.email != "" {
            email_for_deletion = u.email
        }
    }

    let result = sqlx::query("DELETE FROM users WHERE email=$1")
        .bind(&email_for_deletion)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(json!({ "msg": "deleted successfully" })))
    }
}

pub async fn update_user(
    Json(user_to_update): Json<models::auth::User>,
    claims: Claims,
    Extension(pool): Extension<PgPool>,
) -> Result<axum::Json<serde_json::Value>, AppError> {
    // check if email or password is a blank string
    if claims.email.is_empty() {
        return Err(AppError::InvalidToken);
    }

    let mut email_for_update = "".to_string();

    // get the user for the email from database
    let query_str = "SELECT email FROM users where email = $1 LIMIT 1";
    let user = sqlx::query(query_str)
        .bind(&claims.email)
        .map(|row: PgRow| models::auth::User {
            email: row.get("email"),
            password: "".to_string(),
        })
        .fetch_optional(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    if let None = user {
        //if a user with email does not exist return error
        return Err(AppError::UserDoesNotExist);
    }

    if let Some(u) = user {
        if u.email != "" {
            email_for_update = u.email
        }
    }

    println!("email from db: {}", email_for_update);
    println!("new email: {}", user_to_update.email);

    let result = sqlx::query("UPDATE users SET email = $1 WHERE email = $2")
        .bind(&user_to_update.email)
        .bind(&email_for_update)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(json!({ "msg": "user has been updated!" })))
    }
}
