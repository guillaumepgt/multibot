---
name: multibot
description: >-
  Use this skill whenever working on, navigating, building, testing, or resolving issues in the MultiBot project.
---

# MultiBot Runbook & Quick Access Guide

## Project Location & Remote
- **Directory**: `/workspace/users/guillaume_prigent44_gmail_com/workspace/multibot`
- **GitHub Repository**: [https://github.com/guillaumepgt/multibot](https://github.com/guillaumepgt/multibot)
- **Roadmap & Spec**: [`ROADMAP_ISSUES.md`](/workspace/users/guillaume_prigent44_gmail_com/workspace/multibot/ROADMAP_ISSUES.md)

## Milestones & Issue Quick Reference
- **Milestone 1 (Rust Backend Core)**:
  - Issue #1: Workspace Cargo initialization (`crates/multibot-core`, `server`, `tools`)
  - Issue #2: SQLite persistence with SQLx
  - Issue #3: Unified LLM client (Ollama, Gemini, Claude, OpenAI)
  - Issue #4: Tool system (Sandbox, File ops & Git)
- **Milestone 2 (Multi-Agent Engine)**:
  - Issue #5: ReAct loop & state machine
  - Issue #6: Scrum Master Agent (decomposition & backlog)
  - Issue #7: Architect Agent (tech specs & constraints)
  - Issue #8: Developer & QA Agents
  - Issue #9: Human-in-the-Loop & Autonomous mode
- **Milestone 3 (API Server & Realtime)**:
  - Issue #10: Axum REST API
  - Issue #11: WebSocket channels for live logs & token streaming
- **Milestone 4 (React UI)**:
  - Issue #12: Vite + React 19 + TypeScript + Tailwind setup
  - Issue #13: Interactive Kanban & timeline
  - Issue #14: Agent live chat & discussion feed
  - Issue #15: Model manager & settings
  - Issue #16: File explorer & git diff viewer
- **Milestone 5 (Cross-Platform & Deployment)**:
  - Issue #17: Tauri v2 Desktop packaging
  - Issue #18: Mobile adaptation (iOS/Android)
  - Issue #19: 1-click Docker server deployment
  - Issue #20: Open source docs & CI/CD workflows

## Routine Workflows
1. Always `cd /workspace/users/guillaume_prigent44_gmail_com/workspace/multibot` before running commands.
2. Check Git status: `git status`
3. Fetch updates: `git pull origin main`
4. Create feature branch: `git checkout -b feat/issue-<id>-<name>`
5. Push changes: `git push origin <branch>`
