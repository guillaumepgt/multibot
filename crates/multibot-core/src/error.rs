use thiserror::Error;

/// Erreurs principales du moteur MultiBot Core.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Erreur de base de données : {0}")]
    Database(#[from] sqlx::Error),

    #[error("Erreur de migration de base de données : {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Entité non trouvée : {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
