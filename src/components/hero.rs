use dioxus::logger::tracing;
use dioxus::{logger::tracing::trace, prelude::*};
use web_sys::{window, MouseEvent};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use crate::app::desktop::{WindowState, WINDOW_COORD_X, WINDOW_COORD_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::app::app_type::AppType;
use crate::components::compose::ComposeBlog;
use crate::components::{MarkdownViewer_Response, Terminal};
use crate::utils::resize;

const DESK_CSS: Asset = asset!("/assets/styling/desk.css");

#[component]
pub fn Hero() -> Element {
    rsx! {
       DesktopUI {  }
    }
}

#[component]
pub fn DesktopUI() -> Element {
    
    let mut open_app = use_signal(|| None as Option<&'static str>);

    let mut windows = use_signal(|| vec![
        WindowState {
            app_type: AppType::MD_GEN,
            id: 1,
            title: "Markdown Generator",
            x: WINDOW_COORD_X,
            y: WINDOW_COORD_Y,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            is_dragging: false,
            drag_offset: (0, 0),
            is_open: false,
            z_index: 0,
        },
        WindowState {
            app_type: AppType::TERMINAL,
            id: 2,
            title: "Terminal",
            x: 200,
            y: 150,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            is_dragging: false,
            drag_offset: (0, 0),
            is_open: true,
            z_index: 0,
        }
    ]);

    use_context_provider(|| windows);

    let mut open_windows_ref = windows()
    .iter()
    .filter(|w| w.is_open)
    .cloned() 
    .collect::<Vec<_>>(); 
    
    let mut z_counter = use_signal(|| 1); 

    rsx! {
        
        div { class: "desktop",
            document::Link { rel: "stylesheet", href: DESK_CSS }
            // Desktop Icons
            div { class: "icon-grid",
            
                for win in windows().iter() {
                  
                    div {
                        class: "icon",
                        onclick: {
                            let win_id = win.id;


                            move |_| {
                                windows.write().iter_mut()
                                .find(|w| w.id == win_id)
                                .map(|w| {
                                    w.is_open = true;
                                    w.z_index = z_counter();
                                    z_counter.set(z_counter() + 1);
                                });
                            }
                        
                        },
                        div { class: "icon-img", "📄" }
                        span { class: "icon-text", "{win.title}" }
                    }
                }
            }
    
            // Render windows

            for win in open_windows_ref{
                {   
                   
                    rsx!{
                        div {
                            class: "window",
                            style: "position: absolute; left: {win.x}px; top: {win.y}px; width: {win.width}px; height: {win.height}px; z-index: {win.z_index}; display: flex; flex-direction: column;",
                            
                            onmousedown: move |_|{
                                if let Some(w) = windows.write().iter_mut().find(|w| w.id == win.id) {
                                    w.z_index = z_counter(); 
                                    z_counter.set(z_counter() + 1);
                                    tracing::debug!("{}", z_counter());
        
                                }
                            },
        
                            div { class: "window-header", 
                                onmousedown: move |e| {
                                    let offset_x = e.data().client_coordinates().x - (win.x as f64);
                                    let offset_y = e.data().client_coordinates().y - (win.y as f64);
            
                                    windows.write().iter_mut()
                                    .find(|w| w.id == win.id)
                                    .map(|w| {
                                        w.is_dragging = true;
                                        w.drag_offset = (offset_x as i32, offset_y as i32);
                                    });
        
                                    let mut windows = windows.clone();
                                    let drag_id = win.id.clone();
        
                                    let mouse_move_closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
                                        let x = event.client_x();
                                        let y = event.client_y();
                                
                                        let mut wins = windows.write();
                                        if let Some(w) = wins.iter_mut().find(|w| w.id == drag_id && w.is_dragging) {
                                            let new_x = x - w.drag_offset.0;
                                            let new_y = y - w.drag_offset.1;
        
                                            let (viewport_w, viewport_h) = {
                                                let win = web_sys::window().unwrap();
                                                let doc = win.document().unwrap();
                                                let elem = doc.document_element().unwrap();
                                                (elem.client_width(), elem.client_height())
                                                };
                                            // Clamp to stay inside the screen
                                            w.x = new_x.clamp(0, viewport_w - w.width);
                                            w.y = new_y.clamp(0, viewport_h - w.height);
                                        }
                                    });
        
                                    let mouse_up_closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
                                        let mut wins = windows.write();
                                        if let Some(w) = wins.iter_mut().find(|w| w.id == drag_id) {
                                            w.is_dragging = false;
                                        }
                            
                                    });
        
                                    let window = window().unwrap();
                                    window
                                        .remove_event_listener_with_callback("mousemove", mouse_move_closure.as_ref().unchecked_ref())
                                        .unwrap();
                                    window
                                        .remove_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref())
                                        .unwrap();
        
                                    window
                                        .add_event_listener_with_callback("mousemove", mouse_move_closure.as_ref().unchecked_ref())
                                        .unwrap();
                                    window
                                        .add_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref())
                                        .unwrap();
                            
                                    // Prevent closures from being dropped
                                    mouse_move_closure.forget();
                                    mouse_up_closure.forget();
                        
                                },
        
                            
                                "{win.title}", 
                                button { 
                                    onclick: move |_|{ 
                                        windows.write().iter_mut()
                                        .find(|w| w.id == win.id)
                                        .map(|w|{
                                            w.is_open = false;
                                            w.x = WINDOW_COORD_X;
                                            w.y = WINDOW_COORD_Y;
                                            w.width = WINDOW_WIDTH;
                                            w.height = WINDOW_HEIGHT;

                                        });
                                        },
                                    "x"
                                },
                                
                                
                            }
        
                            div { class: "window-body", 
                                {
                                match win.app_type {
                                    AppType::TERMINAL => rsx!( Terminal { } ),
                                    AppType::SETTINGS => rsx!( div { "Settings App" } ),
                                    AppType::MD_GEN => rsx!( ComposeBlog {} ),
                                    AppType::MY_DOCS => rsx!( div { "My Docs" } ),
                                    AppType::BROWSE => rsx!( div { "Browse Files" } ),
                                    AppType::MY_DOCS_RE => rsx!( MarkdownViewer_Response {  }),
                                }
                                }
        
                            }
        
                            for dir in ["top", "right", "bottom", "left", "top-left", "top-right", "bottom-left", "bottom-right"] 
                            { // for loop bracket
                            { // for main bracket

                                let win_clone = win.clone();

                                rsx!{
                                    div {
                                        class: "resize-handle {dir}",
                                        onmousedown: move |e|
                                        {
                                            resize(e.data(), win_clone.clone(), windows, dir.to_string())
                                        }
                                    }
                                }

                            }    
                            }
                        }
                    }
                }
            }
        }
    }
}

