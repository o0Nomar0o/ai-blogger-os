use dioxus::prelude::*;
use crate::ascii_art::GREETINGS;

const TERMINAL_CSS: Asset = asset!("/assets/styling/terminal.css");

#[component]
pub fn Terminal() -> Element{

    rsx! {
        document::Link { rel: "stylesheet", href: TERMINAL_CSS }

        pre {
            code {
                "{GREETINGS}"
            }
        }

    }

}