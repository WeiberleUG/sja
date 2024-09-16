use crate::database::structs::JsonAngebot;
use leptos::prelude::*;

#[component]
pub fn ShowOffers(offers: Result<Vec<JsonAngebot>, ServerFnError>) -> impl IntoView {
    let offers = offers.unwrap_or_default();

    view! {
        <table>
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
            {offers
                .into_iter()
                .map(|offer| {
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
                                                view! {
                                                    {em.email_address}
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
                                                    {tn.komplette_nummer.unwrap_or_default()}
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
                })
                .collect_view()}
        </table>
    }
}
