CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS levels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255),
    description VARCHAR(2000),
    official BOOLEAN,
    commended BOOLEAN,
    version INTEGER,
    solution INTEGER[],
    key VARCHAR(255)[],
    map JSONB,
    size INTEGER[],
    spawn INTEGER[],
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
