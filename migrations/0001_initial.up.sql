CREATE TABLE options (
    id   UUID NOT NULL PRIMARY KEY,
    text TEXT NOT NULL
);

CREATE TABLE questions (
    id                UUID        NOT NULL PRIMARY KEY,
    option_a_id       UUID        NOT NULL REFERENCES options(id),
    option_b_id       UUID        NOT NULL REFERENCES options(id),
    status            TEXT        NOT NULL DEFAULT 'pending',
    author_session_id UUID        NOT NULL,
    is_anonymous      BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at        TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_questions_status ON questions(status);

CREATE TABLE votes (
    id          UUID        NOT NULL PRIMARY KEY,
    question_id UUID        NOT NULL REFERENCES questions(id),
    session_id  UUID        NOT NULL,
    option_id   UUID        NOT NULL REFERENCES options(id),
    created_at  TIMESTAMPTZ NOT NULL,
    UNIQUE (question_id, session_id)
);

CREATE INDEX idx_votes_question_id ON votes(question_id);

CREATE TABLE settings (
    key   TEXT NOT NULL PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT INTO settings (key, value) VALUES ('submission_mode', 'open');
