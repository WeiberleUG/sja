use crate::database::{services, structs::Email};
use leptos::prelude::*;

#[island]
pub fn OfferForm() -> impl IntoView {
    let existingMails = Resource::new(|| (), move |_| services::get_all::get_mails());

    let add_offer: ServerAction<services::add_offer::AddOffer> = ServerAction::new();
    let value = add_offer.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    let existingMails = move || {
        existingMails.with(move |x| {
            x.clone().map(move |res| {
                view! {
                    <datalist id="existingMails">
                        <For
                            each=move || { res.clone().unwrap().into_iter().enumerate() }
                            key=|(i, _)| *i
                            children=move |(_, mail): (usize, Email)| {
                                view! { <option value="Test">{mail.email_address.clone()}</option> }
                            }
                        />
                    </datalist>
                }
            })
        })
    };

    view! {
        <ActionForm action=add_offer>
            <label for="existingMails">"Angebot hinzufügen"</label>
            <input type="text" list="existingMails" placeholder="Hier eingeben" />
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>{existingMails}</Suspense>
            <input type="submit" value="Hinzufügen" />
        </ActionForm>
    }
}
