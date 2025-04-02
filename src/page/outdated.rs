use crate::{
    components::{filter::Filter, header::Header, offer_table::OfferTable},
    database::{services, structs::JsonAngebot},
};
use leptos::prelude::*;

#[component]
pub fn OutdatedPage() -> impl IntoView {
    let offers = Resource::new(|| (), move |_| services::get_offers::get_outdated());
    let filter: RwSignal<Vec<(JsonAngebot, bool)>> = RwSignal::new(Vec::new());

    view! {
        <Header />
        <Filter filter />
        <OfferTable filter />
    }
}
