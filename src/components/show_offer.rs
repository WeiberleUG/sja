use crate::database::structs::JsonAngebot;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ShowOffer(offer: JsonAngebot) -> impl IntoView {
    view! {
        <tr>
            <td>{offer.angebot.angebot_name}</td>
            <td>
                {offer
                    .links
                    .into_iter()
                    .map(|link| {
                        view! {
                            <a href=link.link.clone()>{link.link.clone()}</a>
                            <br />
                            <br />
                        }
                    })
                    .collect_view()}
            </td>
            <td>{offer.angebot.beschreibung}</td>
            <td>
                {offer
                    .adressen
                    .clone()
                    .into_iter()
                    .map(|adress| {
                        view! {
                            <p>
                                {format!("{} {}", adress.strasse, adress.hausnr)}<br />
                                {format!("{} {}", adress.plz, adress.stadtteil)}<br /><br />
                            </p>
                        }
                    })
                    .collect_view()}
            </td>
            <td>
                {offer
                    .adressen
                    .clone()
                    .into_iter()
                    .map(|adress| {
                        view! {
                            {adress.stadtteil}
                            <br />
                            <br />
                        }
                    })
                    .collect_view()}
            </td>
            <td>
                {offer
                    .apartner
                    .clone()
                    .into_iter()
                    .map(|ap| {
                        view! {
                            {format!(
                                "{}, {}",
                                ap.ansprechpartner.nach_name,
                                ap.ansprechpartner.vor_name,
                            )}
                            <br />
                        }
                    })
                    .collect_view()}
            </td>
            <td>
                {offer
                    .apartner
                    .clone()
                    .into_iter()
                    .map(|ap| {
                        ap.emails
                            .into_iter()
                            .map(|em| {
                                let email = em.email_address.clone();
                                view! {
                                    <A href=move || format!("mailto:{}", email)>
                                        {em.email_address.clone()}
                                    </A>
                                    <br />
                                }
                            })
                            .collect_view()
                    })
                    .collect_view()}
            </td>
            <td>
                {offer
                    .apartner
                    .clone()
                    .into_iter()
                    .map(|ap| {
                        ap.telefonnummern
                            .into_iter()
                            .map(|tn| {
                                view! {
                                    {tn.komplette_nummer}
                                    <br />
                                }
                            })
                            .collect_view()
                    })
                    .collect_view()}
            </td>
            <td>
                {offer
                    .sonstiges
                    .into_iter()
                    .map(|sonst| {
                        view! {
                            {sonst.text}
                            <br />
                        }
                    })
                    .collect_view()}
            </td>
        </tr>
    }
}
