#![allow(non_snake_case)]

mod components;

use components::{
    actions,
    user,
};

use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(
        App,
    );
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "grid grid-cols-2 h-[100vh]",
            actions::QuickActions {},
            user::UserBox {},
        }
    ))
}
