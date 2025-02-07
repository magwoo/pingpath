CREATE TABLE IF NOT EXISTS `users` (
    `id` INTEGER PRIMARY KEY,
    `username` TEXT NOT NULL DEFAULT('guest'),
    `github_token` TEXT,
    `icon_url` TEXT
);

CREATE TABLE IF NOT EXISTS `locations` (
    `id` INTEGER PRIMARY KEY,
    `addr` TEXT,
    `country` TEXT NOT NULL CHECK(length(country) == 2),
    `city` TEXT,
    `latitude` INTEGER NOT NULL,
    `longitude` INTEGER NOT NULL
);

