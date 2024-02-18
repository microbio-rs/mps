-- Add up migration script here
CREATE TABLE IF NOT EXISTS projects (
            id UUID PRIMARY KEY,
            user_id UUID NOT NULL,
            name VARCHAR NOT NULL,
            description VARCHAR NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL,
            updated_at TIMESTAMP WITH TIME ZONE NOT NULL
        )
