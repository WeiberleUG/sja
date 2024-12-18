use crate::components::{header::Header, offer_form::OfferForm};
use leptos::prelude::*;

#[component]
pub fn NewPage() -> impl IntoView {
    view! {
        <Header />
        <OfferForm />
    }
}
