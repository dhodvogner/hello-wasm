extern crate wasm_bindgen;

use std::rc::Rc; // A single-threaded reference-counting pointer.
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; // for unchecked_ref()

#[wasm_bindgen]
extern "C" {
    // Bind the console.log from JS
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// Macro to use console_log! with format! (like println!)
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Expose your Rust function to JS
#[wasm_bindgen]
pub fn greet(name: &str) {
    console_log!("Hello, {}!", name);
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use web_sys::window() to access the DOM window object
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Create new elements and append to the body
    //?; -> "If the Result contains an error, return the error immediately. Otherwise, unwrap its value"
    let div = document.create_element("div")?;
    div.set_inner_html("I'm a div from Rust!");
    body.append_child(&div)?;

    let input = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;
    input.set_attribute("type", "text")?;
    input.set_attribute("placeholder", "Enter your name here")?;
    body.append_child(&input)?;

    let button = document.create_element("button")?;
    button.set_inner_html("Say Hello!");
    body.append_child(&button)?;

    // Add click event listener to button
    let div = Rc::new(div); // A new single-threaded reference-counting pointer.
    let input = Rc::new(input);
    {
        let div = div.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            let name = input.value();
            div.set_inner_html("A Hello was said! Go check your console!");
            console_log!("Hello {}, from the other side!", name);
        }) as Box<dyn FnMut(_)>);
        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
