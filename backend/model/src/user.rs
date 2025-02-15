use chrono::NaiveDateTime;

use self::github::UserGithub;

pub mod github;

pub struct User {
    id: i64,
    username: String,
    created_at: NaiveDateTime,
    github: UserGithub,
}
