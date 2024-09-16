#[cfg(not(feature = "ssr"))]
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::types::{
    chrono::{DateTime, Local},
    Uuid,
};
#[cfg(not(feature = "ssr"))]
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Adresse {
    pub adresse_id: Uuid,
    pub plz: String,
    pub strasse: String,
    pub hausnr: String,
    pub stadtteil: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Organisation {
    pub organisation_id: Uuid,
    pub organisation_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonOrganisation {
    pub organisation: Organisation,
    pub adressen: Vec<Adresse>,
    pub links: Vec<Link>,
    pub apartner: Vec<Ansprechpartner>,
    pub angebote: Vec<Angebot>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Email {
    pub email_id: Uuid,
    pub email_address: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Telefonnummer {
    pub telefonnummer_id: Uuid,
    pub land_vorwahl: String,
    pub lokale_nummer: String,
    pub festnetz: bool,
    pub komplette_nummer: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Link {
    pub link_id: Uuid,
    pub link: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Angebot {
    pub angebot_id: Uuid,
    pub angebot_name: String,
    pub beschreibung: Option<String>,
    pub kategorie: String,
    pub organisation_id: Uuid,
    pub created: DateTime<Local>,
    pub last_modified: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonAngebot {
    pub angebot: Angebot,
    pub organisation: JsonOrganisation,
    pub adressen: Vec<Adresse>,
    pub links: Vec<Link>,
    pub apartner: Vec<JsonAnsprechpartner>,
    pub sonstiges: Vec<Sonstiges>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Sonstiges {
    pub sonstiges_id: Uuid,
    pub text: String,
    pub angebot_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Ansprechpartner {
    pub ansprechpartner_id: Uuid,
    pub nach_name: String,
    pub vor_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct JsonAnsprechpartner {
    pub ansprechpartner: Ansprechpartner,
    pub emails: Vec<Email>,
    pub telefonnummern: Vec<Telefonnummer>,
}
