CREATE TABLE counter (
    id SERIAL PRIMARY KEY,
    count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE config (
    id SERIAL PRIMARY KEY,
    text_string TEXT NOT NULL,
    user_role TEXT NOT NULL
);