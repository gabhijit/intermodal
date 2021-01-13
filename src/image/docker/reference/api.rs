//! APIs for Docker Reference Parsing
//!
//! Note: User's outside this module, should only use public API from this module.

use crate::oci::digest::Digest;

use super::errors::ReferenceError;
use super::parser::{ANCHORED_CAPTURING_NAME_RE, ANCHORED_REFERENCE_RE};
use super::types::{DockerReference, DockerReferenceResult, DockerRepo};

pub(crate) const DEFAULT_DOCKER_DOMAIN: &str = "docker.io";
const DEFAULT_DOCKER_IMGNAME_PREFIX: &str = "library";
const DEFAULT_TAG: &str = "latest";
const MAX_REFNAME_LEN: usize = 256;

///
/// Given an input as a string, return a DockerReference Structure or a DockerReference Error
///
/// Allowed Input formats are as follows
/// - 'image' -> Should resolve to docker.io/library/image:latest
/// - 'image:latest' -> Should resolve to docker.io/library/image:latest
/// - 'docker.io/image -> 'docker.io/library/image:latest'
/// - 'docker.io/image:latest -> 'docker.io/library/image:latest'
///
/// Note: Converting 'docker.io' to actual Domain Name is taken care of by Docker Client.
///
pub(crate) fn parse(input_ref: &str) -> DockerReferenceResult {
    if input_ref.is_empty() {
        log::error!("Input reference is Empty!");
        return Err(ReferenceError::EmptyName);
    }

    let (name, mut tag, digest): (String, String, &str);
    let captured_ref = ANCHORED_REFERENCE_RE.captures(input_ref);
    match captured_ref {
        Some(c) => {
            if c.len() != 4 {
                log::error!(
                    "Reference '{}' is not in '[domain]/name[:tag][@digest]' format.",
                    input_ref
                );
                return Err(ReferenceError::InvalidFormat);
            }

            name = String::from(c.get(1).map_or("", |m| m.as_str()));
            if name.is_empty() {
                log::error!("Name part of the reference is empty!");
                return Err(ReferenceError::EmptyName);
            }

            tag = String::from(c.get(2).map_or("", |m| m.as_str()));
            digest = c.get(2).map_or("", |m| m.as_str());

            let name_captures = ANCHORED_CAPTURING_NAME_RE.captures(&name);

            let (mut path_name, mut domain): (String, String);
            match name_captures {
                Some(cn) => {
                    if cn.len() != 3 {
                        log::error!("Parsed name: '{}' not in canonical format!", name);
                        return Err(ReferenceError::NameNotCanonical);
                    }

                    domain = String::from(cn.get(1).map_or("", |m| m.as_str()));
                    if domain.is_empty() {
                        log::debug!(
                            "Empty Domain Found, setting to default '{}'",
                            DEFAULT_DOCKER_DOMAIN
                        );
                        domain = String::from(DEFAULT_DOCKER_DOMAIN);
                    }
                    log::debug!("Domain is '{}'", domain);

                    path_name = String::from(cn.get(2).map_or("", |m| m.as_str()));
                    if path_name.find('/').is_none() && domain == DEFAULT_DOCKER_DOMAIN {
                        log::debug!("Name(Path) found without '/', Setting the default '{}' prefix for the Name.",
                            DEFAULT_DOCKER_IMGNAME_PREFIX
                        );
                        path_name.insert(0, '/');
                        path_name.insert_str(0, DEFAULT_DOCKER_IMGNAME_PREFIX);
                    }

                    if path_name.len() > MAX_REFNAME_LEN {
                        log::debug!(
                            "Length of the name '{}' longer than Maximum '{}',",
                            path_name.len(),
                            MAX_REFNAME_LEN
                        );
                        return Err(ReferenceError::NameTooLong);
                    }
                    log::debug!("Resolved Pathname is : '{}'", path_name);

                    // We always provide default 'latest' tag to image if the input does not
                    // contain a tag
                    if tag.is_empty() {
                        log::debug!(
                            "Tag is Empty. Adding default '{}' tag to the reference.",
                            DEFAULT_TAG
                        );
                        tag = String::from(DEFAULT_TAG);
                    }

                    Ok(DockerReference {
                        repo: DockerRepo {
                            domain,
                            path: path_name,
                        },
                        tag,
                        digest: Digest::new_from_str(digest),
                        input_ref: String::from(input_ref),
                    })
                }
                None => Err(ReferenceError::NameNotCanonical),
            }
        }
        None => Err(ReferenceError::InvalidFormat),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_success_simple() {
        struct ParseTC<'a> {
            input_ref: &'a str,
            output_ref_result: DockerReferenceResult,
        }

        let mut test_cases = vec![
            ParseTC {
                input_ref: "fedora",
                output_ref_result: Ok(DockerReference {
                    repo: DockerRepo {
                        domain: String::from(DEFAULT_DOCKER_DOMAIN),
                        path: String::from("library/fedora"),
                    },
                    tag: String::from("latest"),
                    digest: None,
                    input_ref: String::from("fedora"),
                }),
            },
            ParseTC {
                input_ref: "fedora:f32",
                output_ref_result: Ok(DockerReference {
                    repo: DockerRepo {
                        domain: String::from(DEFAULT_DOCKER_DOMAIN),
                        path: String::from("library/fedora"),
                    },
                    tag: String::from("f32"),
                    digest: None,
                    input_ref: String::from("fedora:f32"),
                }),
            },
            ParseTC {
                input_ref: "",
                output_ref_result: Err(ReferenceError::EmptyName),
            },
        ];

        let mut really_long_refname = "0a".repeat(124);
        really_long_refname.push_str("a");
        let really_long_name_tc = ParseTC {
            input_ref: &really_long_refname,
            output_ref_result: Err(ReferenceError::NameTooLong),
        };
        test_cases.push(really_long_name_tc);

        for tc in test_cases {
            let result = parse(tc.input_ref);
            match result {
                Ok(r) => assert_eq!(r, tc.output_ref_result.ok().unwrap()),
                Err(e) => assert_eq!(e, tc.output_ref_result.err().unwrap()),
            }
        }
    }
}
