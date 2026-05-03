# Rig AI Chat Application

A modern, full-stack Rust web application powered by [Dioxus](https://dioxuslabs.com/) and [Rig](https://rig.rs/).

This application provides a highly aesthetic, responsive chat interface where users can converse with a General AI Assistant powered by OpenAI. It uses a **Dioxus Fullstack** architecture, meaning both the frontend WebAssembly (WASM) client and the backend server API are written in Rust and exist in a single codebase.

## 🚀 Features

* **Fullstack Rust**: Seamlessly integrates frontend UI with backend logic using Dioxus `#[server]` functions.
* **Rig Integration**: Uses `rig-core` for robust, modular LLM application orchestration.
* **Secure API Communication**: OpenAI API requests are securely executed on the backend server, keeping your API key hidden from the client browser.
* **Premium UI/UX**: Features a modern dark-mode aesthetic with glassmorphism panels, CSS micro-animations, and responsive design.

## 🛠️ Prerequisites

Before you begin, ensure you have the following installed:
* [Rust & Cargo](https://rustup.rs/) (latest stable version recommended)
* [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/) (`dx` command line tool)

You can install the Dioxus CLI via:
```bash
cargo install dioxus-cli
```

## ⚙️ Setup and Installation

1. **Clone the repository** (if you haven't already) and navigate to the project directory:
   ```bash
   cd dioxus_rig_app
   ```

2. **Configure your API Key**:
   The application requires an OpenAI API key to communicate with the `gpt-4o-mini` model. Set it as an environment variable in your terminal session before running the app.
   ```bash
   export OPENAI_API_KEY="sk-your-openai-api-key"
   ```

## 🏃 Running the Application

To run the application in development mode with hot-reloading:

```bash
dx serve
```

This command will:
1. Compile the backend server logic.
2. Compile the frontend client into WebAssembly.
3. Start the application on a local development server.

Once running, the CLI will output a local URL (typically `http://localhost:8080` or `http://127.0.0.1:8080`). Open this link in your browser to interact with the AI assistant!

## 📦 Project Structure

* `Dioxus.toml`: Configuration file for the Dioxus CLI (defines titles, assets, styling, etc.).
* `Cargo.toml`: Rust dependencies and feature definitions (`web` and `server` targets).
* `src/main.rs`: The entry point for the Dioxus application.
* `src/server_fns.rs`: Backend logic housing the Rig AI Agent and secure API communication.
* `src/components/chat.rs`: Frontend reactive component governing the chat interface state and design.
* `assets/main.css`: Premium Vanilla CSS stylesheet for application aesthetics.

## 📝 License

This project is open-source and available under the MIT License.
