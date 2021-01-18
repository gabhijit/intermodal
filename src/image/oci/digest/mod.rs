//! Implementation of OCI Digest type
//!
//! # Reference:
//! [Digest implementation in go](https://github.com/opencontainers/go-digest/)
//!
//! # Note:
//!
//! We are not implementing (or rather have not implemented yet, everything from the Go module
//! above, but we'll do so as needed basis.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::string::String;

use futures_core::stream::Stream;
use futures_util::StreamExt;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Digest {
    algorithm: String,
    hex_digest: String,
}

impl Default for Digest {
    fn default() -> Self {
        Digest {
            algorithm: "sha256".to_string(),
            hex_digest: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                .to_string(),
        }
    }
}

#[derive(Debug)]
pub enum DigestError {
    DigestParseError,
}

impl Display for DigestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            DigestError::DigestParseError => write!(f, "Error in Parsing Digest From String"),
        }
    }
}

impl Error for DigestError {}

struct DigestVisitor;

impl Digest {
    pub fn new_from_str(s: &str) -> Option<Self> {
        let tokens: Vec<&str> = s.split(':').collect();
        if tokens.len() == 2 {
            return Some(Digest {
                algorithm: String::from(*tokens.get(0).unwrap()),
                hex_digest: String::from(*tokens.get(1).unwrap()),
            });
        };

        None
    }

    pub async fn verify<S>(&self, stream: &mut S) -> bool
    where
        S: Stream + Send + Sync + Unpin,
    {
        while let Some(_data) = stream.next().await {
            // FIXME: Actually implement a digester
            //
        }
        true
    }
}

impl<'de> Visitor<'de> for DigestVisitor {
    type Value = Digest;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "a string representing an OCI Digest.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let tokens: Vec<&str> = value.split(':').collect();

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

impl Display for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.hex_digest)
    }
}

impl FromStr for Digest {
    type Err = DigestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(':').collect();
        if tokens.len() == 2 {
            return Ok(Digest {
                algorithm: String::from(*tokens.get(0).unwrap()),
                hex_digest: String::from(*tokens.get(1).unwrap()),
            });
        }

        Err(DigestError::DigestParseError)
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
