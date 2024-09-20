#[macro_export]
macro_rules! elt {
    ($type:expr) => {{
        $crate::web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element($type)
            .unwrap()
    }};

    // with properties
    ($type:expr, { $($key:ident : $value:expr),* $(,)? }) => {{
        let element = elt!($type);
        $(
            match stringify!($key) {
                key if key.starts_with("on") => {
                    if let $crate::PropType::Callback(callback) = $value {
                        element
                            .add_event_listener_with_callback(
                                &key[2..],
                                callback.as_ref().unchecked_ref()
                            )
                            .unwrap();
                        callback.forget();
                    } else {
                        panic!("Expected a callback for event listener");
                    }
                },
                _ => {
                    if let $crate::PropType::Attr(attr_value) = $value {
                        element.set_attribute(stringify!($key), &attr_value).unwrap();
                    } else {
                        panic!("Expected a string attribute");
                    }
                }
            }
        )*
        element
    }};

    // with properties and children
    ($type:expr, { $($key:ident : $value:expr),* $(,)? }, $($child:expr),* $(,)?) => {{
        let element = elt!($type, { $($key: $value),* });
        $(
            match $child {
                child if child.is_instance_of::<$crate::web_sys::Node>() => {
                    element.append_child(child.unchecked_ref()).unwrap();
                },
                child => {
                    let text_node = $crate::web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .create_text_node(&child.to_string().as_string().unwrap_or_default());
                    element.append_child(&text_node).unwrap();
                },
            }
        )*
        element
    }};

    // with children and no properties
    ($type:expr, $($child:expr),* $(,)?) => {{
        let element = elt!($type);
        $(
            match $child {
                child if child.is_instance_of::<$crate::web_sys::Node>() => {
                    element.append_child(child.unchecked_ref()).unwrap();
                },
                child => {
                    let text_node = $crate::web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .create_text_node(&child.to_string().as_string().unwrap_or_default());
                    element.append_child(&text_node).unwrap();
                },
            }
        )*
        element
    }};
}

#[macro_export]
macro_rules! text {
    ($content:expr) => {{
        $crate::web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_text_node(&$content.to_string())
    }};
}

#[macro_export]
macro_rules! attr {
    ($value:expr) => {
        $crate::PropType::Attr($value.to_string())
    };
}

#[macro_export]
macro_rules! cb {
    ($value:expr) => {
        $crate::PropType::Callback($crate::wasm_bindgen::closure::Closure::wrap($value))
    };
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;
    use web_sys::{Event, MouseEvent};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_elt_basic_element() {
        let element = elt!("div", { id: attr!("root") });
        assert_eq!(element.tag_name().to_lowercase(), "div");
    }

    #[wasm_bindgen_test]
    fn test_elt_with_text_content() {
        let element = elt!("p", text!("Hello, world!"));
        assert_eq!(element.tag_name().to_lowercase(), "p");
        assert_eq!(element.text_content().unwrap(), "Hello, world!");
    }

    #[wasm_bindgen_test]
    fn test_elt_with_attributes() {
        let element = elt!(
            "a",
            { href: attr!("#"), class: attr!("link") },
            text!("https://c12i.xyz")
        );
        assert_eq!(element.tag_name().to_lowercase(), "a");
        assert_eq!(element.get_attribute("href").unwrap(), "#");
        assert_eq!(element.get_attribute("class").unwrap(), "link");
    }

    #[wasm_bindgen_test]
    fn test_elt_with_child_elements() {
        let element = elt!(
            "div",
            elt!("h1", text!("Title")),
            elt!("p", text!("Paragraph"))
        );

        assert_eq!(element.tag_name().to_lowercase(), "div");
        assert_eq!(element.children().length(), 2);
        assert_eq!(
            element
                .first_element_child()
                .unwrap()
                .tag_name()
                .to_lowercase(),
            "h1"
        );
        assert_eq!(
            element
                .last_element_child()
                .unwrap()
                .tag_name()
                .to_lowercase(),
            "p"
        );
    }

    #[wasm_bindgen_test]
    fn test_elt_with_attributes_and_children() {
        let element = elt!("div",
            { id: attr!("container"), class: attr!("wrapper") },
            elt!("span", text!("Hello")),
            text!(" "),
            elt!("span", text!("World"))
        );

        assert_eq!(element.tag_name().to_lowercase(), "div");
        assert_eq!(element.get_attribute("id").unwrap(), "container");
        assert_eq!(element.get_attribute("class").unwrap(), "wrapper");
        assert_eq!(element.children().length(), 2);
        assert_eq!(element.text_content().unwrap(), "Hello World");
    }

    #[wasm_bindgen_test]
    fn test_elt_nested_structure() {
        let element = elt!(
            "div",
            elt!("header", elt!("h1", text!("My Page"))),
            elt!("main", elt!("p", text!("Content goes here"))),
            elt!("footer", elt!("span", text!("Copyright 2023")))
        );

        assert_eq!(element.tag_name().to_lowercase(), "div");
        assert_eq!(element.children().length(), 3);
        assert_eq!(
            element
                .first_element_child()
                .unwrap()
                .tag_name()
                .to_lowercase(),
            "header"
        );
        assert_eq!(
            element
                .last_element_child()
                .unwrap()
                .tag_name()
                .to_lowercase(),
            "footer"
        );

        let main = element.children().item(1).unwrap();
        assert_eq!(main.tag_name().to_lowercase(), "main");
        assert_eq!(
            main.first_element_child().unwrap().text_content().unwrap(),
            "Content goes here"
        );
    }

    #[wasm_bindgen_test]
    fn test_elt_with_event_listener_and_attributes() {
        let click_count = Rc::new(RefCell::new(0));
        let click_count_clone = Rc::clone(&click_count);

        let element = elt!("button",
            {
                onclick: cb!(
                    Box::new(move |_event: Event| {
                        *click_count_clone.borrow_mut() += 1;
                    }) as Box<dyn FnMut(Event)>
                ),
                id: attr!("test-button"),
                class: attr!("btn btn-primary")
            },
            text!("Click me")
        );

        assert_eq!(element.tag_name().to_lowercase(), "button");
        assert_eq!(element.get_attribute("id").unwrap(), "test-button");
        assert_eq!(element.get_attribute("class").unwrap(), "btn btn-primary");
        assert_eq!(element.text_content().unwrap(), "Click me");

        // Simulate a click event
        let click_event = MouseEvent::new("click").unwrap();
        element.dispatch_event(&click_event).unwrap();

        assert_eq!(*click_count.borrow(), 1);
    }

    #[wasm_bindgen_test]
    #[should_panic]
    fn test_elt_error_handling() {
        _ = elt!("invalid-tag");
    }
}
