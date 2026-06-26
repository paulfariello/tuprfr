use crate::domain::model::Vote;
use crate::domain::ports::question_repository::{QuestionRepository, RepositoryError};
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
/// Returns a [`CastVoteError`] if the question or option is invalid, the
/// session has already voted, or the repository fails.
#[allow(clippy::unused_async)]
pub async fn cast_vote(
    _repo: &impl QuestionRepository,
    _question_id: Uuid,
    _option_id: Uuid,
    _session_id: Uuid,
) -> Result<Vote, CastVoteError> {
    todo!()
}
