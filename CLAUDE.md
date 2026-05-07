# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# 1. Install Tailwind standalone binary (first time only, no npm/Node needed)
curl -sLo ~/.local/bin/tailwindcss \
  https://github.com/tailwindlabs/tailwindcss/releases/download/v4.2.4/tailwindcss-linux-x64
chmod +x ~/.local/bin/tailwindcss

# 2a. Watch CSS during development (run in a separate terminal)
tailwindcss -i ./assets/input.css -o ./assets/tailwind.css --watch

# 2b. Or build CSS once for production
tailwindcss -i ./assets/input.css -o ./assets/tailwind.css --minify

# Development (hot reload) — run alongside tailwindcss watch
dx serve

# Production build
dx build --release

# Docker (local)
docker-compose up --build

# Deploy to Railway
railway up
```

Requires Rust 1.95+, Dioxus CLI 0.7.7 (`cargo install dioxus-cli@0.7.7`). No Node.js or npm required. There are no test or lint commands configured.

## Styling — Tailwind CSS

The project uses **Tailwind CSS v3** with the JIT engine.

| File | Purpose |
|------|---------|
| [assets/input.css](assets/input.css) | Tailwind v4 source — `@import "tailwindcss"`, `@source`, `@custom-variant`, custom keyframes/scrollbar |
| [assets/tailwind.css](assets/tailwind.css) | Generated output (run `tailwindcss -i ./assets/input.css -o ./assets/tailwind.css --minify`) |

**Dark mode** is class-based (`dark` on `<html>`). The `is_dark: Signal<bool>` in context drives a `use_effect` that toggles the class via `document::eval`. Default is dark.

**Color palette (zinc + violet):**

- Backgrounds: `zinc-50` / `zinc-950`
- Cards/panels: `white` / `zinc-900`
- Borders: `zinc-200` / `zinc-800`
- Accent (primary actions): `violet-600`
- Chat button: `violet-*`; Manage button: `sky-*`; Deploy button: `emerald-*`; Danger: `red-*`

**Tailwind scanning**: class strings must be complete static string literals in `.rs` files — never build class names via `format!()` or string concatenation.

## Architecture

**Dioxus Fullstack** app (Rust → WASM + server). The same binary serves both the SSR/API backend (Tokio/Axum) and the client-side WASM bundle. The split is handled by the `#[server]` macro — any function annotated with it runs only on the server; the compiler generates an RPC stub for the client.

### Key files

| File | Purpose |
|------|---------|
| [src/main.rs](src/main.rs) | App entry, router, i18n init, dark-mode signal, SidebarLayout |
| [src/server_fns.rs](src/server_fns.rs) | All `#[server]` functions (LanceDB, OpenAI, n8n) |
| [src/components/](src/components/) | UI components (Chat, AgentList, ManageAgent, DeployAgent, CreateAgentModal) |
| [locales/](locales/) | Fluent `.ftl` translation files (en-US, pt-BR) |

### Layout

The main layout (`SidebarLayout` in `main.rs`) uses `flex h-screen overflow-hidden`. The sidebar is a fixed `w-64` column; the right side is `flex-1 flex flex-col min-h-0`. Each route component is responsible for its own scroll container:

- Regular pages (AgentList, ManageAgent, DeployAgent): wrap content in `flex-1 overflow-y-auto p-8`
- Chat: uses `flex flex-col flex-1 min-h-0` with `flex-1 overflow-y-auto` on the messages area

### Routes

- `/` → `AgentList` — dashboard grid of agent cards
- `/chat/:id` → `Chat` — conversation UI
- `/manage/:id` → `ManageAgent` — edit/delete agent form
- `/deploy/:id` → `DeployAgent` — n8n webhook + Evolution API config

### Data layer

Agents are stored in **LanceDB** (local vector DB at `/data/lancedb`, table `agents_v2`). The Arrow schema holds: `id` (string/UUID), `name`, `specialty`, `n8n_webhook_url`, `n8n_webhook_test_url`. LanceDB is initialized in `get_agents()` and creates a default "General Assistant" agent on first run.

Chat goes through **Rig-core** (LLM orchestration) → OpenAI `gpt-4o-mini`. The API key is read server-side from `OPENAI_API_KEY` env var — it is never sent to the client.

### State management

- `use_signal()` — component-local reactive state
- `use_context()` — global signals (agents `Resource`, `is_dark: Signal<bool>`)
- `use_resource()` — async data fetching; call `.restart()` to refetch
- `spawn(async { ... })` — fire-and-forget async tasks inside event handlers

### i18n

Uses **dioxus-i18n** with Fluent (`.ftl`) files. Translation macro: `t!("key")` or `t!("key", param: value)`. Language is switched at runtime via `i18n.set_language()`. Adding a new locale means adding a `.ftl` file under `locales/` and registering it in `main.rs`.

### n8n integration

Each agent can have two optional webhook URLs (`n8n_webhook_send` for production, `n8n_webhook_receive` for receiving). The server sends a `POST` with `{"message": "..."}` JSON body to the webhook when a chat message is sent.
