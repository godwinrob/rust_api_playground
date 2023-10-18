use serde::{Deserialize, Serialize};
#[derive(Deserialize, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub password: String,
}
