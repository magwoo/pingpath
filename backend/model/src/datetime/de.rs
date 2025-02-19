use chrono::NaiveDateTime;

use super::Datetime;

struct DatetimeVisitor;

impl serde::de::Visitor<'_> for DatetimeVisitor {
    type Value = Datetime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a date and time timestamp")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Datetime(
            NaiveDateTime::parse_from_str(v, Datetime::FMT).map_err(E::custom)?,
        ))
    }
}

impl<'de> serde::Deserialize<'de> for Datetime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DatetimeVisitor)
    }
}
