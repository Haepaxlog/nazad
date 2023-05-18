#![allow(non_snake_case)]
#![allow(dead_code)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

use serde::{Serialize, Deserialize};
use std::{
    fs,
};

#[derive(PartialEq, Clone)]
struct Date {
    day: usize,
    month: usize,
    year: usize,
}

#[derive(PartialEq, Clone)]
struct Goal {
    completed: bool,
    title: String,
    date: Date,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Props)]
struct UserDiagnostics {
    total_studytime: usize,
    daily_average: usize,
    vocab_size: usize,
    chapters_read: usize,
    books_read: usize,
    videos_watched: usize,
    goals_completed: usize,
}

#[derive(PartialEq, Clone, Props)]
struct User {
    picture_path: String,
    name: String,
}

#[derive(PartialEq, Clone)]
pub struct UserProfile {
    profile: User,
    diagnostics: UserDiagnostics,
    goals: Vec<Goal>,
    visible_at_startup: bool,
}

struct Visible(bool);
struct Clicked(bool);
struct Edit(bool);

const BUTTON_STYLE: &str = "bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded";
const NUMBER_INPUT_STYLE: &str = "peer h-full w-full rounded-[7px] border border-blue-gray-200 border-t-transparent bg-transparent px-3 py-2.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border placeholder-shown:border-blue-gray-200 placeholder-shown:border-t-blue-gray-200 focus:border-2 focus:border-pink-500 focus:border-t-transparent focus:outline-0 disabled:border-0 disabled:bg-blue-gray-50";
const NUMBER_LABEL_STYLE: &str = "before:content[' '] after:content[' '] pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-[11px] font-normal leading-tight text-blue-gray-400 transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-blue-gray-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-blue-gray-200 after:transition-all peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[3.75] peer-placeholder-shown:text-blue-gray-500 peer-placeholder-shown:before:border-transparent peer-placeholder-shown:after:border-transparent peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-pink-500 peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:border-pink-500 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:border-pink-500 peer-disabled:text-transparent peer-disabled:before:border-transparent peer-disabled:after:border-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500";

