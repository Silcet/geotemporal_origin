use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn FormTextInput(label: String, name: String, placeholder: String, required: bool) -> Element {
    rsx! {
        div { id: "{name}", class: "field",
            label { class: "label", "{label}" }
            div { class: "control",
                input {
                    class: "input",
                    r#type: "text",
                    name: "{name}",
                    placeholder: "{placeholder}",
                    required,
                }
            }
        }
    }
}
