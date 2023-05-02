#![allow(non_snake_case)]

mod components;

use components::{
    actions
};

use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(
        App,
    );
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            actions::QuickActions {}
        }
    })
}
