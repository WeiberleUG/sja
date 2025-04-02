use crate::{components::show_offer::ShowOffer, database::structs::JsonAngebot};
use leptos::prelude::*;

#[component]
pub fn OfferTable(filter: RwSignal<Vec<(JsonAngebot, bool)>>) -> impl IntoView {
    let offer_view = move || {
        view! {
            <div class="table-container">
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
                            each=move || { filter.get().into_iter().enumerate() }
                            key=|(i, _)| *i
                            children=move |(_, (offer, hidden)): (usize, (JsonAngebot, bool))| {
                                if hidden {
                                    view! {}.into_any()
                                } else {
                                    view! { <ShowOffer offer=offer hidden=hidden /> }.into_any()
                                }
                            }
                        />
                    </tbody>
                </table>
            </div>
        }
    };

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>{offer_view}</Suspense>
    }
}
