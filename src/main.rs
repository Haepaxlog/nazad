#![allow(non_snake_case)]

mod components;

use components::{
    actions,
    user,
};

use dioxus::prelude::*;
use crate::components::user::UserProfile;

fn main() {
    dioxus_web::launch(
        App,
    );
}

fn App(cx: Scope) -> Element {
    let mut user = UserProfile::from_data("../assets/profile.png", "Ian McPerson");

    cx.render(rsx! (
        div {
            actions::QuickActions {},
            user::UserBox {
                user: user,
            },
        }
    ))
}
