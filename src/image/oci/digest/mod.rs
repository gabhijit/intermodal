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

use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use sha2::{digest::DynDigest, Digest as ShaDigest, Sha256};
use tokio::io::{AsyncRead, AsyncReadExt};

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

impl Digest {
    /// Return a Sha256 Digest For the given Vector of bytes.
    ///
    /// Sha256 is our default algorith,
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        <Sha256 as ShaDigest>::reset(&mut hasher);
        <Sha256 as ShaDigest>::update(&mut hasher, bytes);

        Digest {
            algorithm: "sha256".to_string(),
            hex_digest: hex::encode(hasher.finalize()),
        }
    }

    fn digester(&self) -> Result<Box<dyn DynDigest + Send>, DigestError> {
        match &*self.algorithm.to_lowercase() {
            "sha256" => Ok(Box::<sha2::Sha256>::default()),
            _ => Err(DigestError::AlgorithmNotSupported(
                self.algorithm.to_string(),
            )),
        }
    }
}

#[derive(Debug)]
pub enum DigestError {
    CannotParse(String),
    AlgorithmNotSupported(String),
    InvalidDigest,
}

impl Display for DigestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            DigestError::CannotParse(ref s) => {
                write!(f, "Error in Parsing Digest From Input: '{}'", s)
            }
            DigestError::AlgorithmNotSupported(ref s) => {
                write!(f, "Digest Algorithm: {} Not supported.", s)
            }
            DigestError::InvalidDigest => write!(f, "Computed Digest does not match."),
        }
    }
}

impl Error for DigestError {}

struct DigestVisitor;

impl Digest {
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    pub fn hex_digest(&self) -> &str {
        &self.hex_digest
    }

    pub fn new_from_str(s: &str) -> Option<Self> {
        let tokens: Vec<&str> = s.split(':').collect();
        if tokens.len() == 2 {
            return Some(Digest {
                algorithm: String::from(*tokens.first().unwrap()),
                hex_digest: String::from(*tokens.get(1).unwrap()),
            });
        };

        None
    }

    pub async fn verify<R>(&self, reader: &mut R) -> bool
    where
        R: AsyncRead + Send + Sync + Unpin,
    {
        let mut buf: Vec<u8> = vec![0; 16384];
        let mut digester = self.digester().unwrap();

        digester.reset();
        loop {
            let n = reader.read(&mut buf[..]).await.unwrap();
            if n == 0 {
                break;
            }
            digester.update(&buf[..n]);
        }

        let result = digester.finalize();

        log::trace!(
            "Hex Digest: {}, Result: {}",
            self.hex_digest,
            hex::encode(&result)
        );
        hex::encode(result) == self.hex_digest
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
                algorithm: String::from(*tokens.first().unwrap()),
                hex_digest: String::from(*tokens.get(1).unwrap()),
            });
        }

        Err(DigestError::CannotParse(format!(
            "Cannot Parse '{}' as a Digest",
            s
        )))
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

    #[tokio::test]
    async fn test_verify_success() {
        let s = String::from("");
        let d = Digest::default();

        assert!(d.verify(&mut s.as_bytes()).await);
    }
}
