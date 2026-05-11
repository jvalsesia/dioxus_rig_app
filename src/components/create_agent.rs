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
        async move {
            crate::server_fns::get_skills(l)
                .await
                .unwrap_or_default()
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
                    navigator.push(Route::AgentList {});
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
        div { class: "flex-1 overflow-y-auto p-8",
            div { class: "max-w-5xl mx-auto",

                // Header
                header { class: "mb-6 flex items-start justify-between gap-4",
                    div {
                        h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100", {t!("create-page-title")} }
                        p { class: "text-sm text-zinc-500 mt-1 max-w-2xl", {t!("create-page-subtitle")} }
                    }
                    Link {
                        to: Route::AgentList {},
                        class: "shrink-0 text-sm text-violet-600 dark:text-violet-400 font-medium hover:text-violet-500 dark:hover:text-violet-300 no-underline",
                        {t!("manage-back")}
                    }
                }

                // Card: name + specialty
                div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-6 flex flex-col gap-5 shadow-sm mb-6",
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-name")} }
                        input {
                            class: "{input_cls}",
                            placeholder: t!("create-modal-name-placeholder"),
                            value: "{new_name}",
                            oninput: move |evt| *new_name.write() = evt.value(),
                        }
                    }
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-specialty")} }
                        textarea {
                            class: "{input_cls} resize-none",
                            placeholder: t!("create-modal-specialty-placeholder"),
                            value: "{new_specialty}",
                            oninput: move |evt| *new_specialty.write() = evt.value(),
                            rows: 4,
                        }
                    }
                }

                // Two-column: Personality (left) + Skills (right)
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6",

                    // Personality
                    div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-6 flex flex-col gap-3 shadow-sm min-w-0",
                        label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-personality")} }
                        p { class: "text-[11px] text-zinc-500 -mt-2", {t!("create-modal-personality-hint")} }
                        div { class: "flex flex-col gap-3",
                            PersonalitySlider { label: t!("trait-openness"), low: t!("trait-openness-low"), high: t!("trait-openness-high"), value: personality.read().openness, on_change: move |v| personality.write().openness = v }
                            PersonalitySlider { label: t!("trait-conscientiousness"), low: t!("trait-conscientiousness-low"), high: t!("trait-conscientiousness-high"), value: personality.read().conscientiousness, on_change: move |v| personality.write().conscientiousness = v }
                            PersonalitySlider { label: t!("trait-extraversion"), low: t!("trait-extraversion-low"), high: t!("trait-extraversion-high"), value: personality.read().extraversion, on_change: move |v| personality.write().extraversion = v }
                            PersonalitySlider { label: t!("trait-agreeableness"), low: t!("trait-agreeableness-low"), high: t!("trait-agreeableness-high"), value: personality.read().agreeableness, on_change: move |v| personality.write().agreeableness = v }
                            PersonalitySlider { label: t!("trait-emotional-stability"), low: t!("trait-emotional-stability-low"), high: t!("trait-emotional-stability-high"), value: personality.read().emotional_stability, on_change: move |v| personality.write().emotional_stability = v }
                        }
                    }

                    // Skills
                    div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-6 flex flex-col gap-3 shadow-sm min-w-0 min-h-0",
                        label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("create-modal-skills")} }
                        if skills.is_empty() {
                            p { class: "text-xs text-zinc-500", {t!("create-modal-skills-empty")} }
                        } else {
                            ul { class: "chat-scroll flex flex-col gap-2 max-h-96 overflow-y-auto pr-2",
                                for skill in skills.iter() {
                                    {
                                        let sid = skill.id.clone();
                                        let sid_click = sid.clone();
                                        let checked = selected.read().contains(&sid);
                                        rsx! {
                                            li {
                                                key: "{skill.id}",
                                                class: "flex items-start gap-3 p-3 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:border-violet-300 dark:hover:border-violet-500/50 transition-all cursor-pointer",
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
                                                    p { class: "text-sm font-semibold text-zinc-900 dark:text-zinc-100", "{skill.name}" }
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

                // Action bar
                div { class: "flex gap-3 justify-end",
                    Link {
                        to: Route::AgentList {},
                        class: "bg-transparent text-zinc-500 border border-zinc-200 dark:border-zinc-700 py-2.5 px-6 rounded-xl font-semibold text-sm hover:bg-zinc-100 dark:hover:bg-zinc-800 hover:text-zinc-700 dark:hover:text-zinc-200 transition-all no-underline",
                        {t!("manage-edit-cancel")}
                    }
                    button {
                        class: "bg-violet-600 text-white py-2.5 px-8 rounded-xl font-semibold text-sm hover:bg-violet-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed",
                        onclick: create_agent,
                        disabled: *is_loading.read(),
                        if *is_loading.read() { {t!("create-modal-creating")} } else { {t!("create-modal-submit")} }
                    }
                }
            }
        }
    }
}
