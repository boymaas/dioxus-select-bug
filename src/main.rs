#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_web::Config;
use web_sys::window;

pub fn InputSelectTest(cx: Scope) -> Element {
    let selected_option = use_state(&cx, || String::from("option2"));

    cx.render(rsx! {
        select {
            value: "{selected_option}",
            onchange: move |e| selected_option.set(e.value.clone()),

            option { value: "option1", "Option 1" }
            option { value: "option2", "Option 2" }
            option { value: "option3", "Option 3" }
        }

        div {
            "Selected Option: {selected_option}"
        }
    })
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let mut dom = VirtualDom::new(InputSelectTest);
    let _ = dom.rebuild();

    let pre = dioxus_ssr::pre_render(&dom);
    tracing::trace!("{}", pre);

    // set the inner content of main to the pre-rendered content
    window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("main")
        .unwrap()
        .set_inner_html(&pre);

    // now rehydrate
    dioxus_web::launch_with_props(InputSelectTest, (), Config::new().hydrate(true));
}
