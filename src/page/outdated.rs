use crate::components::{
    header::Header,
    offer_table::{OfferTable, SelectedPage},
};
use leptos::prelude::*;

#[component]
pub fn OutdatedPage() -> impl IntoView {
    view! {
        <Header />
        <OfferTable selected=SelectedPage::Outdated />
    }
}
