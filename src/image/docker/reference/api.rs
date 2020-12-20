use crate::image::docker::transport::DockerTransport;
use crate::oci::digest::Digest;

use super::errors::DockerReferenceError;
use super::parser::{ANCHORED_CAPTURING_NAME_RE, ANCHORED_REFERENCE_RE};
use super::types::{DockerReference, DockerReferenceResult, DockerRepo};

const DEFAULT_DOCKER_DOMAIN: &str = "docker.io";
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
pub fn parse(input_ref: &str) -> DockerReferenceResult {
    if input_ref.is_empty() {
        return Err(DockerReferenceError::EmptyNameError);
    }

    let (name, mut tag, digest): (String, String, &str);
    let captured_ref = ANCHORED_REFERENCE_RE.captures(input_ref);
    match captured_ref {
        Some(c) => {
            if c.len() != 4 {
                return Err(DockerReferenceError::InvalidFormatError);
            }

            name = String::from(c.get(1).map_or("", |m| m.as_str()));
            if name.is_empty() {
                return Err(DockerReferenceError::EmptyNameError);
            }

            tag = String::from(c.get(2).map_or("", |m| m.as_str()));
            digest = c.get(2).map_or("", |m| m.as_str());

            let name_captures = ANCHORED_CAPTURING_NAME_RE.captures(&name);

            let (mut path_name, mut domain): (String, String);
            match name_captures {
                Some(cn) => {
                    if cn.len() != 3 {
                        return Err(DockerReferenceError::NameNotCanonicalError);
                    }

                    domain = String::from(cn.get(1).map_or("", |m| m.as_str()));
                    if domain.is_empty() {
                        domain = String::from(DEFAULT_DOCKER_DOMAIN);
                    }

                    path_name = String::from(cn.get(2).map_or("", |m| m.as_str()));
                    if path_name.find('/').is_none() {
                        path_name.insert(0, '/');
                        path_name.insert_str(0, DEFAULT_DOCKER_IMGNAME_PREFIX);
                    }

                    if path_name.len() > MAX_REFNAME_LEN {
                        return Err(DockerReferenceError::NameTooLongError);
                    }

                    if tag.is_empty() {
                        tag = String::from(DEFAULT_TAG);
                    }

                    Ok(DockerReference {
                        repo: DockerRepo {
                            domain,
                            path: path_name,
                        },
                        tag,
                        digest: Digest::from_str(digest),
                        input_ref: String::from(input_ref),
                        transport: DockerTransport::singleton(),
                    })
                }
                None => Err(DockerReferenceError::NameNotCanonicalError),
            }
        }
        None => Err(DockerReferenceError::InvalidFormatError),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_success_simple() {
        struct ParseTC<'a> {
            input_ref: &'a str,
            output_ref_result: DockerReferenceResult<'a>,
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
                    transport: DockerTransport::singleton(),
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
                    transport: DockerTransport::singleton(),
                }),
            },
            ParseTC {
                input_ref: "",
                output_ref_result: Err(DockerReferenceError::EmptyNameError),
            },
        ];

        let mut really_long_refname = "0a".repeat(124);
        really_long_refname.push_str("a");
        let really_long_name_tc = ParseTC {
            input_ref: &really_long_refname,
            output_ref_result: Err(DockerReferenceError::NameTooLongError),
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
