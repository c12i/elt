use wasm_bindgen::prelude::Closure;
use web_sys::Event;

#[derive(Debug)]
pub enum PropType {
    Attr(String),
    Callback(Closure<dyn FnMut(Event)>),
}
