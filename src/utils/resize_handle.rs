
use dioxus::{logger::tracing::trace, prelude::*};
use web_sys::{window, MouseEvent};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use crate::app::desktop::{WindowState, WINDOW_COORD_X, WINDOW_COORD_Y};
use crate::app::app_type::AppType;


pub fn resize(e: Rc<MouseData>, win: WindowState, windows: Signal<Vec<WindowState>>, dir: String){

    let start_x = e.client_coordinates().x as i32;
    let start_y = e.client_coordinates().y as i32;

    let orig_width = win.width;
    let orig_height = win.height;
    let orig_x = win.x;
    let orig_y = win.y;

    let resize_id = win.id.clone();
    let direction = dir.to_string(); 

    let mouse_move = Rc::new(RefCell::new(None::<Closure<dyn FnMut(MouseEvent)>>));
    let mouse_up = Rc::new(RefCell::new(None::<Closure<dyn FnMut(MouseEvent)>>));
    
    let mm = {
        let mouse_move = mouse_move.clone();
        let mouse_up = mouse_up.clone();
        let mut wins = windows.clone();
        Closure::wrap(Box::new(move |event: MouseEvent| {
            let x = event.client_x();
            let y = event.client_y();
            let delta_x = x - start_x;
            let delta_y = y - start_y;

            let (viewport_w, viewport_h) = get_viewport_size();


            if let Some(w) = wins.write().iter_mut().find(|w| w.id == resize_id) {
                match direction.as_str() {
                    "top" => {
                        let new_y = (orig_y as i32 + delta_y).clamp(0, orig_y + orig_height - 100);
                        let new_height = (orig_height as i32 - delta_y).clamp(100, viewport_h);
                        w.y = new_y;
                        w.height = new_height;
                    }

                    "top-right" => {
                        let new_y = (orig_y as i32 + delta_y).clamp(0, orig_y + orig_height - 100);
                        let new_height = (orig_height as i32 - delta_y).clamp(100, viewport_h);
                        let new_width = (orig_width as i32 + delta_x).clamp(200, viewport_w - orig_x);

                        w.y = new_y;
                        w.height = new_height;
                        w.width = new_width;
                    }

                    "top-left" => {
                        let new_x = (orig_x as i32 + delta_x).clamp(0, orig_x + orig_width - 200);
                        let new_y = (orig_y as i32 + delta_y).clamp(0, orig_y + orig_height - 100);
                        let new_width = (orig_width as i32 - delta_x).clamp(200, viewport_w);
                        let new_height = (orig_height as i32 - delta_y).clamp(100, viewport_h);
                
                        w.x = new_x;
                        w.y = new_y;
                        w.width = new_width;
                        w.height = new_height;
                    }

                    "right" => {
                        w.width = (orig_width as i32 + delta_x).clamp(200, viewport_w - w.x);
                    }

                    "bottom" => {
                        w.height = (orig_height as i32 + delta_y).clamp(100, viewport_h - w.y);
                    }

                    "bottom-right" => {
                        w.width = (orig_width as i32 + delta_x).clamp(200, viewport_w - w.x);
                        w.height = (orig_height as i32 + delta_y).clamp(100, viewport_h - w.y);
                    }


                    "bottom-left" => {
                        let new_x = (orig_x as i32 + delta_x).clamp(0, orig_x + orig_width - 200);
                        let new_width = (orig_width as i32 - delta_x).clamp(200, viewport_w);
                        let new_height = (orig_height as i32 + delta_y).clamp(100, viewport_h - orig_y);

                        w.x = new_x;
                        w.width = new_width;
                        w.height = new_height;
                    }

                    "left" => {
                        let new_x = (orig_x as i32 + delta_x).clamp(0, orig_x + orig_width - 200);
                        let new_width = (orig_width as i32 - delta_x).clamp(200, viewport_w);
                        w.x = new_x;
                        w.width = new_width;
                    }

                    _ => {}
                }
            }
        }) as Box<dyn FnMut(_)>)
    };

    let mu = {
        let mouse_move = mouse_move.clone();
        let mouse_up = mouse_up.clone();
        Closure::wrap(Box::new(move |_e: MouseEvent| {
            let win = web_sys::window().unwrap();

            if let Some(mm_ref) = &*mouse_move.borrow() {
                win.remove_event_listener_with_callback("mousemove", mm_ref.as_ref().unchecked_ref()).unwrap();
            }

            if let Some(mu_ref) = &*mouse_up.borrow() {
                win.remove_event_listener_with_callback("mouseup", mu_ref.as_ref().unchecked_ref()).unwrap();
            }

            mouse_move.borrow_mut().take();
            mouse_up.borrow_mut().take();
        }) as Box<dyn FnMut(_)>)
    };

    *mouse_move.borrow_mut() = Some(mm);
    *mouse_up.borrow_mut() = Some(mu);

    let win = window().unwrap();
    win.add_event_listener_with_callback("mousemove", mouse_move.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
    win.add_event_listener_with_callback("mouseup", mouse_up.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();


}

fn get_viewport_size() -> (i32, i32) {
    let win = window().expect("no global `window` exists");
    let doc = win.document().expect("should have a document");
    let elem = doc.document_element().expect("should have document element");
    (elem.client_width(), elem.client_height())
}
