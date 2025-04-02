use crate::database::{
    services,
    structs::{Angebot, Email},
};
use leptos::prelude::*;

#[island]
pub fn OfferForm() -> impl IntoView {
    let add_offer: ServerAction<services::add_offer::AddOffer> = ServerAction::new();
    let value = add_offer.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    let existingAngebote = Resource::new(|| (), move |_| services::get_all::get_angebote());
    let existingAngebote = move || {
        existingAngebote.with(move |x| {
            x.clone().map(move |res| {
                view! {
                    <datalist id="existingAngeboteName">
                        <For
                            each=move || { res.clone().unwrap().into_iter().enumerate() }
                            key=|(i, _)| *i
                            children=move |(_, angebot): (usize, Angebot)| {
                                view! {
                                    <option value=angebot
                                        .angebot_name
                                        .to_string()>{angebot.angebot_name.clone()}</option>
                                }
                            }
                        />
                    </datalist>
                }
            })
        })
    };

    let existingMails = Resource::new(|| (), move |_| services::get_all::get_mails());
    let existingMails = move || {
        existingMails.with(move |x| {
            x.clone().map(move |res| {
                view! {
                    <datalist id="existingMails">
                        <For
                            each=move || { res.clone().unwrap().into_iter().enumerate() }
                            key=|(i, _)| *i
                            children=move |(_, mail): (usize, Email)| {
                                view! {
                                    <option value=mail
                                        .email_address
                                        .to_string()>{mail.email_address.clone()}</option>
                                }
                            }
                        />
                    </datalist>
                }
            })
        })
    };

    view! {
        <ActionForm action=add_offer>
            <label for="angebote">"Angebot hinzufügen"</label>
            <input
                name="offer"
                id="angebote"
                type="text"
                list="existingAngeboteName"
                placeholder="Hier eingeben"
            />
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>{existingAngebote}</Suspense>
            <label for="mails">"E-Mail hinzufügen"</label>
            <input
                name="email"
                id="mails"
                type="text"
                list="existingMails"
                placeholder="Hier eingeben"
            />
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>{existingMails}</Suspense>
            <input type="submit" value="Hinzufügen" />
        </ActionForm>
    }
}
