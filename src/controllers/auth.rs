use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::{PgPool};

use crate::{
    error::AppError,
    models,
};

pub async fn register(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    // check if email or password is a blank string
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential);
    }

    // get the user for the email from database
    let user = sqlx::query_as::<_, models::auth::User>(
        "SELECT email, password FROM users where email = $1",
    )
        .bind(&credentials.email)
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;

    if let Some(_) = user {
        //if a user with email already exits send error
        return Err(AppError::UserAlreadyExits);
    }

    let result = sqlx::query("INSERT INTO users (email, password) values ($1, $2)")
        .bind(&credentials.email)
        .bind(credentials.password)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(json!({ "msg": "registered successfully" })))
    }
}

pub async fn delete_user(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    // check if email or password is a blank string
    if credentials.email.is_empty() {
        return Err(AppError::MissingCredential);
    }

    // get the user for the email from database
    let query_str = "SELECT email, password FROM users where email = $1 LIMIT 1";
    let user = sqlx::query_as::<_, models::auth::User>(query_str)
        .bind(&credentials.email)
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

    if let Some(u) = user {
        if credentials.password != u.password {
            return Err(AppError::WrongCredential);
        }
    }


    let result = sqlx::query("DELETE FROM users WHERE email=$1")
        .bind(&credentials.email)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(json!({ "msg": "deleted successfully" })))
    }
}