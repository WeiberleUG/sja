use crate::database::{services, structs::JsonAngebot};
use leptos::prelude::*;

#[component]
pub fn OfferTable() -> impl IntoView {
    let offers = Resource::new(|| (), move |_| services::get_offers());

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                let offers: Result<Vec<JsonAngebot>, ServerFnError> = match offers.get() {
                    Some(Ok(offers)) => {
                        log::info!("Successfully got: {:?}", offers);
                        Ok(offers)
                    }
                    Some(Err(err)) => {
                        log::error!("{:?}", err);
                        Err(err)
                    }
                    None => {
                        log::error!("No message received");
                        Err(ServerFnError::ServerError("Data not loaded yet".to_string()))
                    }
                };
                view! { <ShowData offers /> }
            }}
        </Suspense>
    }
}

#[component]
pub fn ShowData(offers: Result<Vec<JsonAngebot>, ServerFnError>) -> impl IntoView {
    let data = offers.unwrap_or_default();

    view! {
        <ul>
            {data.into_iter().map(|n| view! { <li>{n.angebot.angebot_name}</li> }).collect_view()}
        </ul>
    }
}
