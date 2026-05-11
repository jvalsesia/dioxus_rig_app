use dioxus::prelude::*;
use dioxus_i18n::t;

#[derive(Clone, PartialEq)]
struct Skill {
    id: &'static str,
    category: &'static str,
}

fn all_skills() -> Vec<Skill> {
    vec![
        Skill { id: "search", category: "research" },
        Skill { id: "calc", category: "utilities" },
        Skill { id: "calendar", category: "productivity" },
        Skill { id: "email", category: "communication" },
        Skill { id: "api", category: "integrations" },
        Skill { id: "summarize", category: "research" },
        Skill { id: "translate", category: "communication" },
        Skill { id: "reports", category: "productivity" },
        Skill { id: "sentiment", category: "research" },
        Skill { id: "meetings", category: "productivity" },
        Skill { id: "sms", category: "communication" },
        Skill { id: "webhooks", category: "integrations" },
    ]
}

fn skill_name(id: &str) -> String {
    match id {
        "search" => t!("skill-search-name"),
        "calc" => t!("skill-calc-name"),
        "calendar" => t!("skill-calendar-name"),
        "email" => t!("skill-email-name"),
        "api" => t!("skill-api-name"),
        "summarize" => t!("skill-summarize-name"),
        "translate" => t!("skill-translate-name"),
        "reports" => t!("skill-reports-name"),
        "sentiment" => t!("skill-sentiment-name"),
        "meetings" => t!("skill-meetings-name"),
        "sms" => t!("skill-sms-name"),
        "webhooks" => t!("skill-webhooks-name"),
        _ => String::new(),
    }
}

fn skill_desc(id: &str) -> String {
    match id {
        "search" => t!("skill-search-desc"),
        "calc" => t!("skill-calc-desc"),
        "calendar" => t!("skill-calendar-desc"),
        "email" => t!("skill-email-desc"),
        "api" => t!("skill-api-desc"),
        "summarize" => t!("skill-summarize-desc"),
        "translate" => t!("skill-translate-desc"),
        "reports" => t!("skill-reports-desc"),
        "sentiment" => t!("skill-sentiment-desc"),
        "meetings" => t!("skill-meetings-desc"),
        "sms" => t!("skill-sms-desc"),
        "webhooks" => t!("skill-webhooks-desc"),
        _ => String::new(),
    }
}

fn category_label(id: &str) -> String {
    match id {
        "research" => t!("skill-cat-research"),
        "utilities" => t!("skill-cat-utilities"),
        "productivity" => t!("skill-cat-productivity"),
        "communication" => t!("skill-cat-communication"),
        "integrations" => t!("skill-cat-integrations"),
        _ => String::new(),
    }
}

#[component]
pub fn Skills() -> Element {
    let mut query = use_signal(String::new);
    let mut category = use_signal(|| "all".to_string());
    let mut added = use_signal(Vec::<String>::new);

    let skills = all_skills();

    let categories: Vec<&'static str> = {
        let mut c: Vec<&'static str> = skills.iter().map(|s| s.category).collect();
        c.sort();
        c.dedup();
        c
    };

    let q = query.read().to_lowercase();
    let cat = category.read().clone();
    let filtered: Vec<Skill> = skills
        .into_iter()
        .filter(|s| {
            (cat == "all" || s.category == cat)
                && (q.is_empty()
                    || skill_name(s.id).to_lowercase().contains(&q)
                    || skill_desc(s.id).to_lowercase().contains(&q))
        })
        .collect();
    let count = filtered.len();

    rsx! {
        div { class: "flex-1 overflow-y-auto p-8",
            // Header
            header { class: "mb-6",
                h1 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100", {t!("skills-title")} }
                p { class: "text-sm text-zinc-500 mt-1 max-w-2xl", {t!("skills-subtitle")} }
                button {
                    class: "mt-3 inline-flex items-center px-3 py-1.5 text-xs font-semibold rounded-lg bg-violet-50 dark:bg-violet-500/10 text-violet-600 dark:text-violet-400 border border-violet-200 dark:border-violet-500/30 hover:bg-violet-100 dark:hover:bg-violet-500/20 transition-all cursor-pointer",
                    {t!("skills-learn-more")}
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
                    for c in categories.iter() {
                        option { value: "{c}", {category_label(c)} }
                    }
                }
                span { class: "text-xs text-zinc-500", {t!("skills-found", count: count as i64)} }
            }

            // List
            ul { class: "flex flex-col gap-3",
                for skill in filtered.iter() {
                    {
                        let name = skill_name(skill.id);
                        let desc = skill_desc(skill.id);
                        let cat_label = category_label(skill.category);
                        let id_key = skill.id.to_string();
                        let is_added = added.read().contains(&id_key);
                        rsx! {
                            li {
                                key: "{skill.id}",
                                class: "flex items-center gap-4 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-4 hover:border-violet-300 dark:hover:border-violet-500/50 transition-all",
                                div { class: "flex-1 min-w-0",
                                    h2 { class: "text-sm font-semibold text-zinc-900 dark:text-zinc-100", "{name}" }
                                    p { class: "text-xs text-zinc-500 mt-0.5", "{desc}" }
                                }
                                span { class: "text-xs font-medium px-2.5 py-1 rounded-full bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300 border border-zinc-200 dark:border-zinc-700 shrink-0",
                                    "{cat_label}"
                                }
                                button {
                                    class: if is_added {
                                        "px-3 py-1.5 text-xs font-semibold rounded-lg bg-emerald-50 dark:bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-200 dark:border-emerald-500/30 cursor-default shrink-0"
                                    } else {
                                        "px-3 py-1.5 text-xs font-semibold rounded-lg bg-violet-50 dark:bg-violet-500/10 text-violet-600 dark:text-violet-400 border border-violet-200 dark:border-violet-500/30 hover:bg-violet-100 dark:hover:bg-violet-500/20 transition-all cursor-pointer shrink-0"
                                    },
                                    onclick: move |_| {
                                        let mut list = added.write();
                                        if !list.contains(&id_key) {
                                            list.push(id_key.clone());
                                        }
                                    },
                                    if is_added { {t!("skills-added")} } else { {t!("skills-add")} }
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
    }
}
