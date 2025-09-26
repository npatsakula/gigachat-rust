pub mod string_to_usize {
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S>(value: &usize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsizeVisitor;

        impl<'de> serde::de::Visitor<'de> for UsizeVisitor {
            type Value = usize;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing a number")
            }

            fn visit_str<E>(self, value: &str) -> Result<usize, E>
            where
                E: serde::de::Error,
            {
                value.parse::<usize>().map_err(E::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<usize, E>
            where
                E: serde::de::Error,
            {
                Ok(value as usize)
            }
        }

        deserializer.deserialize_any(UsizeVisitor)
    }
}

pub mod string_json {
    use serde::{Deserialize, Deserializer, Serializer, de::Error};
    use serde_json::Value;

    /// Deserializes a string-encoded JSON into a `serde_json::Value`.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First, deserialize the incoming data into a string.
        let s: &str = Deserialize::deserialize(deserializer)?;
        // Then, parse the string into a serde_json::Value.
        serde_json::from_str(s).map_err(D::Error::custom)
    }

    /// Serializes a `serde_json::Value` into a string-encoded JSON.
    pub fn serialize<S>(value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert the Value to a string.
        let s = value.to_string();
        // Serialize the string.
        serializer.serialize_str(&s)
    }
}
