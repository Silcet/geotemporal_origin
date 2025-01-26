use crate::Route;

use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let mut favorites = use_resource(crate::backend::list_dogs);
    let favorite_dogs = favorites.suspend()?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favorite_dogs().unwrap() {
                    div { key: id, class: "favorite-dog",
                        Link { to: Route::FavoriteDog { id: id },
                            img { src: "{url}" }
                        }
                        button {
                            id: "delete-button",
                            onclick: move |_| async move {
                                crate::backend::delete_dog(id).await.unwrap();
                                favorites.restart();
                            },
                            "❌"
                        }
                    }
                }
            }
        }
    }
}
