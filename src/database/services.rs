use self::prelude::ServerFnError;
use crate::database::structs::{
    Adresse, Angebot, Ansprechpartner, JsonAngebot, JsonOrganisation, Link, Organisation, Sonstiges,
};
use cfg_if::cfg_if;
use leptos::*;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres};
use std::env;
use uuid::Uuid;

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

#[server(GetOffers)]
pub async fn get_offers() -> Result<Vec<JsonAngebot>, ServerFnError> {
    let pool = db().await?;
    let angebote = sqlx::query_as!(
        Angebot,
        r#"
        SELECT
          *
        FROM
          angebot;"#
    )
    .fetch_all(&pool)
    .await?;

    let mut json_angebote: Vec<JsonAngebot> = Vec::new();
    for angebot in angebote {
        let organisation = select_organisation_for_angebot(angebot.organisation_id).await?;
        let adressen = select_adressen_for_angebot(angebot.angebot_id).await?;
        let links = select_links_for_angebot(angebot.angebot_id).await?;
        let apartner = select_apartner_for_angebot(angebot.angebot_id).await?;
        let sonstiges = select_sonstiges_for_angebot(angebot.angebot_id).await?;

        json_angebote.push(JsonAngebot {
            angebot,
            organisation,
            adressen,
            links,
            apartner,
            sonstiges,
        })
    }

    Ok(json_angebote)
}

#[server]
async fn select_organisation_for_angebot(
    organisation_id: Uuid,
) -> Result<JsonOrganisation, ServerFnError> {
    let pool = db().await?;
    let organisation = sqlx::query_as!(
        Organisation,
        r#"
        SELECT
          *
        FROM
          organisation
        WHERE
          organisation_id = $1"#,
        organisation_id
    )
    .fetch_one(&pool)
    .await?;

    let adressen = select_adressen_for_organisation(organisation.organisation_id).await?;
    let links = select_links_for_organisation(organisation.organisation_id).await?;
    let apartner = select_apartner_for_organisation(organisation.organisation_id).await?;
    let angebote = select_angebote_for_organisation(organisation.organisation_id).await?;

    Ok(JsonOrganisation {
        organisation,
        adressen,
        links,
        apartner,
        angebote,
    })
}

#[server]
async fn select_adressen_for_angebot(angebot_id: Uuid) -> Result<Vec<Adresse>, ServerFnError> {
    let pool = db().await?;
    sqlx::query_as!(
        Adresse,
        r#"
        SELECT
          adresse.*
        FROM
          adresse
          JOIN angebot_adresse ON adresse.adresse_id = angebot_adresse.adresse_id
        WHERE
          angebot_id = $1;"#,
        angebot_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}

#[server]
async fn select_links_for_angebot(angebot_id: Uuid) -> Result<Vec<Link>, ServerFnError> {
    let pool = db().await?;
    sqlx::query_as!(
        Link,
        r#"
        SELECT
          link.*
        FROM
          link
          JOIN angebot_link ON link.link_id = angebot_link.link_id
        WHERE
          angebot_id = $1;"#,
        angebot_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}

#[server]
async fn select_apartner_for_angebot(
    angebot_id: Uuid,
) -> Result<Vec<Ansprechpartner>, ServerFnError> {
    let pool = db().await?;
    sqlx::query_as!(
        Ansprechpartner,
        r#"
        SELECT
          ansprechpartner.*
        FROM
          ansprechpartner
          JOIN angebot_apartner ON ansprechpartner.ansprechpartner_id = angebot_apartner.ansprechpartner_id
        WHERE
          angebot_id = $1;"#,
        angebot_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}

#[server]
async fn select_sonstiges_for_angebot(angebot_id: Uuid) -> Result<Vec<Sonstiges>, ServerFnError> {
    let pool = db().await?;
    sqlx::query_as!(
        Sonstiges,
        r#"
        SELECT
          sonstiges.*
        FROM
          sonstiges
          JOIN angebot ON sonstiges.angebot_id = angebot.angebot_id
        WHERE
          angebot.angebot_id = $1;"#,
        angebot_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}

#[server]
async fn select_adressen_for_organisation(
    organisation_id: Uuid,
) -> Result<Vec<Adresse>, ServerFnError> {
    let pool = db().await?;
    sqlx::query_as!(
        Adresse,
        r#"
        SELECT
          adresse.*
        FROM
          adresse
          JOIN organisation_adresse ON adresse.adresse_id = organisation_adresse.adresse_id
        WHERE
          organisation_id = $1;"#,
        organisation_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}

#[server]
async fn select_links_for_organisation(organisation_id: Uuid) -> Result<Vec<Link>, ServerFnError> {
    let pool = db().await?;
    sqlx::query_as!(
        Link,
        r#"
        SELECT
          link.*
        FROM
          link
          JOIN organisation_link ON link.link_id = organisation_link.link_id
        WHERE
          organisation_id = $1;"#,
        organisation_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}

#[server]
async fn select_apartner_for_organisation(
    organisation_id: Uuid,
) -> Result<Vec<Ansprechpartner>, ServerFnError> {
    let pool = db().await?;
    sqlx::query_as!(
        Ansprechpartner,
        r#"
        SELECT
          ansprechpartner.*
        FROM
          ansprechpartner
          JOIN organisation_apartner ON ansprechpartner.ansprechpartner_id = organisation_apartner.ansprechpartner_id
        WHERE
          organisation_id = $1;"#,
        organisation_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}

#[server]
async fn select_angebote_for_organisation(
    organisation_id: Uuid,
) -> Result<Vec<Angebot>, ServerFnError> {
    let pool = db().await?;
    sqlx::query_as!(
        Angebot,
        r#"
        SELECT
          angebot.*
        FROM
          angebot
        join organisation on organisation.organisation_id = angebot.organisation_id
        WHERE
          angebot.organisation_id = $1;"#,
        organisation_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::Response(e.to_string()))
}
