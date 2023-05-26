#![allow(non_snake_case)]

use dioxus::prelude::*;
use chrono::{DateTime, Utc};

use crate::components::log;

const TITLE: &str = "Nazad";

fn Header(cx: Scope) -> Element {
    let now: DateTime<Utc> = Utc::now();
    let time = format!("{}",now.format("%a %b %e %Y"));

    cx.render(rsx!(
        div {
            class: "flex flex-row",
            span {
                class: "ml-40",
                TITLE
            },
            span {
                class: "ml-auto mr-2",
                time
            }
        },
        hr {

        }
    ))
}

pub fn View(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "col-span-4",
            Header {},
            log::Logger {}
        }
    ))
}