impl UserProfile {
    pub fn new() -> Self {
        Self {
            profile: User{
                picture_path: String::new(),
                name: String::new(),
            },
            goals: vec![Goal {
                completed: false,
                title: String::new(),
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

    pub fn with_data(&mut self, picture_path: &str, user_name: &str) {
        self.profile.picture_path = picture_path.to_string();
        self.profile.name = user_name.to_string();
    }

    pub fn test(&mut self) {
        self.diagnostics.goals_completed += 1;

        if !self.profile.name.eq("changed") {
            self.profile.name = String::from("changed");
        } else {
            self.profile.name = String::from( self.goals[0].clone().date.year.to_string());
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
        diagnostics
    }

    fn mark_goal_as_completed(&mut self, target_goal: &Goal) {
        for (i, goal) in self.goals.iter().enumerate() {
            if goal == target_goal {
                self.goals[i].completed = !self.goals[i].completed;
                return;
            }
        }
    }

    fn remove_goal(&mut self, target_goal: &Goal) {
        for (i, goal) in self.goals.iter().enumerate() {
            if goal == target_goal {
                self.goals.remove(i);
                return;
            }
        }
    }

    fn update_goal(&mut self, target_goal: &Goal, new_goal: Goal) {
        for (i, goal) in self.goals.iter().enumerate() {
            if goal == target_goal {
                self.goals[i] = new_goal;
                return;
            }
        }
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

fn ProfileSection(cx:Scope) -> Element {
    let user = use_shared_state::<UserProfile>(cx).unwrap();
    let profile = user.read().clone().profile;

    cx.render(rsx!(
        div {
            img {
                src: "{profile.picture_path}",
                alt: "Profile Picture"
            },
            p {
                profile.name
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

fn DiagnosticsSection(cx:Scope) -> Element {
    let user = use_shared_state::<UserProfile>(cx).unwrap();
    let diagnostics = user.read().clone().diagnostics;

    cx.render(rsx!(
        div {
            class: "grid grid-rows-2 grid-flow-col gap-4",
            DataParagraph{
                head: "Total Study Time".to_string(),
                data: diagnostics.total_studytime.to_string()
            },
            DataParagraph{
                head: "Daily Average".to_string(),
                data: diagnostics.daily_average.to_string()
            },
            DataParagraph{
                head: "Vocab Size".to_string(),
                data: diagnostics.vocab_size.to_string()
            },
            DataParagraph{
                head: "Chapters Read".to_string(),
                data: diagnostics.chapters_read.to_string()
            },
            DataParagraph{
                head: "Books Read".to_string(),
                data: diagnostics.books_read.to_string()
            },
            DataParagraph{
                head: "Videos Watched".to_string(),
                data: diagnostics.videos_watched.to_string()
            },
            DataParagraph{
                head: "Goals Completed".to_string(),
                data: diagnostics.goals_completed.to_string()
            }
        }
    ))
}

fn ReadDataSection(cx: Scope) -> Element {
    let user = use_shared_state::<UserProfile>(cx).unwrap();

    cx.render(rsx!(
        div {
            class: "grid grid-rows 2 grid-flow-col gap-4",
            p {
                "Log Data from File"
            },
            button {
                class: BUTTON_STYLE,
                onclick: move |_| {
                   user.write().test()
                },
                "Read Data"
            }
        }
    ))
}

#[inline_props]
fn NumberInput<'a>(cx: Scope, value: String, kind: String, on_input: EventHandler<'a, FormEvent>) -> Element {
    cx.render(rsx!(
        div {
            class: "w-72",
            div {
                class: "relative h-10 w-full min-w-[200px]",
                input {
                    class: NUMBER_INPUT_STYLE,
                    placeholder: " ",
                    r#type: "number",
                    min: "0",
                    max: "10000",
                    value: "{value}",
                    oninput: move |evt| {
                        on_input.call(evt)
                    }
                },
                label {
                    class: NUMBER_LABEL_STYLE,
                    "{kind}"
                }
            }
        }
    ))
}

#[inline_props]
fn GoalParagraph(cx: Scope, goal: Goal) -> Element {
    let user = use_shared_state::<UserProfile>(cx).unwrap();

    let clicked = use_state(cx, || Clicked(false));
    let edit = use_state(cx, || Edit(false));

    let title = &goal.title;
    let date = &goal.date;

    let day = use_state(cx, || date.day.to_string());
    let month = use_state(cx, || date.month.to_string());
    let year = use_state(cx, || date.year.to_string());

    if clicked.0 {
        user.write().remove_goal(&goal);
    }

    if !edit.0 {
    return cx.render(rsx!(
       div {
            class: "flex flex-row gap-2 group-checked:opacity-0",
            input {
                id: "{title}",
                class: "checked:text-blue-600 transition-colors will-change-auto",
                r#type: "checkbox",
                name: "{title}",
                onchange: move |_| {
                    user.write().mark_goal_as_completed(goal);
                }
            },
            label {
                    class: "self-center",
                    r#for: "{title}",
                div {
                    class: "flex flex-row gap-2",
                    span {
                    "{title}"
                },
                span {
                    "{day}"
                },
                span {
                    "{month}"
                },
                span {
                    "{year}"
                }
                },

                },
            button {
                        class: BUTTON_STYLE,
                        onclick: move |_| {
                            edit.set(Edit(true));
                        },
                "Edit"
            },
            button {
                class: BUTTON_STYLE,
                onclick: move |_| {
                    clicked.set(Clicked(true));
                },
                "Delete"
            }

        }
    )); }

    cx.render(rsx!(
         div {
            class: "flex flex-row gap-2 group-checked:opacity-0",
            input {
                id: "{title}",
                class: "checked:opacity-0 transition-opacity will-change-auto",
                r#type: "checkbox",
                disabled: "disabled",
                name: "{title}",
            },
            label {
                    class: "self-center",
                    r#for: "{title}",
                div {
                    class: "flex flex-row gap-2",
                    span {
                    "{title}"
                },
                 NumberInput {
                        value: day.to_string(),
                        kind: "Day".to_string(),
                        on_input:  move |event: FormEvent| {
                            day.set(event.value.clone());
                        }
                },
                NumberInput {
                        value: month.to_string(),
                        kind: "Month".to_string(),
                        on_input: move |event: FormEvent| {
                            month.set(event.value.clone());
                        }
                },
                NumberInput {
                        value: year.to_string(),
                        kind: "Year".to_string(),
                        on_input: move |event: FormEvent| {
                            year.set(event.value.clone());
                        }
                }
                },
            },
              button {
                        class: BUTTON_STYLE,
                        onclick: move |_| {
                            let day = day.parse::<usize>().unwrap();
                            let month = month.parse::<usize>().unwrap();
                            let year = year.parse::<usize>().unwrap();

                            user.write().update_goal(goal, Goal{
                            completed: false,
                            title: goal.title.clone(),
                            date: Date{day,month,year}
                    });
                            edit.set(Edit(false));
                        },
                "Edit"
            }
        }
    ))
}

fn GoalsSection(cx: Scope) -> Element {
    let user = use_shared_state::<UserProfile>(cx).unwrap();
    let goals = user.read().clone().goals;

    cx.render(rsx!(
        div {
            goals.iter().map(|goal| rsx!(
                GoalParagraph {
                    goal: goal.clone()
                }
            ))
        }
    ))
}

// Main User Box
pub fn UserBox(cx:Scope) -> Element {
    use_shared_state_provider(cx, || UserProfile::new());
    let user = use_shared_state::<UserProfile>(cx).unwrap();

    use_shared_state_provider(cx, || Visible(user.read().visible_at_startup));
    let visible = use_shared_state::<Visible>(cx).unwrap();

    if visible.read().0 {
        return cx.render(rsx!(
                div {
                    HideUserButton {},
                    ProfileSection {},
                    DiagnosticsSection {},
                    ReadDataSection {},
                    GoalsSection {}
                }
            ));
    } else {
        cx.render(rsx!(
            div {
                HideUserButton {},
            }
        ))
    }
}
