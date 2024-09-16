use crate::{components::show_offer::ShowOffer, database::structs::JsonAngebot};
use leptos::prelude::*;

#[derive(Debug)]
pub enum SelectedPage {
    Default,
    Outdated,
}

#[component]
pub fn OfferTable(offers: Resource<Result<Vec<JsonAngebot>, ServerFnError>>) -> impl IntoView {
    let offer_view = move || {
        offers.with(move |x| {
            x.clone().map(move |res| {
                view! {
                    <table>
                        <tbody>
                            <tr>
                                <th>Name</th>
                                <th>Link</th>
                                <th>Beschreibung</th>
                                <th>Adresse</th>
                                <th>Stadtteil</th>
                                <th>Ansprechpartner</th>
                                <th>Email</th>
                                <th>Telefonnummer</th>
                                <th>Sonstiges</th>
                            </tr>
                            <For
                                each=move || res.clone().unwrap_or_default().into_iter().enumerate()
                                key=|(i, _)| *i
                                children=move |(_, offer): (usize, JsonAngebot)| {
                                    view! { <ShowOffer offer /> }
                                }
                            />
                        </tbody>
                    </table>
                }
            })
        })
    };

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>{offer_view}</Suspense>
    }
}
