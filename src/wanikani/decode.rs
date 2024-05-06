use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

pub fn from_rfc3339<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.into())
        .map_err(|e| serde::de::Error::custom(e.to_string()))
}

pub fn from_rfc3339_option<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    s.map(|s| {
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.into())
            .map_err(|e| serde::de::Error::custom(e.to_string()))
    })
    .transpose()
}
