use crate::{
    components::{header::Header, offer_table::OfferTable},
    database::services,
};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let offers = Resource::new(|| (), move |_| services::get_offers());

    view! {
        <Header />
        <OfferTable offers />
    }
}
