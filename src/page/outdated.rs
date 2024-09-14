use crate::components::{
    header::OutdatedHeader,
    offer_table::{OfferTable, SelectedPage},
};
use leptos::prelude::*;

#[component]
pub fn OutdatedPage() -> impl IntoView {
    view! {
        <OutdatedHeader />
        <OfferTable selected=SelectedPage::Outdated />
    }
}
