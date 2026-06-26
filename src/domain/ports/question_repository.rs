use crate::domain::model::{Question, QuestionOption, QuestionWithOptions, SubmissionMode, Vote};
use async_trait::async_trait;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("database error: {0}")]
    Database(String),
    #[error("not found")]
    NotFound,
}

#[async_trait]
pub trait QuestionRepository {
    async fn random_published_excluding(
        &self,
        seen_ids: &HashSet<Uuid>,
    ) -> Result<Option<QuestionWithOptions>, RepositoryError>;

    async fn save(
        &self,
        question: &Question,
        option_a: &QuestionOption,
        option_b: &QuestionOption,
    ) -> Result<(), RepositoryError>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<QuestionWithOptions>, RepositoryError>;

    async fn vote_counts(&self, question_id: Uuid) -> Result<(u64, u64), RepositoryError>;

    async fn record_vote(&self, vote: &Vote) -> Result<(), RepositoryError>;

    async fn submission_mode(&self) -> Result<SubmissionMode, RepositoryError>;
}
