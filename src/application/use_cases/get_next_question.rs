use crate::domain::model::QuestionWithOptions;
use crate::domain::ports::question_repository::{QuestionRepository, RepositoryError};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum GetNextQuestionError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

/// # Errors
///
/// Returns [`GetNextQuestionError::Repository`] if the repository query fails.
#[allow(clippy::implicit_hasher)]
pub async fn get_next_question(
    repo: &impl QuestionRepository,
    seen_ids: &HashSet<Uuid>,
) -> Result<Option<QuestionWithOptions>, GetNextQuestionError> {
    Ok(repo.random_published_excluding(seen_ids).await?)
}
