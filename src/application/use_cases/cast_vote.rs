use crate::domain::model::Vote;
use crate::domain::ports::question_repository::{QuestionRepository, RepositoryError};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum CastVoteError {
    #[error("question not found")]
    QuestionNotFound,
    #[error("already voted on this question")]
    AlreadyVoted,
    #[error("invalid option for this question")]
    InvalidOption,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

/// # Errors
///
/// Returns [`CastVoteError`] if the question is not found, the option is invalid, or the repository fails.
pub async fn cast_vote(
    repo: &impl QuestionRepository,
    question_id: Uuid,
    option_id: Uuid,
    session_id: Uuid,
) -> Result<Vote, CastVoteError> {
    let q = repo
        .find_by_id(question_id)
        .await?
        .ok_or(CastVoteError::QuestionNotFound)?;
    if option_id != q.question.option_a_id && option_id != q.question.option_b_id {
        return Err(CastVoteError::InvalidOption);
    }
    let vote = Vote {
        id: uuid::Uuid::new_v4(),
        question_id,
        session_id,
        option_id,
        created_at: Utc::now(),
    };
    repo.record_vote(&vote).await?;
    Ok(vote)
}
