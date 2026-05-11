use dioxus::prelude::*;
use dioxus_i18n::{t, prelude::*};
use crate::Skill;

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
        div { class: "flex-1 overflow-y-auto p-8",
            // Header
            header { class: "mb-6 flex items-start justify-between gap-4",
                div {
                    h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100", {t!("skills-title")} }
                    p { class: "text-sm text-zinc-500 mt-1 max-w-2xl", {t!("skills-subtitle")} }
                    button {
                        class: "mt-3 inline-flex items-center px-3 py-1.5 text-xs font-semibold rounded-lg bg-violet-50 dark:bg-violet-500/10 text-violet-600 dark:text-violet-400 border border-violet-200 dark:border-violet-500/30 hover:bg-violet-100 dark:hover:bg-violet-500/20 transition-all cursor-pointer",
                        {t!("skills-learn-more")}
                    }
                }
                button {
                    class: "shrink-0 px-4 py-2 text-sm font-semibold rounded-xl bg-violet-600 text-white hover:bg-violet-500 transition-all cursor-pointer shadow-sm",
                    onclick: move |_| show_modal.set(true),
                    {t!("skills-new")}
                }
            }

            // Search bar
            div { class: "flex flex-wrap items-center gap-3 mb-6 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-3",
                input {
                    r#type: "text",
                    placeholder: "{t!(\"skills-search-placeholder\")}",
                    class: "flex-1 min-w-[200px] bg-zinc-50 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-sm text-zinc-900 dark:text-zinc-100 rounded-xl px-3 py-2 outline-none focus:border-violet-400",
                    value: "{query}",
                    oninput: move |e| query.set(e.value()),
                }
                select {
                    class: "bg-zinc-50 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-sm text-zinc-700 dark:text-zinc-300 rounded-xl px-3 py-2 outline-none cursor-pointer",
                    onchange: move |e| category.set(e.value()),
                    option { value: "all", {t!("skills-all-categories")} }
                    for c in CATEGORY_IDS.iter() {
                        option { value: "{c}", {category_label(c)} }
                    }
                }
                span { class: "text-xs text-zinc-500", {t!("skills-found", count: count as i64)} }
            }

            // List
            if skills_opt.is_none() {
                p { class: "text-sm text-zinc-500", {t!("skills-loading")} }
            } else if skills.is_empty() {
                p { class: "text-sm text-zinc-500", {t!("skills-empty")} }
            } else {
                ul { class: "flex flex-col gap-3",
                    for skill in filtered.iter() {
                        {
                            let id = skill.id.clone();
                            let cat_label = category_label(&skill.category);
                            rsx! {
                                li {
                                    key: "{skill.id}",
                                    class: "flex items-center gap-4 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-4 hover:border-violet-300 dark:hover:border-violet-500/50 transition-all",
                                    div { class: "flex-1 min-w-0",
                                        h2 { class: "text-sm font-semibold text-zinc-900 dark:text-zinc-100", "{skill.name}" }
                                        p { class: "text-xs text-zinc-500 mt-0.5", "{skill.description}" }
                                    }
                                    span { class: "text-xs font-medium px-2.5 py-1 rounded-full bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300 border border-zinc-200 dark:border-zinc-700 shrink-0",
                                        "{cat_label}"
                                    }
                                    button {
                                        class: "px-3 py-1.5 text-xs font-semibold rounded-lg bg-red-50 dark:bg-red-500/10 text-red-600 dark:text-red-400 border border-red-200 dark:border-red-500/30 hover:bg-red-100 dark:hover:bg-red-500/20 transition-all cursor-pointer shrink-0",
                                        onclick: move |_| {
                                            let id = id.clone();
                                            spawn(async move {
                                                if crate::server_fns::delete_skill(id).await.is_ok() {
                                                    skills_resource.restart();
                                                }
                                            });
                                        },
                                        "🗑 " {t!("skills-delete")}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Footer
            footer { class: "mt-8 pt-6 border-t border-zinc-200 dark:border-zinc-800",
                p { class: "text-xs text-zinc-500", {t!("skills-footer")} }
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

    let input_cls = "w-full bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 px-4 py-3 rounded-xl text-zinc-900 dark:text-zinc-100 text-sm placeholder:text-zinc-400 dark:placeholder:text-zinc-500 focus:outline-none focus:border-violet-400 dark:focus:border-violet-500/50 focus:ring-2 focus:ring-violet-200 dark:focus:ring-violet-500/20 transition-all";

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
        div { class: "fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50",
            div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-6 w-full max-w-md mx-4 flex flex-col gap-5 shadow-2xl",
                div { class: "flex items-center justify-between",
                    h3 { class: "text-lg font-semibold text-zinc-900 dark:text-zinc-100", {t!("skill-modal-title")} }
                    button {
                        class: "text-zinc-400 hover:text-zinc-700 dark:hover:text-zinc-200 text-xl cursor-pointer transition-colors bg-transparent border-none leading-none",
                        onclick: move |_| props.onclose.call(()),
                        "✕"
                    }
                }

                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("skill-modal-name")} }
                    input {
                        class: "{input_cls}",
                        placeholder: t!("skill-modal-name-placeholder"),
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                    }
                }

                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("skill-modal-description")} }
                    textarea {
                        class: "{input_cls} resize-none",
                        placeholder: t!("skill-modal-description-placeholder"),
                        value: "{description}",
                        oninput: move |e| description.set(e.value()),
                        rows: 3,
                    }
                }

                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("skill-modal-category")} }
                    select {
                        class: "{input_cls} cursor-pointer",
                        value: "{category}",
                        onchange: move |e| category.set(e.value()),
                        for c in CATEGORY_IDS.iter() {
                            option { value: "{c}", {category_label(c)} }
                        }
                    }
                }

                button {
                    class: "w-full bg-violet-600 text-white py-3 rounded-xl font-semibold text-sm hover:bg-violet-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed",
                    onclick: submit,
                    disabled: *is_loading.read(),
                    if *is_loading.read() { {t!("skill-modal-creating")} } else { {t!("skill-modal-submit")} }
                }
            }
        }
    }
}
