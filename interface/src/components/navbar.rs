use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { id: "navbar", class: "navbar", role: "navigation",
            div { id: "navbar-menu", class: "navbar-menu",
                div { class: "navbar-start",
                    Link { class: "navbar-item", to: Route::Home {}, "Home" }
                    Link { class: "navbar-item", to: Route::Blog { id: 1 }, "Blog" }
                    Link { class: "navbar-item", to: Route::InputForm { id: 1 }, "Input Form" }
                }
            }
        }

        Outlet::<Route> {}
    }
}
