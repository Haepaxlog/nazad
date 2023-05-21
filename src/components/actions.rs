#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

#[inline_props]
fn ActionButton<'a>(cx: Scope, title: &'a str) -> Element {
     cx.render(rsx!(
         div {
             p {
                 "{title}"
             }
         }
     ))
 }

pub fn QuickActions(cx: Scope) -> Element {
    let title_header: &str = "Quick Actions";
    let topics: [&str; 3] = ["Books", "Video", "Audio"];

    cx.render(rsx!(
        div {
            class: "grid gap-2 justify-items-center items-center",
            p {
                title_header
            },
            topics.iter().map(|topic| {
                rsx!(ActionButton {
                    title: topic,
                })
            })
        }
    ))
}


