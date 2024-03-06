use wasm_bindgen::prelude::*;
use web_sys::{Event, EventTarget, Window, Document, MouseEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn page_x_offset(window: &Window) -> f64;

    #[wasm_bindgen(js_namespace = window)]
    fn page_y_offset(window: &Window) -> f64;
}

fn show_scrollbar(document: &Document) -> Result<(), JsValue> {
    if let Some(body) = document.body() {
        body.set_attribute("style", "overflow: auto").map_err(|err| err.into())
    } else {
        Err("Body element not found".into())
    }
}

fn hide_scrollbar(document: &Document) -> Result<(), JsValue> {
    if let Some(body) = document.body() {
        body.set_attribute("style", "overflow: hidden").map_err(|err| err.into())
    } else {
        Err("Body element not found".into())
    }
}

#[wasm_bindgen]
pub fn monitor_scrollbar() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("Failed to get window object")?;
    let document = window.document().ok_or("Failed to get document object")?;

    let document_scroll = document.clone();
    let window_scroll = window.clone();
    let scroll_callback = Closure::wrap(Box::new(move |_: Event| {
        let x = page_x_offset(&window_scroll);
        let y = page_y_offset(&window_scroll);
        if x != 0.0 || y != 0.0 {
            show_scrollbar(&document_scroll).unwrap_or_else(|err| console_error!("Error: {:?}", err));
        }
    }) as Box<dyn FnMut(Event)>);

    let document_resize = document.clone();
    let resize_callback = Closure::wrap(Box::new(move |_: Event| {
        show_scrollbar(&document_resize).unwrap_or_else(|err| console_error!("Error: {:?}", err));
    }) as Box<dyn FnMut(Event)>);

    let window_mousemove = window.clone();
    let document_mousemove = document.clone();
    let mousemove_callback = Closure::wrap(Box::new(move |event: Event| {
        let window_width = window_mousemove.inner_width().unwrap().as_f64().unwrap() as u32;
        let window_height = window_mousemove.inner_height().unwrap().as_f64().unwrap() as u32;
        let edge_threshold = 10;

        if let Ok(mouse_event) = event.dyn_into::<MouseEvent>() {
            let mouse_x = mouse_event.client_x() as u32;
            let mouse_y = mouse_event.client_y() as u32;

            if mouse_x <= edge_threshold
                || mouse_x >= window_width - edge_threshold
                || mouse_y <= edge_threshold
                || mouse_y >= window_height - edge_threshold
            {
                show_scrollbar(&document_mousemove).unwrap_or_else(|err| console_error!("Error: {:?}", err));
            } else {
                hide_scrollbar(&document_mousemove).unwrap_or_else(|err| console_error!("Error: {:?}", err));
            }
        }
    }) as Box<dyn FnMut(Event)>);

    let cloned_window = window.clone();
    let event_target = cloned_window.dyn_into::<EventTarget>()?;

    add_event_listener(&event_target, "scroll", &scroll_callback)?;
    add_event_listener(&event_target, "resize", &resize_callback)?;
    add_event_listener(&event_target, "mousemove", &mousemove_callback)?;

    scroll_callback.forget(); 
    resize_callback.forget(); 
    mousemove_callback.forget(); 

    Ok(())
}

fn add_event_listener(event_target: &EventTarget, event_type: &str, callback: &Closure<dyn FnMut(Event)>) -> Result<(), JsValue> {
    event_target.add_event_listener_with_callback(event_type, callback.as_ref().unchecked_ref())?;
    Ok(())
}

#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
