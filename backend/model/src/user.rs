use chrono::NaiveDateTime;

use self::github::UserGithub;

pub mod github;

pub struct User {
    id: i64,
    username: String,
    create_date: NaiveDateTime,
    github: UserGithub,
}
