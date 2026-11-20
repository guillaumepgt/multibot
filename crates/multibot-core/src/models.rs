use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Représente un projet géré par MultiBot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: String,
    pub nom: String,
    pub description: Option<String>,
    pub mode_autonome: bool,
    pub chemin_dossier: String,
    pub statut: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Données nécessaires à la création d'un nouveau projet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProject {
    pub nom: String,
    pub description: Option<String>,
    pub mode_autonome: bool,
    pub chemin_dossier: String,
    pub statut: Option<String>,
}

/// Données modifiables d'un projet existant.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProject {
    pub nom: Option<String>,
    pub description: Option<Option<String>>,
    pub mode_autonome: Option<bool>,
    pub chemin_dossier: Option<String>,
    pub statut: Option<String>,
}

/// Représente une tâche dans un projet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub titre: String,
    pub description: Option<String>,
    pub statut: String,
    pub assigne_a: Option<String>,
    pub ordre: i64,
    pub dependances: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Données pour créer une nouvelle tâche.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTask {
    pub project_id: String,
    pub titre: String,
    pub description: Option<String>,
    pub statut: Option<String>,
    pub assigne_a: Option<String>,
    pub ordre: Option<i64>,
    pub dependances: Option<String>,
}

/// Données modifiables d'une tâche.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTask {
    pub titre: Option<String>,
    pub description: Option<Option<String>>,
    pub statut: Option<String>,
    pub assigne_a: Option<Option<String>>,
    pub ordre: Option<i64>,
    pub dependances: Option<Option<String>>,
}

/// Représente une entrée de log d'un agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct AgentLog {
    pub id: String,
    pub project_id: String,
    pub task_id: Option<String>,
    pub agent_role: String,
    pub message: String,
    pub tokens_utilises: i64,
    pub timestamp: DateTime<Utc>,
}

/// Données pour créer un log d'agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAgentLog {
    pub project_id: String,
    pub task_id: Option<String>,
    pub agent_role: String,
    pub message: String,
    pub tokens_utilises: Option<i64>,
}

/// Représente la configuration d'un fournisseur de modèle LLM.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct ModelProvider {
    pub id: String,
    pub nom: String,
    pub type_api: String,
    pub endpoint_url: String,
    pub api_key_encryptee: Option<String>,
    pub model_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Données pour déclarer un fournisseur de modèle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewModelProvider {
    pub nom: String,
    pub type_api: String,
    pub endpoint_url: String,
    pub api_key_encryptee: Option<String>,
    pub model_name: String,
}

/// Données modifiables d'un fournisseur de modèle.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateModelProvider {
    pub nom: Option<String>,
    pub type_api: Option<String>,
    pub endpoint_url: Option<String>,
    pub api_key_encryptee: Option<Option<String>>,
    pub model_name: Option<String>,
}
