use crate::{components::LocationForm, Route};
use nominatim_api::Location;

use dioxus::{logger::tracing, prelude::*};
use std::collections::HashMap;

// pub city: String,
//     pub county: Option<String>,
//     pub state: Option<String>,
//     pub country: String,
//     pub postalcode: Option<u16>,

pub static NUMBER_OF_FORMS: GlobalSignal<u8> = Signal::global(|| 1);

#[component]
pub fn InputForm(id: u8) -> Element {
    tracing::info!("InputForm {id}");
    rsx! {
        div { class: "tabs",
            ul {
                for i in (1..=NUMBER_OF_FORMS()) {
                    li { class: if i == id { "is-active" } else { "" },
                        Link { to: Route::InputForm { id: i }, "{i}" }
                    }
                }
            }
        }
        LocationForm { id }
    }
}
