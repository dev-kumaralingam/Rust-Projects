use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, MouseEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window().expect("should have a window");
    let window_ref = window.clone(); // Clone a reference to the window

    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("should have a body on document");

    // Create a new div element for displaying the message
    let message_div = document.create_element("div")?;
    let message_div: HtmlElement = message_div.dyn_into::<HtmlElement>()?;
    message_div.set_text_content(Some("Near edge of the window"));

    // Add the message div to the body
    body.append_child(&message_div)?;

    // Add event listener to show/hide the message div based on cursor position
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let inner_width = window_ref.inner_width().unwrap().as_f64().unwrap();
        let inner_height = window_ref.inner_height().unwrap().as_f64().unwrap();
        let x = event.client_x() as f64;
        let y = event.client_y() as f64;

        // Check if the cursor is near the edges of the window
        if (x <= 10.0 || y <= 10.0 || x >= inner_width - 10.0 || y >= inner_height - 10.0)
            && message_div.get_attribute("style").unwrap_or_default() != "display:block;"
        {
            // Show the message div
            message_div.set_attribute("style", "display:block;").unwrap();
            log("Near edge of the window");
        } else {
            // Hide the message div
            message_div.set_attribute("style", "display:none;").unwrap();
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    // Add event listener directly on the window
    window
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();

    Ok(())
}
