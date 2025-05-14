use dioxus::prelude::*;
use pulldown_cmark::{Parser, Options, html};

use crate::global::GENERATED_DATA;

const MARKDOWN_CSS: Asset = asset!("/assets/styling/markdown.css");

#[component]
pub fn MarkdownViewer_Response() -> Element {
    
    let markdown_data = GENERATED_DATA.read().unwrap();
    
    // let rendered_html = match &*markdown_data{
    //     Some(res_data) => {
    //         if let Some(file) = res_data.data.first() {
    //             markdown_to_html(&file.response)
    //         } else {
    //             "**No files found**".to_string()
    //         }
    //     }
    //     None => "**File not found**".to_string(),
    // };

    let rendered_htmls = match &*markdown_data {
        Some(res_data) => {
            res_data.data
                .iter()
                .map(|file| markdown_to_html(&file.response))
                .collect::<Vec<_>>()
        },
        None => vec!["**File not found**".to_string()],
    };

    rsx!{
        document::Link { rel: "stylesheet", href: MARKDOWN_CSS }
        for html in rendered_htmls {
            div {
                class: "markdown-div",
                dangerous_inner_html: "{html}"
            }
        }
    }
}



fn markdown_to_html(markdown_input: &str) -> String {
    let parser = Parser::new_ext(markdown_input, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
