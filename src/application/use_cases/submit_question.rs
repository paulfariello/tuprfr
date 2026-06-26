use crate::domain::model::{Question, SubmissionMode};
use crate::domain::ports::question_repository::{QuestionRepository, RepositoryError};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum SubmitError {
    #[error("option text must not be empty")]
    EmptyOption,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

/// # Errors
///
/// Returns a [`SubmitError`] if an option text is empty or the repository fails.
#[allow(clippy::unused_async)]
pub async fn submit_question(
    _repo: &impl QuestionRepository,
    _option_a_text: String,
    _option_b_text: String,
    _author_session_id: Uuid,
    _submission_mode: SubmissionMode,
) -> Result<Question, SubmitError> {
    todo!()
}
