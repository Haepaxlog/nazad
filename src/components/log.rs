#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
struct Date {
    day: usize,
    month: usize,
    year: usize,
}

const ENTRY_TYPES:  [&str; 6] = ["FlashCard", "Listening", "Reading", "Speaking", "Writing", "Other"];

#[derive(Clone)]
enum EntryType {
    FlashCard,
    Listening,
    Reading,
    Speaking,
    Writing,
    Other
}

struct Entry {
    title: String,
    entry_type: EntryType,
    hours: usize,
    date: Date
}

impl Entry {
    fn from_data(title: String, entry_type: EntryType, hours: usize, date: Date) -> Self {
        Self {
            title,
            entry_type,
            hours,
            date,
        }
    }
}

pub struct LogData {
    entries: Vec<Entry>
}

impl LogData {
    pub fn new() -> Self {
        Self {
            entries: vec![]
        }
    }

    fn add_entry(&mut self, title: String, entry_type: EntryType, hours: usize, date: Date) {
        let entry = Entry::from_data(title, entry_type, hours, date);
        self.entries.push(entry);
    }
}

#[inline_props]
fn LogTitleInput<'a>(cx: Scope, on_input: EventHandler<'a, FormEvent>) -> Element {
    cx.render(rsx!(
        input {
            class: "col-span-2",
            placeholder: " ",
            r#type: "text",
            oninput: move |evt| {
                on_input.call(evt)
            }
        }
    ))
}

#[inline_props]
fn LogTypeSelect<'a>(cx: Scope, on_input: EventHandler<'a, FormEvent>) -> Element {
    cx.render(rsx!(
        select {
             oninput: move |evt| {
                on_input.call(evt)
            },
            ENTRY_TYPES.iter().map(|entry_type| {
                rsx!(
                    option {
                        value: entry_type.clone(),
                        entry_type.clone()
                    })
            })
        }
    ))
}

#[inline_props]
fn LogHoursInput<'a>(cx: Scope, on_input: EventHandler<'a, FormEvent>) -> Element {
    cx.render(rsx!(
        input {
            class: "",
            r#type: "number",
            oninput: move |evt| {
                on_input.call(evt)
            }
        }
    ))
}

#[inline_props]
fn LogDateInput<'a>(cx: Scope, on_input: EventHandler<'a, FormEvent>) -> Element {
    cx.render(rsx!(
        input {
            r#type: "date",
            oninput: move |evt| {
                on_input.call(evt)
            }
        }
    ))
}

pub fn Logger(cx: Scope) -> Element {
    let title = use_state(cx, || String::from(""));
    let entry_type = use_state(cx, ||  EntryType::Other);
    let hours = use_state(cx, || 0usize);
    let date = use_state(cx, || Date{
        day: 0,
        month: 0,
        year: 0
    });

    cx.render(rsx!(
        div {
            class: "grid grid-cols-4 gap-2 bg-zinc-600 m-4 p-2 rounded-md border-8 border-zinc-600",
            LogTitleInput {
                on_input: move |event: FormEvent| {
                    title.set(event.value.clone());
                }
            },
            LogTypeSelect {
                on_input: move |event: FormEvent| {
                    let select = match event.value.as_str() {
                        "FlashCard" => EntryType::FlashCard,
                        "Listening" => EntryType::Listening,
                        "Reading" => EntryType::Reading,
                        "Speaking" => EntryType::Speaking,
                        "Writing" => EntryType::Writing,
                        _ => EntryType::Other
                    };
                    entry_type.set(select);
                }
            },
            LogHoursInput {
                on_input: move |event: FormEvent| {
                    hours.set(match event.value.parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => 0
                    });
                }
            },
            LogDateInput {
                on_input: move |event: FormEvent| {

                }
            },
            button {
                class: "col-span-1 col-end-5 bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded",
                onclick: move |_| {
                    let log_data = use_shared_state::<LogData>(cx).unwrap();
                    log_data.write().add_entry(title.get().to_string(), entry_type.get().clone(), *hours.get(), date.get().clone());
                },
                "Add Log"
            }
        }
    ))
}