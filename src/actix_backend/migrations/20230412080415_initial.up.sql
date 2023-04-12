CREATE TABLE IF NOT EXISTS "users" (
    "id"               SERIAL PRIMARY KEY,
    "username"         VARCHAR(50) NOT NULL,
    "password"         TEXT NOT NULL,
    "created_at"       TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "allow_login"      BOOLEAN NOT NULL DEFAULT TRUE,
    "is_staff"         BOOLEAN NOT NULL DEFAULT FALSE,
    "is_superuser"     BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS "message" (
    "id"         SERIAL PRIMARY KEY,
    "author_id"  INTEGER,
    "content"    VARCHAR(500),
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,

    FOREIGN KEY("author_id") REFERENCES users(id)
)
