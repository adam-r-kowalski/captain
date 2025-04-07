use chrono::Local;
use chrono::Timelike;
use dioxus::{events::Key, prelude::*};
use dioxus_free_icons::{icons::hi_solid_icons as icons, Icon};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::{Config, WindowBuilder};
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_resizable(true)
                    .with_title("Captain"),
            ),
        )
        .launch(App);
}

#[cfg(feature = "web")]
fn main() {
    dioxus::launch(App)
}

#[component]
fn Home(onkeydown: EventHandler<KeyboardEvent>) -> Element {
    let mut prompt = use_signal(String::new);

    // Get the current local time and determine the greeting
    let now = Local::now();
    let current_hour = now.hour(); // Use the Timelike trait method
    let greeting = match current_hour {
        5..=11 => "Good morning",    // 5 AM to 11:59 AM
        12..=17 => "Good afternoon", // 12 PM to 5:59 PM
        _ => "Good evening",         // 6 PM to 4:59 AM
    };

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "h-screen flex flex-col p-2",
            class: "bg-amber-50 text-stone-800",
            class: "dark:bg-stone-900 dark:text-amber-100",
            onkeydown,
            tabindex: 0,
            h1 { class: "flex-1 flex items-center justify-center text-4xl", "{greeting}, Adam!" }
            form {
                class: "rounded-lg p-2 flex flex-col gap-2 shadow-md border",
                class: "bg-white border-stone-200",
                class: "dark:bg-stone-800 dark:border-2 dark:border-stone-700",
                onsubmit: move |e| {
                    e.prevent_default();
                    document::eval(&format!("console.log('{}')", prompt()));
                },
                textarea {
                    class: "resize-none outline-none p-2 rounded bg-transparent",
                    class: "placeholder:text-stone-500",
                    class: "dark:placeholder:text-stone-400",
                    placeholder: "How can I help you today?",
                    value: prompt(),
                    oninput: move |e| prompt.set(e.value()),
                }
                div { class: "flex justify-end ",
                    button {
                        r#type: "submit",
                        disabled: prompt().is_empty(),
                        class: "p-2 rounded-lg transition-colors duration-200 disabled:opacity-50 hover:cursor-pointer disabled:hover:cursor-default",
                        class: "bg-green-600 text-white hover:bg-green-700 disabled:hover:bg-green-600",
                        class: "dark:bg-green-700 dark:text-white dark:hover:bg-green-600 disabled:dark:hover:bg-green-700",
                        Icon { icon: icons::HiArrowUp }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "desktop")]
#[component]
fn App() -> Element {
    use dioxus::desktop::use_window;
    let window = use_window();
    rsx! {
        Home {
            onkeydown: move |e: KeyboardEvent| {
                let modifiers = e.modifiers();
                let key = Key::Character("^".to_string());
                if modifiers.alt() && modifiers.meta() && e.key() == key {
                    window.devtool();
                }
            },
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn App() -> Element {
    rsx! {
        Home { onkeydown: move |_| {} }
    }
}
