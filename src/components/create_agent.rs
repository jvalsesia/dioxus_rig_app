use crate::components::create_agent_modal::PersonalitySlider;
use crate::{Agent, Personality, Route, Skill};
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};
use std::collections::HashSet;

#[component]
pub fn CreateAgent() -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();
    let navigator = use_navigator();
    let i18n = i18n();
    let lang_str = i18n.language().to_string();

    let skills_resource = use_resource(move || {
        let l = lang_str.clone();
        async move { crate::server_fns::get_skills(l).await.unwrap_or_default() }
    });

    let mut new_name = use_signal(String::new);
    let mut new_specialty = use_signal(String::new);
    let mut selected = use_signal(HashSet::<String>::new);
    let mut is_loading = use_signal(|| false);
    let mut personality = use_signal(Personality::default);

    let input_cls = "w-full bg-transparent border-0 border-b border-zinc-300 dark:border-zinc-700 px-0 py-3 text-zinc-900 dark:text-zinc-100 text-lg placeholder:text-zinc-400 dark:placeholder:text-zinc-600 focus:outline-none focus:border-zinc-900 dark:focus:border-zinc-100 transition-colors font-display";

    let textarea_cls = "w-full bg-zinc-50/60 dark:bg-zinc-900/40 border border-zinc-200 dark:border-zinc-800 rounded-xl px-4 py-3 text-zinc-900 dark:text-zinc-100 text-sm placeholder:text-zinc-400 dark:placeholder:text-zinc-600 focus:outline-none focus:border-zinc-400 dark:focus:border-zinc-600 resize-none transition-colors";

    let create_agent = move |_| {
        let name = new_name.read().clone();
        let specialty = new_specialty.read().clone();
        if name.trim().is_empty() || specialty.trim().is_empty() {
            return;
        }
        let skill_ids: Vec<String> = selected.read().iter().cloned().collect();
        let p = *personality.read();
        *is_loading.write() = true;
        spawn(async move {
            match crate::server_fns::add_agent(name, specialty, p).await {
                Ok(agent) => {
                    if !skill_ids.is_empty() {
                        let _ = crate::server_fns::set_agent_skills(agent.id, skill_ids).await;
                    }
                    agents_resource.restart();
                    *is_loading.write() = false;
                    navigator.push(Route::AgentList {});
                }
                Err(_) => *is_loading.write() = false,
            }
        });
    };

    let skills_opt = skills_resource.read_unchecked();
    let skills: Vec<Skill> = skills_opt.clone().unwrap_or_default();

    rsx! {
        div { class: "flex-1 overflow-y-auto",
            div { class: "max-w-5xl mx-auto px-10 py-12",

                // ── Page head ─────────────────────────────────────────────────
                header { class: "mb-12 reveal",
                    div { class: "flex items-baseline gap-3 mb-2",
                        span { class: "font-mono text-[11px] tracking-[0.28em] uppercase text-amber-600 dark:text-amber-400",
                            "New entry · 01 / 03"
                        }
                        span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
                        Link {
                            to: Route::AgentList {},
                            class: "font-mono text-[11px] tracking-[0.22em] uppercase text-zinc-500 hover:text-zinc-900 dark:hover:text-zinc-100 no-underline",
                            "← back"
                        }
                    }
                    h1 { class: "font-display text-5xl text-zinc-900 dark:text-zinc-100 leading-[0.95]",
                        {t!("create-page-title")}
                    }
                    p { class: "mt-3 font-display italic text-base text-zinc-500 dark:text-zinc-400 max-w-2xl",
                        {t!("create-page-subtitle")}
                    }
                }

                // ── 01 · Identity ─────────────────────────────────────────────
                Section {
                    index: "01",
                    eyebrow: "Identity",
                    title: t!("create-modal-title"),
                    div { class: "flex flex-col gap-6",
                        div { class: "flex flex-col gap-2",
                            label { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-500", {t!("create-modal-name")} }
                            input {
                                class: "{input_cls}",
                                placeholder: t!("create-modal-name-placeholder"),
                                value: "{new_name}",
                                oninput: move |evt| *new_name.write() = evt.value(),
                            }
                        }
                        div { class: "flex flex-col gap-2",
                            label { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-500", {t!("create-modal-specialty")} }
                            textarea {
                                class: "{textarea_cls}",
                                placeholder: t!("create-modal-specialty-placeholder"),
                                value: "{new_specialty}",
                                oninput: move |evt| *new_specialty.write() = evt.value(),
                                rows: 5,
                            }
                        }
                    }
                }

                // ── 02 · Personality + 03 · Skills (two columns) ──────────────
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6",

                    Section {
                        index: "02",
                        eyebrow: "Personality",
                        title: t!("create-modal-personality"),
                        p { class: "font-display italic text-sm text-zinc-500 dark:text-zinc-500 mb-4",
                            {t!("create-modal-personality-hint")}
                        }
                        div { class: "flex flex-col gap-5",
                            PersonalitySlider { label: t!("trait-openness"),             low: t!("trait-openness-low"),             high: t!("trait-openness-high"),             value: personality.read().openness,             on_change: move |v| personality.write().openness = v }
                            PersonalitySlider { label: t!("trait-conscientiousness"),    low: t!("trait-conscientiousness-low"),    high: t!("trait-conscientiousness-high"),    value: personality.read().conscientiousness,    on_change: move |v| personality.write().conscientiousness = v }
                            PersonalitySlider { label: t!("trait-extraversion"),         low: t!("trait-extraversion-low"),         high: t!("trait-extraversion-high"),         value: personality.read().extraversion,         on_change: move |v| personality.write().extraversion = v }
                            PersonalitySlider { label: t!("trait-agreeableness"),        low: t!("trait-agreeableness-low"),        high: t!("trait-agreeableness-high"),        value: personality.read().agreeableness,        on_change: move |v| personality.write().agreeableness = v }
                            PersonalitySlider { label: t!("trait-emotional-stability"),  low: t!("trait-emotional-stability-low"),  high: t!("trait-emotional-stability-high"),  value: personality.read().emotional_stability,  on_change: move |v| personality.write().emotional_stability = v }
                        }
                    }

                    Section {
                        index: "03",
                        eyebrow: "Skills",
                        title: t!("create-modal-skills"),
                        if skills.is_empty() {
                            p { class: "font-display italic text-sm text-zinc-500", {t!("create-modal-skills-empty")} }
                        } else {
                            ul { class: "chat-scroll flex flex-col gap-2 max-h-[26rem] overflow-y-auto pr-2",
                                for skill in skills.iter() {
                                    {
                                        let sid = skill.id.clone();
                                        let sid_click = sid.clone();
                                        let checked = selected.read().contains(&sid);
                                        rsx! {
                                            li {
                                                key: "{skill.id}",
                                                class: if checked {
                                                    "group flex items-start gap-3 p-3 rounded-xl border border-zinc-900 dark:border-zinc-100 bg-zinc-50 dark:bg-zinc-900 transition-colors cursor-pointer"
                                                } else {
                                                    "group flex items-start gap-3 p-3 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:border-zinc-400 dark:hover:border-zinc-600 transition-colors cursor-pointer"
                                                },
                                                onclick: move |_| {
                                                    let sid = sid_click.clone();
                                                    let mut s = selected.write();
                                                    if s.contains(&sid) { s.remove(&sid); } else { s.insert(sid); }
                                                },
                                                span { class: if checked {
                                                    "shrink-0 w-5 h-5 rounded border border-zinc-900 dark:border-zinc-100 bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 font-mono text-[11px] flex items-center justify-center mt-0.5"
                                                } else {
                                                    "shrink-0 w-5 h-5 rounded border border-zinc-300 dark:border-zinc-700 mt-0.5"
                                                },
                                                    if checked { "✓" } else { "" }
                                                }
                                                div { class: "flex-1 min-w-0",
                                                    p { class: "text-sm font-medium text-zinc-900 dark:text-zinc-100 font-display", "{skill.name}" }
                                                    p { class: "text-xs text-zinc-500 mt-0.5", "{skill.description}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Action bar ───────────────────────────────────────────────
                div { class: "mt-10 flex items-center justify-between gap-4",
                    span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-400 dark:text-zinc-600",
                        "End of entry"
                    }
                    div { class: "flex gap-3",
                        Link {
                            to: Route::AgentList {},
                            class: "px-6 py-3 rounded-lg bg-transparent border border-zinc-200 dark:border-zinc-800 text-zinc-600 dark:text-zinc-300 font-mono text-[11px] tracking-[0.22em] uppercase hover:border-zinc-400 dark:hover:border-zinc-600 transition-colors no-underline",
                            {t!("manage-edit-cancel")}
                        }
                        button {
                            class: "px-8 py-3 rounded-lg bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 font-mono text-[11px] tracking-[0.22em] uppercase hover:opacity-90 transition-opacity disabled:opacity-50",
                            onclick: create_agent,
                            disabled: *is_loading.read(),
                            if *is_loading.read() { {t!("create-modal-creating")} } else { {t!("create-modal-submit")} }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SectionProps {
    index: &'static str,
    eyebrow: &'static str,
    title: String,
    children: Element,
}

#[component]
fn Section(props: SectionProps) -> Element {
    rsx! {
        section { class: "bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-7 reveal",
            div { class: "flex items-baseline gap-3 mb-5",
                span { class: "font-mono text-2xl text-amber-600 dark:text-amber-400 leading-none",
                    "{props.index}"
                }
                span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-400 dark:text-zinc-600",
                    "{props.eyebrow}"
                }
                span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
            }
            h2 { class: "font-display text-2xl text-zinc-900 dark:text-zinc-100 mb-5",
                "{props.title}"
            }
            {props.children}
        }
    }
}
