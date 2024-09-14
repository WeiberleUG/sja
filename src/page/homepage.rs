use crate::components::{
    header::HomeHeader,
    offer_table::{OfferTable, SelectedPage},
};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <HomeHeader />
        <OfferTable selected=SelectedPage::Default />
    }
}
