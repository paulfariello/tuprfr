use crate::domain::model::{Question, QuestionOption, Status, SubmissionMode};
use crate::domain::ports::question_repository::{QuestionRepository, RepositoryError};
use chrono::Utc;
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
/// Returns [`SubmitError::EmptyOption`] if either option text is blank, or [`SubmitError::Repository`] on DB failure.
pub async fn submit_question(
    repo: &impl QuestionRepository,
    text_a: String,
    text_b: String,
    author_session_id: Uuid,
    submission_mode: SubmissionMode,
) -> Result<Question, SubmitError> {
    if text_a.trim().is_empty() || text_b.trim().is_empty() {
        return Err(SubmitError::EmptyOption);
    }
    let option_a = QuestionOption {
        id: Uuid::new_v4(),
        text: text_a,
    };
    let option_b = QuestionOption {
        id: Uuid::new_v4(),
        text: text_b,
    };
    let status = if submission_mode == SubmissionMode::Open {
        Status::Published
    } else {
        Status::Pending
    };
    let question = Question {
        id: Uuid::new_v4(),
        option_a_id: option_a.id,
        option_b_id: option_b.id,
        status,
        author_session_id,
        is_anonymous: true,
        created_at: Utc::now(),
    };
    repo.save(&question, &option_a, &option_b).await?;
    Ok(question)
}
