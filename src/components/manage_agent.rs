use crate::components::create_agent_modal::PersonalitySlider;
use crate::{Agent, Personality, Route, Skill};
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};
use std::collections::HashSet;

#[component]
pub fn ManageAgent(id: String) -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();
    let navigator = use_navigator();

    let agent_opt = agents_resource
        .read()
        .as_ref()
        .and_then(|agents| agents.iter().find(|a| a.id == id).cloned());

    if agent_opt.is_none() {
        return rsx! {
            div { class: "flex-1 overflow-y-auto px-10 py-12",
                p { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-500", {t!("loading-agents")} }
            }
        };
    }

    let agent = agent_opt.unwrap();
    let initial = agent.name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_else(|| "·".into());

    let mut edit_mode = use_signal(|| false);
    let mut edit_name = use_signal(|| agent.name.clone());
    let mut edit_specialty = use_signal(|| agent.specialty.clone());
    let mut edit_personality = use_signal(|| agent.personality);
    let mut is_saving = use_signal(|| false);

    let delete_agent = {
        let id = id.clone();
        move |_| {
            let id = id.clone();
            spawn(async move {
                if crate::server_fns::delete_agent(id).await.is_ok() {
                    agents_resource.restart();
                    navigator.push(Route::AgentList {});
                }
            });
        }
    };

    let save_agent = {
        let id = id.clone();
        let existing_send = agent.n8n_webhook_send.clone();
        let existing_receive = agent.n8n_webhook_receive.clone();
        move |_| {
            let id = id.clone();
            let name = edit_name.read().clone();
            let specialty = edit_specialty.read().clone();
            let send_opt = existing_send.clone();
            let recv_opt = existing_receive.clone();
            let p = *edit_personality.read();

            *is_saving.write() = true;
            spawn(async move {
                if crate::server_fns::update_agent(id, name, specialty, send_opt, recv_opt, p).await.is_ok() {
                    agents_resource.restart();
                    *edit_mode.write() = false;
                }
                *is_saving.write() = false;
            });
        }
    };

    let input_cls = "w-full bg-transparent border-0 border-b border-zinc-300 dark:border-zinc-700 px-0 py-2.5 text-zinc-900 dark:text-zinc-100 text-lg placeholder:text-zinc-400 focus:outline-none focus:border-zinc-900 dark:focus:border-zinc-100 transition-colors font-display";
    let textarea_cls = "w-full bg-zinc-50/60 dark:bg-zinc-900/40 border border-zinc-200 dark:border-zinc-800 rounded-xl px-4 py-3 text-zinc-900 dark:text-zinc-100 text-sm focus:outline-none focus:border-zinc-400 dark:focus:border-zinc-600 resize-none transition-colors";

    rsx! {
        div { class: "flex-1 overflow-y-auto",
            div { class: "max-w-4xl mx-auto px-10 py-12",

                // ── Top breadcrumb ────────────────────────────────────────────
                div { class: "mb-8 flex items-baseline gap-3",
                    Link {
                        to: Route::AgentList {},
                        class: "font-mono text-[11px] tracking-[0.22em] uppercase text-zinc-500 hover:text-zinc-900 dark:hover:text-zinc-100 no-underline",
                        {t!("manage-back")}
                    }
                    span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
                    span { class: "font-mono text-[11px] tracking-[0.28em] uppercase text-amber-600 dark:text-amber-400",
                        "Manage"
                    }
                }

                // ── Editorial hero ────────────────────────────────────────────
                div { class: "flex items-start gap-6 mb-10 reveal",
                    div { class: "monogram-ring w-24 h-24 rounded-2xl border border-zinc-200 dark:border-zinc-800 flex items-center justify-center font-display italic text-5xl text-zinc-900 dark:text-zinc-100 shrink-0",
                        "{initial}"
                    }
                    div { class: "flex-1 min-w-0",
                        span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-500", "agent file" }
                        if *edit_mode.read() {
                            input {
                                class: "{input_cls} text-5xl mt-1",
                                value: "{edit_name}",
                                oninput: move |evt| *edit_name.write() = evt.value(),
                            }
                        } else {
                            h1 { class: "font-display text-5xl text-zinc-900 dark:text-zinc-100 leading-none mt-1",
                                "{agent.name}"
                            }
                        }
                        if !*edit_mode.read() {
                            p { class: "font-display italic text-lg text-zinc-500 dark:text-zinc-400 mt-3 leading-relaxed",
                                "{agent.specialty}"
                            }
                        }
                    }
                }

                // ── Action bar ───────────────────────────────────────────────
                div { class: "flex flex-wrap gap-2 mb-10",
                    if *edit_mode.read() {
                        button {
                            class: "px-6 py-2.5 rounded-lg bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 font-mono text-[11px] tracking-[0.22em] uppercase hover:opacity-90 transition-opacity disabled:opacity-50",
                            onclick: save_agent,
                            disabled: *is_saving.read(),
                            if *is_saving.read() { {t!("manage-edit-saving")} } else { {t!("manage-edit-save")} }
                        }
                        button {
                            class: "px-6 py-2.5 rounded-lg bg-transparent border border-zinc-200 dark:border-zinc-800 text-zinc-600 dark:text-zinc-300 font-mono text-[11px] tracking-[0.22em] uppercase hover:border-zinc-400 dark:hover:border-zinc-600 transition-colors",
                            onclick: move |_| *edit_mode.write() = false,
                            {t!("manage-edit-cancel")}
                        }
                    } else {
                        Link {
                            to: Route::Chat { id: id.clone() },
                            class: "px-6 py-2.5 rounded-lg bg-violet-600 text-white font-mono text-[11px] tracking-[0.22em] uppercase hover:bg-violet-500 transition-colors no-underline",
                            {t!("manage-start-chat")}
                        }
                        button {
                            class: "px-6 py-2.5 rounded-lg bg-transparent border border-zinc-200 dark:border-zinc-800 text-zinc-600 dark:text-zinc-300 font-mono text-[11px] tracking-[0.22em] uppercase hover:border-zinc-400 dark:hover:border-zinc-600 transition-colors",
                            onclick: move |_| *edit_mode.write() = true,
                            {t!("manage-edit-btn")}
                        }
                        button {
                            class: "px-6 py-2.5 rounded-lg bg-transparent border border-red-200 dark:border-red-500/30 text-red-600 dark:text-red-400 font-mono text-[11px] tracking-[0.22em] uppercase hover:bg-red-50 dark:hover:bg-red-500/10 transition-colors",
                            onclick: delete_agent,
                            {t!("manage-delete-btn")}
                        }
                    }
                }

                if *edit_mode.read() {
                    EditPanel {
                        edit_specialty: edit_specialty,
                        edit_personality: edit_personality,
                        textarea_cls: textarea_cls.to_string(),
                    }
                } else {
                    PersonalityEqualizer { personality: agent.personality }
                }

                AgentSkills { agent_id: id.clone() }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct EditPanelProps {
    edit_specialty: Signal<String>,
    edit_personality: Signal<Personality>,
    textarea_cls: String,
}

#[component]
fn EditPanel(props: EditPanelProps) -> Element {
    let mut edit_specialty = props.edit_specialty;
    let mut edit_personality = props.edit_personality;
    let textarea_cls = props.textarea_cls;
    rsx! {
        section { class: "bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-7 mb-6",
            div { class: "flex items-baseline gap-3 mb-5",
                span { class: "font-mono text-2xl text-amber-600 dark:text-amber-400 leading-none", "01" }
                span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-400 dark:text-zinc-600", "Specialty" }
                span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
            }
            textarea {
                class: "{textarea_cls}",
                value: "{edit_specialty}",
                oninput: move |evt| *edit_specialty.write() = evt.value(),
                rows: 5,
            }
        }
        section { class: "bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-7 mb-6",
            div { class: "flex items-baseline gap-3 mb-5",
                span { class: "font-mono text-2xl text-amber-600 dark:text-amber-400 leading-none", "02" }
                span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-400 dark:text-zinc-600", "Personality" }
                span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
            }
            div { class: "flex flex-col gap-5",
                PersonalitySlider { label: t!("trait-openness"),            low: t!("trait-openness-low"),            high: t!("trait-openness-high"),            value: edit_personality.read().openness,            on_change: move |v| edit_personality.write().openness = v }
                PersonalitySlider { label: t!("trait-conscientiousness"),   low: t!("trait-conscientiousness-low"),   high: t!("trait-conscientiousness-high"),   value: edit_personality.read().conscientiousness,   on_change: move |v| edit_personality.write().conscientiousness = v }
                PersonalitySlider { label: t!("trait-extraversion"),        low: t!("trait-extraversion-low"),        high: t!("trait-extraversion-high"),        value: edit_personality.read().extraversion,        on_change: move |v| edit_personality.write().extraversion = v }
                PersonalitySlider { label: t!("trait-agreeableness"),       low: t!("trait-agreeableness-low"),       high: t!("trait-agreeableness-high"),       value: edit_personality.read().agreeableness,       on_change: move |v| edit_personality.write().agreeableness = v }
                PersonalitySlider { label: t!("trait-emotional-stability"), low: t!("trait-emotional-stability-low"), high: t!("trait-emotional-stability-high"), value: edit_personality.read().emotional_stability, on_change: move |v| edit_personality.write().emotional_stability = v }
            }
        }
    }
}

/// Read-only Five-Factor display rendered as a vertical 5-column equalizer.
#[component]
fn PersonalityEqualizer(personality: Personality) -> Element {
    let bars = [
        ("O", t!("trait-openness"),            personality.openness),
        ("C", t!("trait-conscientiousness"),   personality.conscientiousness),
        ("E", t!("trait-extraversion"),        personality.extraversion),
        ("A", t!("trait-agreeableness"),       personality.agreeableness),
        ("N", t!("trait-emotional-stability"), personality.emotional_stability),
    ];
    rsx! {
        section { class: "bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-7 mb-6 reveal",
            div { class: "flex items-baseline gap-3 mb-6",
                span { class: "font-mono text-2xl text-amber-600 dark:text-amber-400 leading-none", "01" }
                span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-400 dark:text-zinc-600", "Five-Factor signature" }
                span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
                span { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-zinc-500", "O · C · E · A · N" }
            }
            div { class: "grid grid-cols-5 gap-3",
                for (initial, label, value) in bars.iter() {
                    {
                        let pct = (*value * 100.0).round() as i32;
                        let height_style = format!("height: {}%;", pct.max(4));
                        rsx! {
                            div { key: "{initial}", class: "flex flex-col items-center gap-2",
                                // Bar
                                div { class: "relative w-full h-40 rounded-lg bg-zinc-100/60 dark:bg-zinc-800/40 border border-zinc-200 dark:border-zinc-800 overflow-hidden flex items-end",
                                    div {
                                        style: "{height_style}",
                                        class: "w-full bg-gradient-to-t from-violet-600 to-violet-400 dark:from-violet-500 dark:to-violet-300",
                                    }
                                    span { class: "absolute top-2 left-2 font-mono text-[10px] text-zinc-500 tabular-nums",
                                        "{pct}"
                                    }
                                    span { class: "absolute top-2 right-2 font-mono text-[10px] text-amber-600 dark:text-amber-400",
                                        "{initial}"
                                    }
                                }
                                span { class: "font-display italic text-xs text-center text-zinc-600 dark:text-zinc-300 leading-tight",
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AgentSkills(agent_id: String) -> Element {
    let i18n = i18n();
    let lang_str = i18n.language().to_string();

    let all_skills_resource = use_resource(move || {
        let l = lang_str.clone();
        async move { crate::server_fns::get_skills(l).await.unwrap_or_default() }
    });

    let mut selected = use_signal(|| None::<HashSet<String>>);
    let mut is_saving = use_signal(|| false);
    let mut just_saved = use_signal(|| false);

    let aid_for_load = agent_id.clone();
    use_future(move || {
        let aid = aid_for_load.clone();
        async move {
            let ids = crate::server_fns::get_agent_skill_ids(aid).await.unwrap_or_default();
            selected.set(Some(ids.into_iter().collect()));
        }
    });

    let all_skills_opt = all_skills_resource.read_unchecked();
    let all_skills: Vec<Skill> = all_skills_opt.clone().unwrap_or_default();
    let is_ready = selected.read().is_some();

    let save = {
        let aid = agent_id.clone();
        move |_| {
            let aid = aid.clone();
            let ids: Vec<String> = match selected.read().as_ref() {
                Some(set) => set.iter().cloned().collect(),
                None => return,
            };
            is_saving.set(true);
            just_saved.set(false);
            spawn(async move {
                let ok = crate::server_fns::set_agent_skills(aid, ids).await.is_ok();
                is_saving.set(false);
                if ok { just_saved.set(true); }
            });
        }
    };

    rsx! {
        section { class: "bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-7 reveal",
            div { class: "flex items-baseline gap-3 mb-5",
                span { class: "font-mono text-2xl text-amber-600 dark:text-amber-400 leading-none", "02" }
                span { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-400 dark:text-zinc-600", "Skills" }
                span { class: "h-px flex-1 bg-zinc-200 dark:bg-zinc-800" }
            }
            h2 { class: "font-display text-2xl text-zinc-900 dark:text-zinc-100 mb-1", {t!("manage-skills-title")} }
            p { class: "font-display italic text-sm text-zinc-500 mb-5", {t!("manage-skills-subtitle")} }

            if !is_ready {
                p { class: "font-mono text-[10px] tracking-[0.28em] uppercase text-zinc-500", {t!("manage-skills-saving")} }
            } else if all_skills.is_empty() {
                p { class: "font-display italic text-sm text-zinc-500", {t!("manage-skills-empty")} }
            } else {
                ul { class: "flex flex-col gap-2 mb-5",
                    for skill in all_skills.iter() {
                        {
                            let sid = skill.id.clone();
                            let sid_for_click = sid.clone();
                            let checked = selected.read().as_ref().map(|s| s.contains(&sid)).unwrap_or(false);
                            rsx! {
                                li {
                                    key: "{skill.id}",
                                    class: if checked {
                                        "flex items-start gap-3 p-3 rounded-xl border border-zinc-900 dark:border-zinc-100 bg-zinc-50 dark:bg-zinc-900 cursor-pointer"
                                    } else {
                                        "flex items-start gap-3 p-3 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:border-zinc-400 dark:hover:border-zinc-600 transition-colors cursor-pointer"
                                    },
                                    onclick: move |_| {
                                        let sid = sid_for_click.clone();
                                        let mut s = selected.write();
                                        if let Some(set) = s.as_mut() {
                                            if set.contains(&sid) { set.remove(&sid); } else { set.insert(sid); }
                                        }
                                        drop(s);
                                        just_saved.set(false);
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

                div { class: "flex items-center gap-3",
                    button {
                        class: "px-6 py-2.5 rounded-lg bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 font-mono text-[11px] tracking-[0.22em] uppercase hover:opacity-90 transition-opacity disabled:opacity-50",
                        onclick: save,
                        disabled: *is_saving.read(),
                        if *is_saving.read() { {t!("manage-skills-saving")} } else { {t!("manage-skills-save")} }
                    }
                    if *just_saved.read() {
                        span { class: "font-mono text-[10px] tracking-[0.22em] uppercase text-emerald-600 dark:text-emerald-400",
                            {t!("manage-skills-saved")}
                        }
                    }
                }
            }
        }
    }
}
