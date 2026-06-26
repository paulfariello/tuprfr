use crate::adapters::inbound::http::error::AppError;
use crate::adapters::outbound::db::question_repo::SqlxQuestionRepository;
use crate::application::use_cases::get_next_question::get_next_question;
use crate::domain::model::SessionData;
use crate::AppState;
use askama::Template;
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use tower_sessions::Session;
use uuid::Uuid;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_next_question_handler))
        .route("/votes", post(post_vote_handler))
        .route("/skips", post(post_skip_handler))
        .route("/questions/new", get(get_new_question_form_handler))
        .route("/questions", post(post_question_handler))
        .route("/questions/:id/results", get(get_question_results_handler))
        .with_state(state)
}

#[derive(Template)]
#[template(path = "question.html")]
struct QuestionTemplate {
    question_id: Uuid,
    option_a_id: Uuid,
    option_a_text: String,
    option_b_id: Uuid,
    option_b_text: String,
}

#[derive(Template)]
#[template(path = "pool_exhaustion.html")]
struct PoolExhaustionTemplate;

#[derive(serde::Deserialize)]
struct SkipForm {
    question_id: Uuid,
}

const SESSION_KEY: &str = "data";

async fn get_next_question_handler(
    State(state): State<AppState>,
    session: Session,
) -> Result<Response, AppError> {
    let mut session_data: SessionData = session
        .get(SESSION_KEY)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .unwrap_or_default();

    if session_data.id.is_none() {
        session_data.id = Some(Uuid::new_v4());
        session
            .insert(SESSION_KEY, &session_data)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let repo = SqlxQuestionRepository::new(state.db);

    match get_next_question(&repo, &session_data.seen_question_ids).await? {
        Some(q) => Ok(QuestionTemplate {
            question_id: q.question.id,
            option_a_id: q.option_a.id,
            option_a_text: q.option_a.text,
            option_b_id: q.option_b.id,
            option_b_text: q.option_b.text,
        }
        .into_response()),
        None => Ok(PoolExhaustionTemplate.into_response()),
    }
}

async fn post_vote_handler(State(_state): State<AppState>, _session: Session) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn post_skip_handler(
    State(_state): State<AppState>,
    session: Session,
    Form(form): Form<SkipForm>,
) -> Result<impl IntoResponse, AppError> {
    let mut session_data: SessionData = session
        .get(SESSION_KEY)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .unwrap_or_default();
    session_data.seen_question_ids.insert(form.question_id);
    session
        .insert(SESSION_KEY, &session_data)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Redirect::to("/"))
}

async fn get_new_question_form_handler(
    State(_state): State<AppState>,
    _session: Session,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn post_question_handler(State(_state): State<AppState>, _session: Session) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn get_question_results_handler(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
    _session: Session,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}
