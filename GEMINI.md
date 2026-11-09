# Directives Spécifiques - MultiBot

## 🌟 Présentation du Projet
**MultiBot** est une plateforme open-source d'ingénierie logicielle multi-agent autonome et collaborative (Human-in-the-Loop).

- **Dépôt GitHub :** [https://github.com/guillaumepgt/multibot](https://github.com/guillaumepgt/multibot)
- **Feuille de route & Spécifications des tickets :** Consulter [`ROADMAP_ISSUES.md`](./ROADMAP_ISSUES.md)

---

## 🏗️ Architecture Technique
1. **Backend & Moteur Agentique (Rust)**
   - Workspace Cargo modulaire :
     - `crates/multibot-core` : Modèles de données, machine à états ReAct, traits des LLM providers.
     - `crates/multibot-server` : Serveur Axum, API REST, WebSockets temps réel.
     - `crates/multibot-tools` : Sandbox filesystem, exécution de commandes, commandes Git.
   - Base de données : SQLite asynchrone via `sqlx`.
   - Modèles LLM : Connecteur unifié (Ollama local, Gemini, Claude, OpenAI-compatible).

2. **Frontend (React)**
   - Vite + React 19 + TypeScript + Tailwind CSS.
   - Gestion d'état : Zustand.
   - Icônes : Lucide React.
   - Communication : REST API + flux WebSockets temps réel.

3. **Packaging Multiplateforme (Desktop & Mobile)**
   - Tauri v2 (Linux, macOS, Windows, Android, iOS).

---

## 📌 Milestones & Issues GitHub
- **Milestone 1 : Socle Backend & Moteur d'Exécution en Rust** (#1 à #4)
- **Milestone 2 : Moteur Multi-Agent & Orchestration Dynamique** (#5 à #9)
- **Milestone 3 : Serveur API & Communication Temps Réel** (#10, #11)
- **Milestone 4 : Interface Utilisateur React** (#12 à #16)
- **Milestone 5 : Multiplateforme (Desktop, Mobile) & Déploiement** (#17 à #20)

---

## 🛠️ Conventions de Développement & Git
1. **Branches** :
   - Branche principale : `main`
   - Pour chaque issue : `feat/issue-<num>-<description>` ou `fix/issue-<num>-<description>`
2. **Messages de commit** :
   - Format conventionnel : `feat(scope): message (closes #<num>)`
   - Exemples : `feat(core): setup cargo workspace (closes #1)`, `feat(db): sqlite migrations with sqlx (closes #2)`
3. **Synchronisation Git** :
   - Le remote `origin` est déjà configuré avec les droits d'écriture.
   - Toujours s'assurer que les builds ou vérifications passent avant de pousser.
