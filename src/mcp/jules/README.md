# Jules & Hermes: The Agent OS
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/billlzzz10/jules-mcp-server)
[![Version](https://img.shields.io/badge/version-v0.2.2-blue)](https://github.com/billlzzz10/jules-mcp-server/blob/main/CHANGELOG.md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A self-managing, multi-agent infrastructure designed for autonomous software development, directed by a Human Architect.

---

## 📖 Description

This project, born from a collaboration between a Human Architect and a Gemini CLI agent, establishes a robust "Agent Operating System" on a local machine (initially Termux/Android). It integrates multiple specialized AI agents (like Google's Jules and Nous Research's Hermes) under a unified command-line interface, enabling them to work together on complex software development tasks, from research and planning to implementation and self-healing.

The core problem it solves is orchestrating multiple, specialized AI agents in an environment with limited resources, while ensuring stability, security, and adherence to the high-level strategic goals set by a non-coding architect.

### ✨ Key Features
- **Multi-Agent Synergy:** Integrates disparate AI agents (Jules, Hermes) into a cooperative workforce via a Rust-based Multi-Chip-Proxy (MCP) server.
- **Unified Control Plane:** A single, lightweight CLI (`gh jules`) provides a consistent interface to check status, run diagnostics, and dispatch tasks to any agent.
- **Autonomous Self-Maintenance:** Features a `memory_guard` to prevent context overflow and a `maintenance` skill, created by an agent for itself, to monitor system health.
- **Environment-Agnostic Slim Bridge:** A lightweight Python bridge (`jules_api_bridge.py`) allows interaction with cloud-based agent APIs without heavy dependencies, ensuring it runs anywhere (including Termux).
- **Strict Protocol Enforcement:** Implements the Conductor Protocol for task management and a custom `boost_check.sh` hook to prevent runaway loops and ensure all actions are intentional.

## 📚 Table of Contents
1. [Description](#-description)
2. [Installation](#-installation)
3. [Quick Start](#-quick-start)
4. [Usage & Commands](#-usage--commands)
5. [Configuration](#-configuration)
6. [Development](#-development)
7. [Project Roadmap](#-project-roadmap)
8. [Contributing](#-contributing)
9. [License](#-license)
10. [Authors & Acknowledgments](#-authors--acknowledgments)

## ⚙️ Installation

### Prerequisites
- **Rust:** `cargo` & `rustc`
- **Node.js:** `node` & `npm`
- **Python:** `python3`
- **GitHub CLI:** `gh`
- **Hermes Agent:** The `hermes` command must be in your `PATH`.

### Installation Steps
The project includes 1-click setup scripts for both Termux/Linux and Windows.

**On Termux / Linux:**
```bash
bash setup.sh
```

**On Windows:**
```powershell
.\setup.ps1
```
These scripts will:
1.  Install the `jules-mcp-server` binary to your local bin path.
2.  Set up the necessary directory structures (`conductor`, `shared_memory`, etc.).
3.  Ensure the `gh jules` CLI extension is ready to use.

### Verification
After installation, run the system health check to ensure all components are operational:
```bash
gh jules doctor
```
You should see a report with all checks passing (`[OK]`).

## 🚀 Quick Start

1.  **Initialize the Environment:** If it's your first time, set up the Conductor directory.
    ```bash
    gh jules setup
    ```

2.  **Start a New Task:** To begin a new software development track, use the `new` command. This will automatically create a new track in `conductor/`.
    ```bash
    gh jules new "Implement a multi-agent research hub"
    ```

3.  **Check System Status:** Get a quick overview of the Agent OS.
    ```bash
    gh jules status
    ```

4.  **Delegate a Task to Hermes:** For autonomous research or background tasks.
    ```bash
    gh jules hermes "Research the best practices for Modal.com data pipelines"
    ```

## 🤖 Usage & Commands

The primary interface is the `gh jules` CLI extension.

| Command             | Description                                                                                             |
| ------------------- | ------------------------------------------------------------------------------------------------------- |
| `gh jules setup`    | Initializes the Conductor protocol directory (`conductor/`). If it exists, prompts the user to `implement`. |
| `gh jules new "task"` | Creates a new development track for a given task.                                                       |
| `gh jules status`   | Displays the real-time Agent OS dashboard (Agent status, Memory health).                                |
| `gh jules doctor`   | Runs a deep diagnostic health check on all system components (Runtimes, Agents, Binaries).                |
| `gh jules hermes "task"` | Dispatches a task directly to the Hermes agent for autonomous execution.                                  |
| `gh jules "task"`   | (Default) Dispatches a task to the Jules agent via the lightweight Python API bridge.                     |

## 🔧 Configuration

### Environment Variables
- **`JULES_API_KEY`**: The API key for the Jules agent. The `gh jules` CLI expects this to be stored in a `jules.key` file in the project root for the Slim Bridge to function.

### Project Files
- **`.gemini/GEMINI.md`**: Contains the global rules and operating principles for the AI agents.
- **`.gemini/hooks/boost_check.sh`**: The "Constriction Protocol" hook that intercepts prompts to prevent AI-slop, security risks, and runaway loops.
- **`conductor/tracks.md`**: The master registry for all active and archived development tracks.
- **`docs/MASTER_TECH_STACK.md`**: The Architect's definitive guide on which technology to use for which task (Rust vs. Python vs. TS).

## 👨‍💻 Development

### Setup
Follow the main [Installation](#-installation) steps. The same script prepares the development environment.

### Running Tests
The project has two primary test suites:
1.  **Rust Unit Tests:**
    ```bash
    cargo test
    ```
2.  **Intelligence Hook Validation:**
    ```bash
    bash tests/validate_hooks.sh
    ```

### Building the Project
To build the `jules-mcp-server` binary from source:
```bash
# For a release build
cargo build --release

# The binary will be in target/release/jules-mcp-server
```

## 🗺️ Project Roadmap

-   [ ] **Research Hub:** Complete the multi-agent research pipeline (`gh jules research "topic"`).
-   [ ] **Distributed Bridge:** Implement a bridge to allow agents to work across different machines (e.g., Termux coordinating with a Windows machine).
-   [ ] **Web UI Dashboard:** Evolve the CLI dashboard into a more visual web-based interface.

## ✨ Contributing
Contributions that adhere to the Architect's vision are welcome. Please follow the standard GitHub flow:
1.  Fork the repository.
2.  Create a new branch (`git checkout -b feature/your-feature`).
3.  Commit your changes (`git commit -am 'Add some feature'`).
4.  Push to the branch (`git push origin feature/your-feature`).
5.  Open a Pull Request.

## 📜 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👥 Authors & Acknowledgments
-   **Human Architect:** The visionary and strategist.
-   **Gemini CLI:** The lead implementation agent.
-   **Hermes Agent:** Specialist in autonomous research and self-maintenance.
-   **Jules Agent:** Specialist in cloud-based codebase analysis.

---
*This README was proudly generated by the Gemini CLI agent within the Agent OS it helped create.*
