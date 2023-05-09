#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

use serde::{Serialize, Deserialize};
use std::fs;

#[derive(PartialEq)]
struct Date {
    day: usize,
    month: usize,
    year: usize,
}

#[derive(PartialEq)]
struct Goal {
    completed: bool,
    title: String,
    date: Date,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props)]
struct UserDiagnostics {
    total_studytime: usize,
    daily_average: usize,
    vocab_size: usize,
    chapters_read: usize,
    books_read: usize,
    videos_watched: usize,
    goals_completed: usize,
}

#[derive(PartialEq, Props)]
struct User {
    picture_path: String,
    name: String,
}

#[derive(PartialEq)]
pub struct UserProfile {
    profile: User,
    diagnostics: UserDiagnostics,
    goals: Vec<Goal>,
    visible_at_startup: bool,
}

struct Visible(bool);

const BUTTON_STYLE: &str = "bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded";

impl UserProfile {
    pub fn from_data(picture_path: &str, user_name: &str) -> UserProfile {
        UserProfile {
            profile: User{
                picture_path: picture_path.to_string(),
                name: user_name.to_string(),
            },
            goals: vec![Goal {
                completed: false,
                title: "".to_string(),
                date: Date {
                    day: 0,
                    month: 0,
                    year: 0,
                },
            }],
            visible_at_startup: true,
            diagnostics: UserDiagnostics {
                total_studytime: 0,
                daily_average: 0,
                vocab_size: 0,
                chapters_read: 0,
                books_read: 0,
                videos_watched: 0,
                goals_completed: 0,
            },
        }
    }


    pub fn generate_diagnostic_data(&mut self, data_path: &str) {
        let raw_diagnostic_data = UserProfile::get_diagnostics(data_path).unwrap();
        let diagnostics = UserProfile::deserialize_diagnostics(&raw_diagnostic_data).unwrap();

        self.diagnostics = UserProfile::process_diagnostics(diagnostics);
    }

    fn get_diagnostics(data_path: &str) -> std::io::Result<String> {
        fs::read_to_string(data_path)
    }

    fn deserialize_diagnostics(diagnostic_data: &String) -> serde_json::Result<UserDiagnostics> {
        serde_json::from_str(diagnostic_data)
    }

    fn process_diagnostics(diagnostics: UserDiagnostics) -> UserDiagnostics {
        // TODO
        diagnostics
    }
}

fn HideUserButton(cx: Scope) -> Element {
    let visible = use_shared_state::<Visible>(cx).unwrap();

    let set_visibility = !visible.read().0;

    cx.render(rsx!(
        button {
            class: BUTTON_STYLE,
            onclick: move |_| {
                visible.write().0 = set_visibility;
                },
            "Hide"
        }
    ))
}

fn ProfileSection(cx:Scope<User>) -> Element {
    cx.render(rsx!(
        div {
            img {
                src: cx.props.picture_path.as_str(),
                alt: "Profile Picture"
            },
            p {
                "{cx.props.name}"
            }
        }
    ))
}

#[inline_props]
fn DataParagraph(cx: Scope, head: String, data: String) -> Element {
    cx.render(rsx!(
        div {
            p {
                class: "text-xs text-gray-500",
                "{head}"
            },
            p {
                "{data}"
            }
        }
    ))
}

fn DiagnosticsSection(cx:Scope<UserDiagnostics>) -> Element {
    cx.render(rsx!(
        div {
            class: "grid grid-rows-2 grid-flow-col gap-4",
            DataParagraph{
                head: "Total Study Time".to_string(),
                data: cx.props.total_studytime.to_string()
            },
            DataParagraph{
                head: "Daily Average".to_string(),
                data: cx.props.daily_average.to_string()
            },
            DataParagraph{
                head: "Vocab Size".to_string(),
                data: cx.props.vocab_size.to_string()
            },
            DataParagraph{
                head: "Chapters Read".to_string(),
                data: cx.props.chapters_read.to_string()
            },
            DataParagraph{
                head: "Books Read".to_string(),
                data: cx.props.books_read.to_string()
            },
            DataParagraph{
                head: "Videos Watched".to_string(),
                data: cx.props.videos_watched.to_string()
            },
            DataParagraph{
                head: "Goals Completed".to_string(),
                data: cx.props.goals_completed.to_string()
            }
        }
    ))
}

#[inline_props]
fn ReadDataSection(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "grid grid-rows 2 grid-flow-col gap-4",
            p {
                "Log Data from File"
            },
            button {
                class: BUTTON_STYLE,
                onclick: move |_| {

                },
                "Read Data"
            }
        }
    ))
}

// Main User Box
#[inline_props]
pub fn UserBox(cx:Scope, user: UserProfile) -> Element {
    use_shared_state_provider(cx, || Visible(user.visible_at_startup));

    let visible = use_shared_state::<Visible>(cx).unwrap();

    if visible.read().0 {
        return cx.render(rsx!(
                div {
                    HideUserButton {

                    },
                    ProfileSection {
                      name: user.profile.name.clone(),
                      picture_path: user.profile.picture_path.clone()
                    },
                    DiagnosticsSection {
                      total_studytime: user.diagnostics.total_studytime,
                      daily_average: user.diagnostics.daily_average,
                      vocab_size: user.diagnostics.vocab_size,
                      chapters_read: user.diagnostics.chapters_read,
                      books_read: user.diagnostics.books_read,
                      videos_watched: user.diagnostics.videos_watched,
                      goals_completed: user.diagnostics.goals_completed
                    },
                    ReadDataSection {
                    }
                }
            ));
    } else {
        cx.render(rsx!(
            div {
                HideUserButton{

                },
            }
        ))
    }
}
