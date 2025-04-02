use crate::{
    components::{filter::Filter, header::Header, offer_table::OfferTable},
    database::{services, structs::JsonAngebot},
};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let offers = Resource::new(|| (), move |_| services::get_offers::get_offers());
    let filter: RwSignal<Vec<(JsonAngebot, bool)>> = RwSignal::new(Vec::new());

    view! {
        <Header />

        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                filter
                    .set(
                        match offers.get() {
                            Some(_) => {
                                offers
                                    .with(move |x| {
                                        x.clone()
                                            .and_then(|res| res.ok())
                                            .unwrap_or_default()
                                            .iter()
                                            .map(|offer| (offer.clone(), false))
                                            .collect()
                                    })
                            }
                            None => Vec::new(),
                        },
                    );
            }} <Filter filter /> <OfferTable filter />
        </Suspense>
    }
}
