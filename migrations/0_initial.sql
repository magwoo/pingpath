CREATE TABLE IF NOT EXISTS `users` (
    `id` INTEGER PRIMARY KEY,
    `username` TEXT NOT NULL DEFAULT('guest'),
    `github_token` TEXT,
    `img_url` TEXT,
    `created_at` DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS `locations` (
    `id` INTEGER PRIMARY KEY,
    `addr` TEXT,
    `country` TEXT NOT NULL CHECK(length(country) == 2),
    `city` TEXT,
    `latitude` INTEGER NOT NULL,
    `longitude` INTEGER NOT NULL,
    `created_at` DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS `user_addrs` (
    `id` INTEGER PRIMARY KEY,
    `name` TEXT,
    `addr` TEXT NOT NULL,
    `user_id` INTEGER NOT NULL,
    `created_at` DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS `pings` (
    `id` INTEGER PRIMARY KEY,
    `addr_id` INTEGER NOT NULL,
    `latency` TEXT NOT NULL,
    `started_at` DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS `sessions` (
    `id` INTEGER PRIMARY KEY,
    `user_id` INTEGER NOT NULL,
    `token` TEXT NOT NULL,
    `created_at` DATETIME NOT NULL,
    `expire_date` DATETIME NOT NULL
);

