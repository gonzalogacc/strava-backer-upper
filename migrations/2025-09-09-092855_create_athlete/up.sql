CREATE TABLE athletes (
    id bigint PRIMARY KEY,
    username TEXT,
    firstname TEXT,
    lastname TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
)