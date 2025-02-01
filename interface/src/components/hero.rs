use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div {
                id: "links",
                class: "buttons is-grouped is-flex-direction-column",
                a {
                    class: "button is-large",
                    href: "https://dioxuslabs.com/learn/0.6/",
                    "📚 Learn Dioxus"
                }
                a {
                    class: "button is-large",
                    href: "https://dioxuslabs.com/awesome",
                    "🚀 Awesome Dioxus"
                }
                a {
                    class: "button is-large",
                    href: "https://github.com/dioxus-community/",
                    "📡 Community Libraries"
                }
                a {
                    class: "button is-large",
                    href: "https://github.com/DioxusLabs/sdk",
                    "⚙️ Dioxus Development Kit"
                }
                a {
                    class: "button is-large",
                    href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a {
                    class: "button is-large",
                    href: "https://discord.gg/XgGxMSkvUM",
                    "👋 Community Discord"
                }
            }
        }
    }
}
