# eltr

`eltr` is an experimental Rust crate for creating HTML elements with a declarative syntax, designed for use in WebAssembly projects. It provides macros to simplify the creation of DOM elements, handling of attributes, and event listeners.

**⚠️ Warning: This crate is a work in progress and is currently in an experimental stage. APIs may change, and it may not be suitable for production use yet.**

## Features

- Declarative syntax for creating HTML elements
- Easy attribute setting
- Simplified event listener attachment
- Re-exports `wasm_bindgen` and `web_sys` for convenience

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
eltr = { git = "https://github.com/c12i/eltr" }
```

Note: You don't need to explicitly include `wasm_bindgen` or `web_sys` in your dependencies, as `elt` re-exports these for you.

## Usage

Here's a basic example of how to use `eltr`:

```rust,no_run
use std::rc::Rc;
use std::cell::RefCell;

use eltr::wasm_bindgen::prelude::*;
use eltr::web_sys::*;
use eltr::{attr, elt, text, cb};

fn create_app() -> Element {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = Rc::clone(&click_count);

    elt!("div",
        {
            class: attr!("container")
        },
        elt!("h1", text!("Welcome to elt!")),
        elt!("p", text!("Click the button below:")),
        elt!("button",
            {
                onclick: cb!(
                    Box::new(move |_event: Event| {
                        *click_count_clone.borrow_mut() += 1;
                        console::log_1(&format!("Clicked {} times!", click_count_clone.borrow()).into());
                    }) as Box<dyn FnMut(Event)>
                ),
                class: attr!("btn btn-primary")
            },
            text!("Click me!")
        ),
        elt!("p", text!("This is an experimental library."))
    )
}

#[wasm_bindgen(start)]
pub fn run() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let app = create_app();
    body.append_child(&app).unwrap();
}
```

This example creates a simple app with a button that counts clicks and logs them to the console.

## Documentation

For more detailed documentation, please run `cargo doc --open` in your project directory.

## Contributing

As this is an experimental project, contributions, suggestions, and feedback are welcome! Please feel free to open issues or submit pull requests.

## License

This project is licensed under [LICENSE NAME] - see the [LICENSE.md](LICENSE.md) file for details.

---

Remember, `eltr` is still experimental and potentially buggy.
