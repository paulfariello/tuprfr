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
/// Returns a [`SkipError`] if the question does not exist or the repository fails.
#[allow(clippy::unused_async)]
pub async fn skip_question(
    _repo: &impl QuestionRepository,
    _question_id: Uuid,
) -> Result<(), SkipError> {
    todo!()
}
