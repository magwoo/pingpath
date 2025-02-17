use chrono::NaiveDateTime;
use rqlite_rs::query::arguments::{RqliteArgument, RqliteArgumentRaw};
use std::fmt;
use std::ops::{Deref, DerefMut};

mod de;
mod se;

#[derive(Debug, Clone, Copy)]
pub struct Datetime(NaiveDateTime);

impl Datetime {
    const FMT: &str = "%Y-%m-%dT%H:%M:%SZ";

    pub fn new(dt: NaiveDateTime) -> Self {
        Self(dt)
    }

    pub fn now() -> Self {
        Self(chrono::Utc::now().naive_utc())
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.format(Self::FMT).to_string().as_str())
    }
}

impl Deref for Datetime {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Datetime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RqliteArgumentRaw for Datetime {
    fn encode(&self) -> RqliteArgument {
        RqliteArgument::String(self.to_string())
    }
}
