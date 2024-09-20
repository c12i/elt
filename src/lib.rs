#[doc = include_str!("../README.md")]
pub mod elt;
mod props;

pub use props::PropType;

pub mod wasm_bindgen {
    pub use wasm_bindgen::*;
}

pub mod web_sys {
    pub use web_sys::*;
}
