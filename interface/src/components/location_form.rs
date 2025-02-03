use crate::components::FormTextInput;
use crate::views::NUMBER_OF_FORMS;

use dioxus::{logger::tracing, prelude::*};
use std::collections::HashMap;

#[component]
pub fn LocationForm(id: u8) -> Element {
    tracing::info!("Got into form {id}");
    let mut values = use_signal(HashMap::new);
    let mut submitted_values = use_signal(HashMap::new);

    rsx! {
        div { class: "section",
            form {
                class: "control",
                id: "address-form",

                oninput: move |ev| {
                    values.set(ev.values());
                },

                onsubmit: move |ev| {
                    submitted_values.set(ev.values());
                    *NUMBER_OF_FORMS.write() += 1;
                },

                FormTextInput {
                    label: "Street and number",
                    name: "street",
                    placeholder: "My Street Name 1",
                    required: true,
                }

                FormTextInput {
                    label: "City",
                    name: "city",
                    placeholder: "City",
                    required: true,
                }

                FormTextInput {
                    label: "Country",
                    name: "country",
                    placeholder: "Country",
                    required: true,
                }

                button { class: "button is-primary", r#type: "submit", "Calculate" }
            }
            div { class: "content",
                h1 { "Values:" }
                pre { "{values:#?}" }
            }
            div { class: "content",
                h1 { "Submitted Values:" }
                pre { "{submitted_values:#?}" }
            }
        }
    }
}
