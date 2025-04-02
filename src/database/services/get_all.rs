use self::prelude::ServerFnError;
use crate::database::{
    services::database,
    structs::{Angebot, Email},
};
use leptos::*;
use std::env;

#[server(GetAngebote, "/api")]
pub async fn get_angebote() -> Result<Vec<Angebot>, ServerFnError> {
    let pool = database::db().await?;

    sqlx::query_as!(
        Angebot,
        r#"
        SELECT
          *
        FROM
          angebot;"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}

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
