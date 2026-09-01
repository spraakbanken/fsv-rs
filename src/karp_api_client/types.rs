use chrono::{DateTime, Utc};
use serde::de;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SearchQuery {
    total: u64,
    hits: Vec<SearchQueryHit>,
    distribution: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SearchQueryHit {
    resource: String,
    id: ulid::Ulid,
    // hits: Vec<SearchQueryHit>
    entry: serde_json::Value,
    last_modified_by: String,
    #[serde(deserialize_with = "crate::karp_api_client::types::from_timestamp")]
    last_modified: DateTime<Utc>,
    version: u64,
}

pub fn from_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_f64(F64TimestampVisitor)
}

struct F64TimestampVisitor;

impl de::Visitor<'_> for F64TimestampVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a unix timestamp as f64")
    }

    /// Deserialize a timestamp in
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let secs = v.floor();
        let nsecs = ((v - secs) * 1e9) as u32;
        DateTime::from_timestamp(secs as i64, nsecs)
            .ok_or_else(|| E::custom(format!("Can't parse timestamp from '{v}'")))
    }
}
