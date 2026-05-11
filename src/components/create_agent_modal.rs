use dioxus::prelude::*;
use dioxus_i18n::{t, prelude::*};
use std::collections::HashSet;
use crate::{Agent, Personality, Skill};

#[derive(Props, Clone, PartialEq)]
pub struct CreateAgentModalProps {
    pub onclose: EventHandler<()>,
}

#[component]
pub fn CreateAgentModal(props: CreateAgentModalProps) -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();
    let i18n = i18n();
    let lang_str = i18n.language().to_string();

    let skills_resource = use_resource(move || {
        let l = lang_str.clone();
        async move {
            crate::server_fns::get_skills(l).await.unwrap_or_default()
        }
    });

    let mut new_name = use_signal(String::new);
    let mut new_specialty = use_signal(String::new);
    let mut selected = use_signal(HashSet::<String>::new);
    let mut is_loading = use_signal(|| false);
    let mut personality = use_signal(Personality::default);

    let input_cls = "w-full bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 px-4 py-3 rounded-xl text-zinc-900 dark:text-zinc-100 text-sm placeholder:text-zinc-400 dark:placeholder:text-zinc-500 focus:outline-none focus:border-violet-400 dark:focus:border-violet-500/50 focus:ring-2 focus:ring-violet-200 dark:focus:ring-violet-500/20 transition-all";

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
                    props.onclose.call(());
                }
                Err(_) => {
                    *is_loading.write() = false;
                }
            }
        });
    };

    let skills_opt = skills_resource.read_unchecked();
    let skills: Vec<Skill> = skills_opt.clone().unwrap_or_default();

    rsx! {
        div { class: "fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50",
            div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-6 w-full max-w-2xl mx-4 flex flex-col gap-5 shadow-2xl max-h-[90vh]",

                // Header
                div { class: "flex items-center justify-between shrink-0",
                    h3 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100", {t!("create-modal-title")} }
                    button {
                        class: "text-zinc-400 hover:text-zinc-700 dark:hover:text-zinc-200 text-xl cursor-pointer transition-colors bg-transparent border-none leading-none",
                        onclick: move |_| props.onclose.call(()),
                        "✕"
                    }
                }

                // Name field
                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-name")} }
                    input {
                        class: "{input_cls}",
                        placeholder: t!("create-modal-name-placeholder"),
                        value: "{new_name}",
                        oninput: move |evt| *new_name.write() = evt.value()
                    }
                }

                // Specialty field
                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-specialty")} }
                    textarea {
                        class: "{input_cls} resize-none",
                        placeholder: t!("create-modal-specialty-placeholder"),
                        value: "{new_specialty}",
                        oninput: move |evt| *new_specialty.write() = evt.value(),
                        rows: 3
                    }
                }

                // Personality (left) + Skills (right)
                div { class: "grid grid-cols-2 gap-4 min-h-0",

                // Personality (Five-Factor) sliders
                div { class: "flex flex-col gap-2 min-w-0",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-personality")} }
                    p { class: "text-[11px] text-zinc-500 -mt-1", {t!("create-modal-personality-hint")} }
                    div { class: "chat-scroll flex flex-col gap-2 max-h-56 overflow-y-auto pr-2 rounded-lg border border-zinc-200 dark:border-zinc-800 p-2",
                        PersonalitySlider { label: t!("trait-openness"), low: t!("trait-openness-low"), high: t!("trait-openness-high"), value: personality.read().openness, on_change: move |v| personality.write().openness = v }
                        PersonalitySlider { label: t!("trait-conscientiousness"), low: t!("trait-conscientiousness-low"), high: t!("trait-conscientiousness-high"), value: personality.read().conscientiousness, on_change: move |v| personality.write().conscientiousness = v }
                        PersonalitySlider { label: t!("trait-extraversion"), low: t!("trait-extraversion-low"), high: t!("trait-extraversion-high"), value: personality.read().extraversion, on_change: move |v| personality.write().extraversion = v }
                        PersonalitySlider { label: t!("trait-agreeableness"), low: t!("trait-agreeableness-low"), high: t!("trait-agreeableness-high"), value: personality.read().agreeableness, on_change: move |v| personality.write().agreeableness = v }
                        PersonalitySlider { label: t!("trait-emotional-stability"), low: t!("trait-emotional-stability-low"), high: t!("trait-emotional-stability-high"), value: personality.read().emotional_stability, on_change: move |v| personality.write().emotional_stability = v }
                    }
                }

                // Skills picker
                div { class: "flex flex-col gap-2 min-w-0 min-h-0",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-skills")} }
                    if skills.is_empty() {
                        p { class: "text-xs text-zinc-500", {t!("create-modal-skills-empty")} }
                    } else {
                        ul { class: "chat-scroll flex flex-col gap-1.5 h-56 overflow-y-auto pr-2 rounded-lg border border-zinc-200 dark:border-zinc-800 p-2",
                            for skill in skills.iter() {
                                {
                                    let sid = skill.id.clone();
                                    let sid_click = sid.clone();
                                    let checked = selected.read().contains(&sid);
                                    rsx! {
                                        li {
                                            key: "{skill.id}",
                                            class: "flex items-start gap-2 p-2 rounded-lg border border-zinc-200 dark:border-zinc-800 hover:border-violet-300 dark:hover:border-violet-500/50 transition-all cursor-pointer",
                                            onclick: move |_| {
                                                let sid = sid_click.clone();
                                                let mut s = selected.write();
                                                if s.contains(&sid) {
                                                    s.remove(&sid);
                                                } else {
                                                    s.insert(sid);
                                                }
                                            },
                                            input {
                                                r#type: "checkbox",
                                                checked: checked,
                                                class: "mt-0.5 accent-violet-600 pointer-events-none",
                                                onchange: move |_| {},
                                            }
                                            div { class: "flex-1 min-w-0",
                                                p { class: "text-xs font-semibold text-zinc-900 dark:text-zinc-100", "{skill.name}" }
                                                p { class: "text-[11px] text-zinc-500 mt-0.5 line-clamp-2", "{skill.description}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                }

                // Submit
                button {
                    class: "w-full bg-violet-600 text-white py-3 rounded-xl font-semibold text-sm hover:bg-violet-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed shrink-0",
                    onclick: create_agent,
                    disabled: *is_loading.read(),
                    if *is_loading.read() { {t!("create-modal-creating")} } else { {t!("create-modal-submit")} }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PersonalitySliderProps {
    pub label: String,
    pub low: String,
    pub high: String,
    pub value: f32,
    pub on_change: EventHandler<f32>,
}

#[component]
pub fn PersonalitySlider(props: PersonalitySliderProps) -> Element {
    let pct = (props.value * 100.0).round() as i32;
    let initial = props.label.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
    let style = format!("--p: {pct}%;");
    rsx! {
        div { class: "flex flex-col gap-1.5",
            div { class: "flex items-center gap-3",
                span { class: "shrink-0 w-7 h-7 rounded-md border border-amber-300/70 dark:border-amber-400/40 bg-amber-50 dark:bg-amber-400/10 text-amber-700 dark:text-amber-300 font-mono text-[11px] flex items-center justify-center",
                    "{initial}"
                }
                span { class: "flex-1 text-sm font-medium text-zinc-800 dark:text-zinc-200 truncate",
                    title: "{props.low} ↔ {props.high}",
                    "{props.label}"
                }
                span { class: "font-mono text-[11px] tabular-nums text-zinc-500 dark:text-zinc-400",
                    "{pct}"
                }
            }
            div { class: "pl-10 flex flex-col gap-1",
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    step: "1",
                    value: "{pct}",
                    style: "{style}",
                    class: "slider-fine w-full cursor-pointer",
                    oninput: move |evt| {
                        if let Ok(v) = evt.value().parse::<f32>() {
                            props.on_change.call(v / 100.0);
                        }
                    },
                }
                div { class: "flex items-center justify-between font-mono text-[9px] tracking-wider uppercase text-zinc-400 dark:text-zinc-600",
                    span { "{props.low}" }
                    span { "{props.high}" }
                }
            }
        }
    }
}
