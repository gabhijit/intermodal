use crate::oci::digest::Digest;

use super::errors::DockerReferenceError;
use super::parser::{ANCHORED_CAPTURING_NAME_RE, ANCHORED_REFERENCE_RE};
use super::types::{DockerReference, DockerReferenceResult, DockerRepo};

const DEFAULT_DOCKER_DOMAIN: &str = "docker.io";
///
/// Given an input as a string, return a DockerReference Structure or a DockerReference Error
///
/// Allowed Input formats are as follows
/// - 'image' -> Should resolve to docker.io/library/image:latest
/// - 'image:latest' -> Should resolve to docker.io/library/image:latest
/// - 'docker.io/image -> 'docker.io/library/image:latest'
/// - 'docker.io/image:latest -> 'docker.io/library/image:latest'
///
/// Note: Converting 'docker.io' to actual URL is taken care of by Docker Client.
pub fn parse<'a>(input_ref: &'a str) -> DockerReferenceResult {
    let captured_ref = ANCHORED_REFERENCE_RE.captures(input_ref);

    let mut domain: &str;
    let name: &str;
    match captured_ref {
        Some(c) => {
            if c.len() != 3 {
                return Err(DockerReferenceError::InvalidFormatError);
            }
            domain = c.get(1).map_or("", |m| m.as_str());
            name = c.get(2).map_or("", |m| m.as_str());

            if name.len() == 0 {
                return Err(DockerReferenceError::EmptyNameError);
            }

            if domain.len() == 0 {
                domain = DEFAULT_DOCKER_DOMAIN
            }

            let captured_name = ANCHORED_CAPTURING_NAME_RE.captures(name);

            let tag: &str;
            let digest: &str;
            let path_name: &str;
            match captured_name {
                Some(cn) => {
                    if cn.len() != 4 {}
                    path_name = cn.get(1).map_or("", |m| m.as_str());
                    tag = cn.get(2).map_or("", |m| m.as_str());
                    digest = cn.get(3).map_or("", |m| m.as_str());

                    return Ok(DockerReference {
                        repo: DockerRepo {
                            domain: String::from(domain),
                            path: String::from(path_name),
                        },
                        tag: String::from(tag),
                        digest: Digest::from_str(digest),
                    });
                }
                None => {
                    return Err(DockerReferenceError::NameNotCanonicalError);
                }
            };
        }
        None => {
            return Err(DockerReferenceError::InvalidFormatError);
        }
    };
}
