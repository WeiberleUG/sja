use crate::{
    components::show_offers::ShowOffers,
    database::{services, structs::JsonAngebot},
};
use leptos::prelude::*;

#[derive(Debug)]
pub enum SelectedPage {
    Default,
    Outdated,
}

#[component]
pub fn OfferTable(selected: SelectedPage) -> impl IntoView {
    let offers = match selected {
        SelectedPage::Default => Resource::new(|| (), move |_| services::get_offers()),
        //  TODO: make get_outdated <2024-09-14>
        SelectedPage::Outdated => Resource::new(|| (), move |_| services::get_offers()),
    };

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
                view! { <ShowOffers offers /> }
            }}
        </Suspense>
    }
}
