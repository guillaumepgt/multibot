use chrono::Utc;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use std::str::FromStr;
use uuid::Uuid;

use crate::error::{CoreError, Result};
use crate::models::{
    AgentLog, ModelProvider, NewAgentLog, NewModelProvider, NewProject, NewTask, Project, Task,
    UpdateModelProvider, UpdateProject, UpdateTask,
};

/// Couche d'accès aux données SQLite pour MultiBot.
#[derive(Clone, Debug)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    /// Initialise une connexion SQLite vers un fichier ou une URL de connexion.
    pub async fn connect(database_url: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(database_url)
            .map_err(CoreError::Database)?
            .create_if_missing(true)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    /// Crée une base de données SQLite volatile en mémoire (idéal pour les tests).
    pub async fn connect_in_memory() -> Result<Self> {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")
            .map_err(CoreError::Database)?
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    /// Exécute les migrations SQL initiales.
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(CoreError::Migration)
    }

    /// Retourne une référence vers le pool de connexions sous-jacent.
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    // =========================================================================
    // CRUD Projets
    // =========================================================================

    pub async fn create_project(&self, new_proj: NewProject) -> Result<Project> {
        let id = Uuid::new_v4().to_string();
        let statut = new_proj.statut.unwrap_or_else(|| "active".to_string());
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO projects (id, nom, description, mode_autonome, chemin_dossier, statut, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
            "#,
        )
        .bind(&id)
        .bind(&new_proj.nom)
        .bind(&new_proj.description)
        .bind(new_proj.mode_autonome)
        .bind(&new_proj.chemin_dossier)
        .bind(&statut)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.get_project(&id)
            .await?
            .ok_or_else(|| CoreError::NotFound(format!("Project {}", id)))
    }

    pub async fn get_project(&self, id: &str) -> Result<Option<Project>> {
        let proj = sqlx::query_as::<_, Project>(
            r#"
            SELECT id, nom, description, mode_autonome, chemin_dossier, statut, created_at, updated_at
            FROM projects
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(proj)
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        let projects = sqlx::query_as::<_, Project>(
            r#"
            SELECT id, nom, description, mode_autonome, chemin_dossier, statut, created_at, updated_at
            FROM projects
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(projects)
    }

    pub async fn update_project(
        &self,
        id: &str,
        update: UpdateProject,
    ) -> Result<Option<Project>> {
        let current = match self.get_project(id).await? {
            Some(p) => p,
            None => return Ok(None),
        };

        let nom = update.nom.unwrap_or(current.nom);
        let description = match update.description {
            Some(desc) => desc,
            None => current.description,
        };
        let mode_autonome = update.mode_autonome.unwrap_or(current.mode_autonome);
        let chemin_dossier = update.chemin_dossier.unwrap_or(current.chemin_dossier);
        let statut = update.statut.unwrap_or(current.statut);
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE projects
            SET nom = ?1, description = ?2, mode_autonome = ?3, chemin_dossier = ?4, statut = ?5, updated_at = ?6
            WHERE id = ?7
            "#,
        )
        .bind(nom)
        .bind(description)
        .bind(mode_autonome)
        .bind(chemin_dossier)
        .bind(statut)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_project(id).await
    }

    pub async fn delete_project(&self, id: &str) -> Result<bool> {
        let res = sqlx::query("DELETE FROM projects WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    // =========================================================================
    // CRUD Tâches
    // =========================================================================

    pub async fn create_task(&self, new_task: NewTask) -> Result<Task> {
        let id = Uuid::new_v4().to_string();
        let statut = new_task.statut.unwrap_or_else(|| "pending".to_string());
        let ordre = new_task.ordre.unwrap_or(0);
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO tasks (id, project_id, titre, description, statut, assigne_a, ordre, dependances, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)
            "#,
        )
        .bind(&id)
        .bind(&new_task.project_id)
        .bind(&new_task.titre)
        .bind(&new_task.description)
        .bind(&statut)
        .bind(&new_task.assigne_a)
        .bind(ordre)
        .bind(&new_task.dependances)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.get_task(&id)
            .await?
            .ok_or_else(|| CoreError::NotFound(format!("Task {}", id)))
    }

    pub async fn get_task(&self, id: &str) -> Result<Option<Task>> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, project_id, titre, description, statut, assigne_a, ordre, dependances, created_at, updated_at
            FROM tasks
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    pub async fn list_tasks_by_project(&self, project_id: &str) -> Result<Vec<Task>> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, project_id, titre, description, statut, assigne_a, ordre, dependances, created_at, updated_at
            FROM tasks
            WHERE project_id = ?1
            ORDER BY ordre ASC, created_at ASC
            "#,
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    pub async fn update_task(
        &self,
        id: &str,
        update: UpdateTask,
    ) -> Result<Option<Task>> {
        let current = match self.get_task(id).await? {
            Some(t) => t,
            None => return Ok(None),
        };

        let titre = update.titre.unwrap_or(current.titre);
        let description = match update.description {
            Some(desc) => desc,
            None => current.description,
        };
        let statut = update.statut.unwrap_or(current.statut);
        let assigne_a = match update.assigne_a {
            Some(a) => a,
            None => current.assigne_a,
        };
        let ordre = update.ordre.unwrap_or(current.ordre);
        let dependances = match update.dependances {
            Some(deps) => deps,
            None => current.dependances,
        };
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE tasks
            SET titre = ?1, description = ?2, statut = ?3, assigne_a = ?4, ordre = ?5, dependances = ?6, updated_at = ?7
            WHERE id = ?8
            "#,
        )
        .bind(titre)
        .bind(description)
        .bind(statut)
        .bind(assigne_a)
        .bind(ordre)
        .bind(dependances)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_task(id).await
    }

    pub async fn delete_task(&self, id: &str) -> Result<bool> {
        let res = sqlx::query("DELETE FROM tasks WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    // =========================================================================
    // CRUD Agent Logs
    // =========================================================================

    pub async fn create_agent_log(&self, new_log: NewAgentLog) -> Result<AgentLog> {
        let id = Uuid::new_v4().to_string();
        let tokens = new_log.tokens_utilises.unwrap_or(0);
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO agent_logs (id, project_id, task_id, agent_role, message, tokens_utilises, timestamp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
        )
        .bind(&id)
        .bind(&new_log.project_id)
        .bind(&new_log.task_id)
        .bind(&new_log.agent_role)
        .bind(&new_log.message)
        .bind(tokens)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let log = sqlx::query_as::<_, AgentLog>(
            r#"
            SELECT id, project_id, task_id, agent_role, message, tokens_utilises, timestamp
            FROM agent_logs
            WHERE id = ?1
            "#,
        )
        .bind(&id)
        .fetch_one(&self.pool)
        .await?;

        Ok(log)
    }

    pub async fn list_logs_by_project(&self, project_id: &str) -> Result<Vec<AgentLog>> {
        let logs = sqlx::query_as::<_, AgentLog>(
            r#"
            SELECT id, project_id, task_id, agent_role, message, tokens_utilises, timestamp
            FROM agent_logs
            WHERE project_id = ?1
            ORDER BY timestamp ASC
            "#,
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }

    pub async fn list_logs_by_task(&self, task_id: &str) -> Result<Vec<AgentLog>> {
        let logs = sqlx::query_as::<_, AgentLog>(
            r#"
            SELECT id, project_id, task_id, agent_role, message, tokens_utilises, timestamp
            FROM agent_logs
            WHERE task_id = ?1
            ORDER BY timestamp ASC
            "#,
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }

    // =========================================================================
    // CRUD Model Providers
    // =========================================================================

    pub async fn create_provider(
        &self,
        new_provider: NewModelProvider,
    ) -> Result<ModelProvider> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO model_providers (id, nom, type_api, endpoint_url, api_key_encryptee, model_name, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
            "#,
        )
        .bind(&id)
        .bind(&new_provider.nom)
        .bind(&new_provider.type_api)
        .bind(&new_provider.endpoint_url)
        .bind(&new_provider.api_key_encryptee)
        .bind(&new_provider.model_name)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.get_provider(&id)
            .await?
            .ok_or_else(|| CoreError::NotFound(format!("ModelProvider {}", id)))
    }

    pub async fn get_provider(&self, id: &str) -> Result<Option<ModelProvider>> {
        let provider = sqlx::query_as::<_, ModelProvider>(
            r#"
            SELECT id, nom, type_api, endpoint_url, api_key_encryptee, model_name, created_at, updated_at
            FROM model_providers
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(provider)
    }

    pub async fn list_providers(&self) -> Result<Vec<ModelProvider>> {
        let providers = sqlx::query_as::<_, ModelProvider>(
            r#"
            SELECT id, nom, type_api, endpoint_url, api_key_encryptee, model_name, created_at, updated_at
            FROM model_providers
            ORDER BY created_at ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(providers)
    }

    pub async fn update_provider(
        &self,
        id: &str,
        update: UpdateModelProvider,
    ) -> Result<Option<ModelProvider>> {
        let current = match self.get_provider(id).await? {
            Some(p) => p,
            None => return Ok(None),
        };

        let nom = update.nom.unwrap_or(current.nom);
        let type_api = update.type_api.unwrap_or(current.type_api);
        let endpoint_url = update.endpoint_url.unwrap_or(current.endpoint_url);
        let api_key_encryptee = match update.api_key_encryptee {
            Some(key) => key,
            None => current.api_key_encryptee,
        };
        let model_name = update.model_name.unwrap_or(current.model_name);
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE model_providers
            SET nom = ?1, type_api = ?2, endpoint_url = ?3, api_key_encryptee = ?4, model_name = ?5, updated_at = ?6
            WHERE id = ?7
            "#,
        )
        .bind(nom)
        .bind(type_api)
        .bind(endpoint_url)
        .bind(api_key_encryptee)
        .bind(model_name)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_provider(id).await
    }

    pub async fn delete_provider(&self, id: &str) -> Result<bool> {
        let res = sqlx::query("DELETE FROM model_providers WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_initialization_and_migrations() {
        let db = Database::connect_in_memory()
            .await
            .expect("Failed to create in-memory SQLite");
        db.run_migrations().await.expect("Failed to run migrations");
    }

    #[tokio::test]
    async fn test_project_crud() {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();

        // 1. Create
        let project = db
            .create_project(NewProject {
                nom: "Test Project".to_string(),
                description: Some("Description test".to_string()),
                mode_autonome: true,
                chemin_dossier: "/tmp/test".to_string(),
                statut: Some("active".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(project.nom, "Test Project");
        assert!(project.mode_autonome);

        // 2. Read
        let fetched = db.get_project(&project.id).await.unwrap().unwrap();
        assert_eq!(fetched.id, project.id);

        let list = db.list_projects().await.unwrap();
        assert_eq!(list.len(), 1);

        // 3. Update
        let updated = db
            .update_project(
                &project.id,
                UpdateProject {
                    nom: Some("Updated Project".to_string()),
                    statut: Some("completed".to_string()),
                    ..Default::default()
                },
            )
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated.nom, "Updated Project");
        assert_eq!(updated.statut, "completed");

        // 4. Delete
        let deleted = db.delete_project(&project.id).await.unwrap();
        assert!(deleted);
        assert!(db.get_project(&project.id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_task_crud() {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();

        let proj = db
            .create_project(NewProject {
                nom: "Proj with tasks".to_string(),
                description: None,
                mode_autonome: false,
                chemin_dossier: "/workspace".to_string(),
                statut: None,
            })
            .await
            .unwrap();

        let task = db
            .create_task(NewTask {
                project_id: proj.id.clone(),
                titre: "Tâche 1".to_string(),
                description: Some("Faire le backend".to_string()),
                statut: Some("pending".to_string()),
                assigne_a: Some("dev".to_string()),
                ordre: Some(1),
                dependances: None,
            })
            .await
            .unwrap();

        assert_eq!(task.titre, "Tâche 1");
        assert_eq!(task.statut, "pending");

        let tasks = db.list_tasks_by_project(&proj.id).await.unwrap();
        assert_eq!(tasks.len(), 1);

        let updated_task = db
            .update_task(
                &task.id,
                UpdateTask {
                    statut: Some("done".to_string()),
                    ..Default::default()
                },
            )
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated_task.statut, "done");

        let deleted = db.delete_task(&task.id).await.unwrap();
        assert!(deleted);
    }

    #[tokio::test]
    async fn test_agent_logs() {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();

        let proj = db
            .create_project(NewProject {
                nom: "Proj logs".to_string(),
                description: None,
                mode_autonome: false,
                chemin_dossier: "/tmp".to_string(),
                statut: None,
            })
            .await
            .unwrap();

        let log = db
            .create_agent_log(NewAgentLog {
                project_id: proj.id.clone(),
                task_id: None,
                agent_role: "scrum_master".to_string(),
                message: "Planning sprint".to_string(),
                tokens_utilises: Some(150),
            })
            .await
            .unwrap();

        assert_eq!(log.agent_role, "scrum_master");
        assert_eq!(log.tokens_utilises, 150);

        let logs = db.list_logs_by_project(&proj.id).await.unwrap();
        assert_eq!(logs.len(), 1);
    }

    #[tokio::test]
    async fn test_model_providers() {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();

        let prov = db
            .create_provider(NewModelProvider {
                nom: "Ollama Local".to_string(),
                type_api: "ollama".to_string(),
                endpoint_url: "http://localhost:11434".to_string(),
                api_key_encryptee: None,
                model_name: "llama3".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(prov.nom, "Ollama Local");

        let providers = db.list_providers().await.unwrap();
        assert_eq!(providers.len(), 1);

        let updated = db
            .update_provider(
                &prov.id,
                UpdateModelProvider {
                    model_name: Some("qwen2.5-coder".to_string()),
                    ..Default::default()
                },
            )
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated.model_name, "qwen2.5-coder");

        let deleted = db.delete_provider(&prov.id).await.unwrap();
        assert!(deleted);
    }
}
