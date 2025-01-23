use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let skip = move |evt| {};
    let save = move |evt| {};

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { id: "skip", onclick: skip, "skip" }
            button { id: "save", onclick: save, "save!" }
        }
    }
}
