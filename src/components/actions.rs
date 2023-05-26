#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

const BUTTON_STYLE: &str = "bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded";

const TOPICS: [&str; 3] = ["Books", "Video", "Audio"];
const TITLE_HEADER: &str = "Quick Actions";

enum Category {
    Books,
    Video,
    Audio
}

pub struct SelectedCategory {
    curr: Category
}

impl SelectedCategory {
    pub fn new() -> Self {
        Self {
            curr: Category::Books
        }
    }

    fn change_category(&mut self, title: &str) {
        match title {
            "Books" => self.curr = Category::Books,
            "Video" => self.curr = Category::Video,
            "Audio" => self.curr = Category::Audio,
            _ => self.curr = Category::Books
        }
    }
}

#[inline_props]
fn ActionButton<'a>(cx: Scope, title: &'a str) -> Element {
    let category = use_shared_state::<SelectedCategory>(cx).unwrap();

     cx.render(rsx!(
         div {
             button {
                 class: BUTTON_STYLE,
                 onclick: move |_| {
                     category.write().change_category(title);
                 },
                 "{title}"
             }
         }
     ))
 }

pub fn QuickActions(cx: Scope) -> Element {

    cx.render(rsx!(
        div {
            class: "grid gap-2 justify-items-center items-center",
            p {
                TITLE_HEADER
            },
            TOPICS.iter().map(|topic| {
                rsx!(ActionButton {
                    title: topic,
                })
            })
        }
    ))
}


