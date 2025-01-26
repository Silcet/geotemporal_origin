use crate::backend;

use dioxus::prelude::*;

#[derive(serde::Deserialize)]
pub struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    let save = move |_| async move {
        let current = img_src.cloned().unwrap();

        img_src.restart();

        backend::save_dog(current).await.unwrap();
    };

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { id: "skip", onclick: move |_| img_src.restart(), "skip" }
            button { id: "save", onclick: save, "save!" }
        }
    }
}

#[component]
pub fn FavoriteDog(id: usize) -> Element {
    let img_src = use_resource(move || async move { backend::get_dog(id).await.unwrap() });

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
    }
}
