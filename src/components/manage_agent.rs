use crate::{Agent, Route, Skill};
use dioxus::prelude::*;
use dioxus_i18n::{t, prelude::*};
use std::collections::HashSet;

#[component]
pub fn ManageAgent(id: String) -> Element {
    let mut agents_resource = use_context::<Resource<Vec<Agent>>>();
    let navigator = use_navigator();

    let agent_opt = agents_resource.read().as_ref().and_then(|agents| {
        agents.iter().find(|a| a.id == id).cloned()
    });

    if agent_opt.is_none() {
        return rsx! {
            div { class: "flex-1 overflow-y-auto p-8",
                p { class: "text-sm text-zinc-500", {t!("loading-agents")} }
            }
        };
    }

    let agent = agent_opt.unwrap();

    let mut edit_mode = use_signal(|| false);
    let mut edit_name = use_signal(|| agent.name.clone());
    let mut edit_specialty = use_signal(|| agent.specialty.clone());
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

            *is_saving.write() = true;
            spawn(async move {
                if crate::server_fns::update_agent(id, name, specialty, send_opt, recv_opt).await.is_ok() {
                    agents_resource.restart();
                    *edit_mode.write() = false;
                }
                *is_saving.write() = false;
            });
        }
    };

    let input_cls = "w-full bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 px-4 py-3 rounded-xl text-zinc-900 dark:text-zinc-100 text-sm placeholder:text-zinc-400 dark:placeholder:text-zinc-500 focus:outline-none focus:border-violet-400 dark:focus:border-violet-500/50 focus:ring-2 focus:ring-violet-200 dark:focus:ring-violet-500/20 transition-all";

    rsx! {
        div { class: "flex-1 overflow-y-auto p-8",
            div { class: "max-w-2xl mx-auto",

                div { class: "mb-6",
                    Link {
                        to: Route::AgentList {},
                        class: "text-sm text-violet-600 dark:text-violet-400 hover:text-violet-500 dark:hover:text-violet-300 font-medium no-underline",
                        {t!("manage-back")}
                    }
                }

                div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-8 flex flex-col gap-6 shadow-sm mb-6",

                    if *edit_mode.read() {
                        h3 { class: "text-xl font-semibold text-zinc-900 dark:text-zinc-100", {t!("manage-edit-title")} }

                        div { class: "flex flex-col gap-1.5",
                            label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("manage-edit-name")} }
                            input {
                                class: "{input_cls}",
                                value: "{edit_name}",
                                oninput: move |evt| *edit_name.write() = evt.value()
                            }
                        }

                        div { class: "flex flex-col gap-1.5",
                            label { class: "text-xs font-medium text-zinc-500 uppercase tracking-wider", {t!("manage-edit-specialty")} }
                            textarea {
                                class: "{input_cls} resize-none",
                                value: "{edit_specialty}",
                                oninput: move |evt| *edit_specialty.write() = evt.value(),
                                rows: 4
                            }
                        }

                        div { class: "flex gap-3",
                            button {
                                class: "bg-violet-600 text-white py-2.5 px-6 rounded-xl font-semibold text-sm hover:bg-violet-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed",
                                onclick: save_agent,
                                disabled: *is_saving.read(),
                                if *is_saving.read() { {t!("manage-edit-saving")} } else { {t!("manage-edit-save")} }
                            }
                            button {
                                class: "bg-transparent text-zinc-500 border border-zinc-200 dark:border-zinc-700 py-2.5 px-6 rounded-xl font-semibold text-sm hover:bg-zinc-100 dark:hover:bg-zinc-800 hover:text-zinc-700 dark:hover:text-zinc-200 transition-all",
                                onclick: move |_| *edit_mode.write() = false,
                                {t!("manage-edit-cancel")}
                            }
                        }
                    } else {
                        h2 { class: "text-3xl font-bold text-zinc-900 dark:text-zinc-100", "{agent.name}" }
                        p { class: "text-zinc-500 leading-relaxed", "{agent.specialty}" }

                        div { class: "flex gap-3",
                            Link {
                                to: Route::Chat { id: id.clone() },
                                class: "flex-1 bg-violet-600 text-white py-2.5 px-4 rounded-xl font-semibold text-sm text-center hover:bg-violet-500 transition-all no-underline",
                                {t!("manage-start-chat")}
                            }
                            button {
                                class: "flex-1 bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-200 py-2.5 px-4 rounded-xl font-semibold text-sm hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-all",
                                onclick: move |_| *edit_mode.write() = true,
                                {t!("manage-edit-btn")}
                            }
                            button {
                                class: "flex-1 bg-red-50 dark:bg-red-500/10 text-red-600 dark:text-red-400 border border-red-200 dark:border-red-500/30 py-2.5 px-4 rounded-xl font-semibold text-sm hover:bg-red-100 dark:hover:bg-red-500/20 transition-all",
                                onclick: delete_agent,
                                {t!("manage-delete-btn")}
                            }
                        }
                    }
                }

                AgentSkills { agent_id: id.clone() }
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
        async move {
            crate::server_fns::get_skills(l).await.unwrap_or_default()
        }
    });

    // None = still loading; Some(set) = ready for editing.
    let mut selected = use_signal(|| None::<HashSet<String>>);
    let mut is_saving = use_signal(|| false);
    let mut just_saved = use_signal(|| false);

    let aid_for_load = agent_id.clone();
    use_future(move || {
        let aid = aid_for_load.clone();
        async move {
            let ids = crate::server_fns::get_agent_skill_ids(aid)
                .await
                .unwrap_or_default();
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
                if ok {
                    just_saved.set(true);
                }
            });
        }
    };


    rsx! {
        div { class: "bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl p-8 flex flex-col gap-4 shadow-sm",
            div {
                h3 { class: "text-xl font-semibold text-zinc-900 dark:text-zinc-100", {t!("manage-skills-title")} }
                p { class: "text-sm text-zinc-500 mt-1", {t!("manage-skills-subtitle")} }
            }

            if !is_ready {
                p { class: "text-sm text-zinc-500", {t!("manage-skills-saving")} }
            } else if all_skills.is_empty() {
                p { class: "text-sm text-zinc-500", {t!("manage-skills-empty")} }
            } else {
                ul { class: "flex flex-col gap-2",
                    for skill in all_skills.iter() {
                        {
                            let sid = skill.id.clone();
                            let sid_for_click = sid.clone();
                            let checked = selected.read().as_ref()
                                .map(|s| s.contains(&sid))
                                .unwrap_or(false);
                            rsx! {
                                li {
                                    key: "{skill.id}",
                                    class: "flex items-start gap-3 p-3 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:border-violet-300 dark:hover:border-violet-500/50 transition-all cursor-pointer",
                                    onclick: move |_| {
                                        let sid = sid_for_click.clone();
                                        let mut s = selected.write();
                                        if let Some(set) = s.as_mut() {
                                            if set.contains(&sid) {
                                                set.remove(&sid);
                                            } else {
                                                set.insert(sid);
                                            }
                                        }
                                        drop(s);
                                        just_saved.set(false);
                                    },
                                    input {
                                        r#type: "checkbox",
                                        checked: checked,
                                        class: "mt-1 accent-violet-600 cursor-pointer pointer-events-none",
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

                div { class: "flex items-center gap-3 pt-2",
                    button {
                        class: "bg-violet-600 text-white py-2.5 px-6 rounded-xl font-semibold text-sm hover:bg-violet-500 transition-all disabled:opacity-50 disabled:cursor-not-allowed",
                        onclick: save,
                        disabled: *is_saving.read(),
                        if *is_saving.read() { {t!("manage-skills-saving")} } else { {t!("manage-skills-save")} }
                    }
                    if *just_saved.read() {
                        span { class: "text-xs text-emerald-600 dark:text-emerald-400 font-medium",
                            {t!("manage-skills-saved")}
                        }
                    }
                }
            }
        }
    }
}
