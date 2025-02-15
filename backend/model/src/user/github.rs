use rqlite_rs::FromRow;

pub struct UserGithub {
    token: String,
    img_url: String,
}

impl FromRow for UserGithub {
    fn from_row(row: rqlite_rs::Row) -> Result<Self, rqlite_rs::IntoTypedError> {
        Ok(Self {
            token: row.get("token")?,
            img_url: row.get("img_url")?,
        })
    }
}
