use self::prelude::ServerFnError;
use crate::database::{services::database, structs::Email};
use leptos::*;
use std::env;

#[server(GetMails, "/api")]
pub async fn get_mails() -> Result<Vec<Email>, ServerFnError> {
    let pool = database::db().await?;

    sqlx::query_as!(
        Email,
        r#"
        SELECT
          *
        FROM
          email;"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}
