use self::prelude::ServerFnError;
use cfg_if::cfg_if;
use leptos::*;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres};
use std::env;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub async fn db() -> Result<Pool<Postgres>, ServerFnError> {
            let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL should be set");

            sqlx::postgres::PgPool::connect(&database_url)
                .await.map_err(|e| ServerFnError::ServerError(e.to_string()))
        }

        pub async fn migrations() -> Result<(), ServerFnError> {
            let pool = db().await.unwrap();

            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))
        }
    }
}
