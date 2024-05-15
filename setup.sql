CREATE TABLE IF NOT EXISTS server_settings (
    guild_id TEXT PRIMARY KEY,
    warnings INTEGER NOT NULL,
    mute_time TEXT NOT NULL,
    use_warnings BOOLEAN NOT NULL,
    sensitivity REAL NOT NULL,
    local_language TEXT DEFAULT 'en',
    mute_enabled BOOLEAN NOT NULL,
    logs_channel_id TEXT
);
