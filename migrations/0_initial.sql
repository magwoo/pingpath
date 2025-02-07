CREATE TABLE IF NOT EXISTS `users` (
    `id` INTEGER PRIMARY KEY,
    `username` TEXT NOT NULL DEFAULT('guest'),
    `github_token` TEXT,
    `icon_url` TEXT
);
