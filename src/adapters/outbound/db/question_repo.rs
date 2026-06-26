use crate::domain::model::{Question, QuestionOption, QuestionWithOptions, Status, Vote};
use crate::domain::ports::question_repository::{QuestionRepository, RepositoryError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

pub struct SqlxQuestionRepository {
    pool: PgPool,
}

impl SqlxQuestionRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct QuestionWithOptionsRow {
    question_id: Uuid,
    option_a_id: Uuid,
    option_b_id: Uuid,
    status: String,
    author_session_id: Uuid,
    is_anonymous: bool,
    created_at: DateTime<Utc>,
    opt_a_id: Uuid,
    opt_a_text: String,
    opt_b_id: Uuid,
    opt_b_text: String,
}

impl From<QuestionWithOptionsRow> for QuestionWithOptions {
    fn from(r: QuestionWithOptionsRow) -> Self {
        let status = match r.status.as_str() {
            "published" => Status::Published,
            "pending" => Status::Pending,
            _ => Status::Hidden,
        };
        QuestionWithOptions {
            question: Question {
                id: r.question_id,
                option_a_id: r.option_a_id,
                option_b_id: r.option_b_id,
                status,
                author_session_id: r.author_session_id,
                is_anonymous: r.is_anonymous,
                created_at: r.created_at,
            },
            option_a: QuestionOption {
                id: r.opt_a_id,
                text: r.opt_a_text,
            },
            option_b: QuestionOption {
                id: r.opt_b_id,
                text: r.opt_b_text,
            },
        }
    }
}

#[async_trait]
impl QuestionRepository for SqlxQuestionRepository {
    async fn random_published_excluding(
        &self,
        seen_ids: &HashSet<Uuid>,
    ) -> Result<Option<QuestionWithOptions>, RepositoryError> {
        let seen: Vec<Uuid> = seen_ids.iter().copied().collect();
        sqlx::query_as::<_, QuestionWithOptionsRow>(
            "SELECT q.id AS question_id,
                    q.option_a_id,
                    q.option_b_id,
                    q.status,
                    q.author_session_id,
                    q.is_anonymous,
                    q.created_at,
                    oa.id   AS opt_a_id,
                    oa.text AS opt_a_text,
                    ob.id   AS opt_b_id,
                    ob.text AS opt_b_text
             FROM questions q
             JOIN options oa ON q.option_a_id = oa.id
             JOIN options ob ON q.option_b_id = ob.id
             WHERE q.status = 'published'
               AND NOT (q.id = ANY($1))
             ORDER BY RANDOM()
             LIMIT 1",
        )
        .bind(&seen)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::Database(e.to_string()))
        .map(|opt| opt.map(QuestionWithOptions::from))
    }

    async fn save(
        &self,
        _question: &Question,
        _option_a: &QuestionOption,
        _option_b: &QuestionOption,
    ) -> Result<(), RepositoryError> {
        todo!()
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<QuestionWithOptions>, RepositoryError> {
        todo!()
    }

    async fn vote_counts(&self, _question_id: Uuid) -> Result<(u64, u64), RepositoryError> {
        todo!()
    }

    async fn record_vote(&self, _vote: &Vote) -> Result<(), RepositoryError> {
        todo!()
    }
}
