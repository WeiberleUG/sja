use self::prelude::ServerFnError;
use crate::database::structs::Email;
use leptos::*;
use std::env;

#[server(AddOffer, "/api")]
async fn add_offer(offer: Email) -> Result<(), ServerFnError> {
    log::info!("{:?}", offer);
    todo!()
}
