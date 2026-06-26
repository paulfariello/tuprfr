use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Question {
    pub id: Uuid,
    pub option_a_id: Uuid,
    pub option_b_id: Uuid,
    pub status: Status,
    pub author_session_id: Uuid,
    pub is_anonymous: bool,
    pub created_at: DateTime<Utc>,
}

/// Named `QuestionOption` to avoid shadowing `std::option::Option`.
#[derive(Debug, Clone)]
pub struct QuestionOption {
    pub id: Uuid,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct QuestionWithOptions {
    pub question: Question,
    pub option_a: QuestionOption,
    pub option_b: QuestionOption,
}

#[derive(Debug, Clone)]
pub struct Vote {
    pub id: Uuid,
    pub question_id: Uuid,
    pub session_id: Uuid,
    pub option_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Pending,
    Published,
    Hidden,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Pending => write!(f, "pending"),
            Status::Published => write!(f, "published"),
            Status::Hidden => write!(f, "hidden"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmissionMode {
    Open,
    Moderated,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionData {
    pub id: Option<Uuid>,
    pub seen_question_ids: HashSet<Uuid>,
    pub submission_timestamps: Vec<DateTime<Utc>>,
}
