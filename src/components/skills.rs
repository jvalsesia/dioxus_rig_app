use crate::Skill;
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};

const CATEGORY_IDS: &[&str] = &[
    "research",
    "utilities",
    "productivity",
    "communication",
    "integrations",
];

fn category_label(id: &str) -> String {
    match id {
        "research" => t!("skill-cat-research"),
        "utilities" => t!("skill-cat-utilities"),
        "productivity" => t!("skill-cat-productivity"),
        "communication" => t!("skill-cat-communication"),
        "integrations" => t!("skill-cat-integrations"),
        other => other.to_string(),
    }
}

#[component]
pub fn Skills() -> Element {
    let i18n = i18n();
    let lang_str = i18n.language().to_string();

    let mut skills_resource = use_resource(move || {
        let l = lang_str.clone();
        async move {
            crate::server_fns::get_skills(l)
                .await
                .unwrap_or_else(|_| vec![])
        }
    });

    let mut query = use_signal(String::new);
    let mut category = use_signal(|| "all".to_string());
    let mut show_modal = use_signal(|| false);

    let skills_opt = skills_resource.read_unchecked();
    let skills: Vec<Skill> = skills_opt.clone().unwrap_or_default();

    let q = query.read().to_lowercase();
    let cat = category.read().clone();
    let filtered: Vec<Skill> = skills
        .iter()
        .filter(|s| {
            (cat == "all" || s.category == cat)
                && (q.is_empty()
                    || s.name.to_lowercase().contains(&q)
                    || s.description.to_lowercase().contains(&q))
        })
        .cloned()
        .collect();
    let count = filtered.len();

    rsx! {
        div { class: "flex-1 overflow-y-auto",
            div { class: "max-w-6xl mx-auto px-10 py-12",

                // ── Header ────────────────────────────────────────────────────
                header { class: "mb-10 reveal flex items-end justify-between gap-6 flex-wrap",
                    div {
                        h1 { class: "font-display font-semibold text-6xl text-zinc-900 dark:text-zinc-100 leading-[0.95] tracking-tight",
                            {t!("skills-title")}
                        }
                        p { class: "mt-3 text-base text-zinc-500 dark:text-zinc-400 max-w-2xl",
                            {t!("skills-subtitle")}
                        }
                    }
                    button {
                        class: "px-5 py-2.5 rounded-lg bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 text-sm font-medium hover:opacity-90 transition-opacity",
                        onclick: move |_| show_modal.set(true),
                        {t!("skills-new")}
                    }
                }

                // ── Search bar ────────────────────────────────────────────────
                div { class: "flex flex-wrap items-center gap-3 mb-8 bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl px-4 py-3 reveal-1",
                    input {
                        r#type: "text",
                        placeholder: "{t!(\"skills-search-placeholder\")}",
                        class: "flex-1 min-w-[200px] bg-transparent text-sm text-zinc-900 dark:text-zinc-100 px-2 py-1.5 outline-none placeholder:text-zinc-400 dark:placeholder:text-zinc-600",
                        value: "{query}",
                        oninput: move |e| query.set(e.value()),
                    }
                    select {
                        class: "bg-zinc-50 dark:bg-zinc-800/50 border border-zinc-200 dark:border-zinc-800 text-xs font-medium text-zinc-700 dark:text-zinc-300 rounded-lg px-3 py-1.5 outline-none cursor-pointer",
                        onchange: move |e| category.set(e.value()),
                        option { value: "all", {t!("skills-all-categories")} }
                        for c in CATEGORY_IDS.iter() {
                            option { value: "{c}", {category_label(c)} }
                        }
                    }
                    span { class: "text-xs text-zinc-500",
                        {t!("skills-found", count: count as i64)}
                    }
                }

                // ── List ─────────────────────────────────────────────────────
                if skills_opt.is_none() {
                    p { class: "text-sm text-zinc-500", {t!("skills-loading")} }
                } else if skills.is_empty() {
                    div { class: "rounded-2xl border border-dashed border-zinc-300 dark:border-zinc-800 bg-white/60 dark:bg-zinc-900/40 px-10 py-16 flex flex-col items-center text-center max-w-xl mx-auto",
                        h3 { class: "font-display font-semibold text-3xl text-zinc-900 dark:text-zinc-100 mb-2 tracking-tight",
                            {t!("skills-empty")}
                        }
                    }
                } else {
                    ul { class: "flex flex-col gap-3",
                        for skill in filtered.iter() {
                            {
                                let id = skill.id.clone();
                                let cat_label = category_label(&skill.category);
                                rsx! {
                                    li {
                                        key: "{skill.id}",
                                        class: "group flex items-center gap-5 bg-white dark:bg-zinc-900/60 border border-zinc-200 dark:border-zinc-800 rounded-2xl px-5 py-4 hover:border-zinc-400 dark:hover:border-zinc-600 transition-colors",
                                        div { class: "flex-1 min-w-0",
                                            h2 { class: "font-display font-semibold text-lg text-zinc-900 dark:text-zinc-100 tracking-tight", "{skill.name}" }
                                            p { class: "text-sm text-zinc-500 mt-0.5 leading-relaxed", "{skill.description}" }
                                        }
                                        span { class: "text-xs font-medium px-3 py-1 rounded-full border border-zinc-200 dark:border-zinc-800 text-zinc-600 dark:text-zinc-300 shrink-0",
                                            "{cat_label}"
                                        }
                                        button {
                                            class: "px-3 py-1.5 text-xs font-medium rounded-lg bg-transparent border border-red-200 dark:border-red-500/30 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-500/10 transition-colors shrink-0",
                                            onclick: move |_| {
                                                let id = id.clone();
                                                spawn(async move {
                                                    if crate::server_fns::delete_skill(id).await.is_ok() {
                                                        skills_resource.restart();
                                                    }
                                                });
                                            },
                                            {t!("skills-delete")}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                footer { class: "mt-12 pt-6 border-t border-zinc-200 dark:border-zinc-800",
                    p { class: "text-sm text-zinc-500", {t!("skills-footer")} }
                }
            }
        }

        if *show_modal.read() {
            CreateSkillModal {
                onclose: move |_| show_modal.set(false),
                onsaved: move |_| {
                    show_modal.set(false);
                    skills_resource.restart();
                },
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CreateSkillModalProps {
    onclose: EventHandler<()>,
    onsaved: EventHandler<()>,
}

#[component]
fn CreateSkillModal(props: CreateSkillModalProps) -> Element {
    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut category = use_signal(|| "research".to_string());
    let mut is_loading = use_signal(|| false);

    let input_cls = "w-full bg-transparent border-0 border-b border-zinc-300 dark:border-zinc-700 px-0 py-2.5 text-zinc-900 dark:text-zinc-100 text-base placeholder:text-zinc-400 focus:outline-none focus:border-zinc-900 dark:focus:border-zinc-100 transition-colors";

    let submit = move |_| {
        let n = name.read().trim().to_string();
        let d = description.read().trim().to_string();
        let c = category.read().clone();
        if n.is_empty() || d.is_empty() {
            return;
        }
        is_loading.set(true);
        spawn(async move {
            let ok = crate::server_fns::add_skill(n, d, c).await.is_ok();
            is_loading.set(false);
            if ok {
                props.onsaved.call(());
            }
        });
    };

    rsx! {
        div { class: "fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50",
            div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-8 w-full max-w-lg mx-4 flex flex-col gap-6",
                div { class: "flex items-center justify-between",
                    h3 { class: "font-display font-semibold text-2xl text-zinc-900 dark:text-zinc-100 tracking-tight", {t!("skill-modal-title")} }
                    button {
                        class: "text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100 text-xl cursor-pointer bg-transparent border-none leading-none",
                        onclick: move |_| props.onclose.call(()),
                        "✕"
                    }
                }

                div { class: "flex flex-col gap-2",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("skill-modal-name")} }
                    input {
                        class: "{input_cls}",
                        placeholder: t!("skill-modal-name-placeholder"),
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                    }
                }

                div { class: "flex flex-col gap-2",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("skill-modal-description")} }
                    textarea {
                        class: "w-full bg-zinc-50/60 dark:bg-zinc-900/40 border border-zinc-200 dark:border-zinc-800 rounded-xl px-4 py-3 text-sm focus:outline-none focus:border-zinc-400 dark:focus:border-zinc-600 resize-none text-zinc-900 dark:text-zinc-100",
                        placeholder: t!("skill-modal-description-placeholder"),
                        value: "{description}",
                        oninput: move |e| description.set(e.value()),
                        rows: 3,
                    }
                }

                div { class: "flex flex-col gap-2",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("skill-modal-category")} }
                    select {
                        class: "w-full bg-zinc-50/60 dark:bg-zinc-900/40 border border-zinc-200 dark:border-zinc-800 rounded-xl px-4 py-3 text-sm text-zinc-700 dark:text-zinc-300 cursor-pointer outline-none",
                        value: "{category}",
                        onchange: move |e| category.set(e.value()),
                        for c in CATEGORY_IDS.iter() {
                            option { value: "{c}", {category_label(c)} }
                        }
                    }
                }

                button {
                    class: "w-full px-6 py-3 rounded-lg bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 text-sm font-medium hover:opacity-90 transition-opacity disabled:opacity-50",
                    onclick: submit,
                    disabled: *is_loading.read(),
                    if *is_loading.read() { {t!("skill-modal-creating")} } else { {t!("skill-modal-submit")} }
                }
            }
        }
    }
}
