use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use dioxus::prelude::*;
use dioxus::{ html::HasFileData, prelude::dioxus_elements::FileEngine };

use crate::app::app_type::AppType;
use crate::app::desktop::{WindowState, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::components::loader;
use crate::model::{GeneratedResponse, UploadedFile};
use crate::utils::send_files_to_python;

use crate::global::GENERATED_DATA;

const COMPOSE_CSS: Asset = asset!("/assets/styling/compose.css");
const UPLOAD_SVG: Asset = asset!("/assets/upload.svg");
const FILE_SVG: Asset = asset!("/assets/file.svg");
const TRASH_SVG: Asset = asset!("/assets/trash.svg");


#[component]
pub fn ComposeBlog() -> Element {

    let mut is_loading = use_signal(|| false);

    let mut enable_directory_upload = use_signal(|| false);
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<UploadedFile>);
    let mut hovered = use_signal(|| false);


    let read_files = move |file_engine: Arc<dyn FileEngine>| {
        let mut files_uploaded = files_uploaded.clone();
        async move {
            let files = file_engine.files();
            for file_name in &files {
                if let Some(contents) = file_engine.read_file_to_string(file_name).await {
                    files_uploaded.write().push(UploadedFile {
                        name: file_name.clone(),
                        contents,
                    });
                }
            }
        }
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };

    rsx! {

        document::Link { rel: "stylesheet", href: COMPOSE_CSS }

        div {class: "container",
            div { class: "header",
                ondragover: move |evt| {
                    evt.prevent_default();
                    hovered.set(true)
                },
                ondragleave: move |_| hovered.set(false),
                ondrop: move |evt| async move {
                    evt.prevent_default();
                    hovered.set(false);
                    if let Some(file_engine) = evt.files() {
                        read_files(file_engine).await;
                    }
                },
                img { src: UPLOAD_SVG, id: "hicon"}
                p { "Drop file here"}
            }
            label {
                for: "file",
                class: "footer",
                img { src: FILE_SVG, id: "file-icon" }
                p {
                    if files_uploaded.read().is_empty() {
                        "No selected file"
                    } else {
                        "{files_uploaded.read().get(0).map(|file| &file.name).unwrap_or(&\"No file selected\".to_string())}"
                    }
                }
                img {
                    src: TRASH_SVG,
                    id: "trash-icon",
                    onclick: move |evt| {
                        evt.prevent_default(); 
                        files_uploaded.write().clear(); 
                    }
                }
            }
            input { 
                r#type: "file", 
                id: "file", 
                multiple: true,
                directory: enable_directory_upload, 
                onchange: upload_files,
            }
          
        }

        ul {
            for file in files_uploaded.read().iter().rev() {
                li { id: "tempo",
                    span { "{file.name}" }
                    pre  { "{file.contents}"  }
                }
            }
        }

        button {  
            class: "submit-btn",
            onclick: move |_|{
                let files = files_uploaded.read().clone();
                is_loading.set(true);
                async move {

                    let mut windows = consume_context::<Signal<Vec<WindowState>>>();

                    match send_files_to_python(files).await {
                        Ok(response) => {
                            is_loading.set(false);
                            let mut data_lock = GENERATED_DATA.write().unwrap();
                            *data_lock = Some(response.clone());

                            //
                            windows.write().push(WindowState {
                                app_type: AppType::MY_DOCS_RE,
                                id: generate_window_id(),
                                title: "Markdown Viewer".into(),
                                x: 300,
                                y: 200,
                                width: WINDOW_WIDTH,
                                height: WINDOW_HEIGHT,
                                is_dragging: false,
                                drag_offset: (0, 0),
                                is_open: true,
                                z_index: 100,
                            });
                            //
                        },

                        Err(err) => {
                            is_loading.set(false);
                            eprintln!("Upload error: {:?}", err);
                            
                        }
                    }
                }
    
            },
            "Generate"
        }

        if *is_loading.read() {

            div { class: "loading-view-overlay",
                div { class: "loading-view",
                    loader::Loader {}
                }
            }

        }
    }
}

static WINDOW_ID_COUNTER: AtomicU32 = AtomicU32::new(1000);

fn generate_window_id() -> u32 {
    WINDOW_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}
