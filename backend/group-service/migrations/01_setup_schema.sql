CREATE TABLE IF NOT EXISTS groups (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL,
    channel_id UUID NOT NULL,
    created_at TIMESTAMP DEFAULT now()
);

CREATE TABLE IF NOT EXISTS group_members (
    group_id UUID REFERENCES groups(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    PRIMARY KEY (group_id, user_id)
);
