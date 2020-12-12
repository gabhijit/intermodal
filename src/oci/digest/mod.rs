// Implementation of Digest type

use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::string::String;

#[derive(PartialEq, Eq, Debug)]
pub struct Digest {
    algorithm: String,
    hex_digest: String,
}

struct DigestVisitor;

impl<'de> Visitor<'de> for DigestVisitor {
    type Value = Digest;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string representing an OCI Digest.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let tokens: Vec<&str> = value.split(":").collect();

        if tokens.len() != 2 {
            return Err(de::Error::custom("Invalid value: "));
        }

        Ok(Digest {
            algorithm: String::from(tokens[0]),
            hex_digest: String::from(tokens[1]),
        })
    }
}

impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DigestVisitor)
    }
}

impl Serialize for Digest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut out = String::from(&self.algorithm);
        out.push(':');
        out.push_str(&self.hex_digest);

        serializer.serialize_str(&out)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serialize() {
        let d = Digest {
            algorithm: String::from("sha256"),
            hex_digest: String::from("deadbeef"),
        };
        let output = serde_json::to_string(&d).unwrap();

        assert_eq!(output, "\"sha256:deadbeef\"");
    }

    #[test]
    fn test_deserialize_valid() {
        let d: Digest = serde_json::from_str("\"sha256:deadbeef\"").unwrap();

        assert_eq!(d.algorithm, "sha256");
        assert_eq!(d.hex_digest, "deadbeef");
    }

    #[test]
    fn test_deserialize_invalid() {
        let res = serde_json::from_str::<Digest>("\"deadbeef\"");

        assert!(res.is_err());
    }
}
