use crate::domain::ports::question_repository::{QuestionRepository, RepositoryError};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum SkipError {
    #[error("question not found")]
    QuestionNotFound,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

/// # Errors
///
/// No current error paths; `Result` kept for interface consistency with other use cases.
#[allow(clippy::unused_async)]
pub async fn skip_question(
    _repo: &impl QuestionRepository,
    _question_id: Uuid,
) -> Result<(), SkipError> {
    Ok(())
}
