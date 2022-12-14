#[allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Main(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            id: "main",
            link {
                href: "main.css",
            }
        }
    })
}
