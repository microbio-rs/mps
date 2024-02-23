CREATE TYPE service_kind AS ENUM (
    'application',
    'database'
);

CREATE TABLE IF NOT EXISTS services (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    project_id UUID NOT NULL,
    environment_id UUID NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR DEFAULT NULL,
    kind service_kind NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL)

