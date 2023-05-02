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
    let title_header = "Quick Actions";
    let topics = ["Books", "Videos"];

    cx.render(rsx!(
        div {
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


