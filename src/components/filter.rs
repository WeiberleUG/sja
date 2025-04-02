use crate::database::structs::JsonAngebot;
use leptos::prelude::*;
use levenshtein::levenshtein;

#[component]
pub fn Filter(filter: RwSignal<Vec<(JsonAngebot, bool)>>) -> impl IntoView {
    let search_string = RwSignal::new(String::from("Default"));

    view! {
        <input
            type="text"
            on:input=move |ev| {
                search_string.set(event_target_value(&ev));
                filter
                    .update(|offer| {
                        offer
                            .iter_mut()
                            .map(|offer| {
                                log::warn!("{}", levenshtein(&offer.0.angebot.angebot_name, &search_string.get()));
                                if levenshtein(&offer.0.angebot.angebot_name, &search_string.get()) > 7 {
                                    offer.1 = true;
                                }
                            })
                            .collect()
                    });
            }
        />
    }
}
