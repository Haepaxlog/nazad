#![allow(non_snake_case)]

mod components;

use components::{
    actions,
    user,
    dashboard,
    log
};

use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(
        App,
    );
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || user::UserProfile::new());
    use_shared_state_provider(cx, || actions::SelectedCategory::new());
    use_shared_state_provider(cx, || log::LogData::new());

    cx.render(rsx! (
        div {
            class: "grid grid-cols-6 h-[100vh]",
            actions::QuickActions {},
            dashboard::View {},
            user::UserBox {}
        }
    ))
}
