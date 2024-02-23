CREATE TYPE environment_mode AS ENUM (
    'development',
    'production',
    'staging'
);

CREATE TABLE IF NOT EXISTS environments (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    project_id UUID NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR DEFAULT NULL,
    mode environment_mode NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL)

