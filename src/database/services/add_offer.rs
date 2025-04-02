use self::prelude::ServerFnError;
use crate::database::structs::{Angebot, Email};
use leptos::*;
use std::env;

#[server(AddOffer, "/api")]
async fn add_offer(offer: String, email: String) -> Result<(), ServerFnError> {
    log::info!("{:?}", offer);
    log::info!("{:?}", email);
    todo!()
}
