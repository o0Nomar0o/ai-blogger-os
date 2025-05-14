use dioxus::prelude::*;

const LOADER_CSS: Asset = asset!("/assets/styling/loader.css");


#[component]
pub fn Loader() -> Element{
    rsx!{

       document::Link { rel: "stylesheet", href: LOADER_CSS }
        div { class: "typewriter",
            div {class: "slide", i{} }
            div {class: "paper"}
            div {class: "keyboard"}
        }
    }
}