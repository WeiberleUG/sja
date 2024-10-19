use crate::{
    components::{header::Header, offer_table::OfferTable},
    database::services,
};
use leptos::prelude::*;

#[component]
pub fn OutdatedPage() -> impl IntoView {
    let offers = Resource::new(|| (), move |_| services::get_offers::get_outdated());

    view! {
        <Header />
        <OfferTable offers />
    }
}
