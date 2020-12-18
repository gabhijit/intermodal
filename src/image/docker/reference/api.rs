use crate::oci::digest::Digest;

use super::errors::DockerReferenceError;
use super::parser::{ANCHORED_CAPTURING_NAME_RE, ANCHORED_REFERENCE_RE};
use super::types::{DockerReference, DockerReferenceResult, DockerRepo};

const DEFAULT_DOCKER_DOMAIN: &str = "docker.io";
const DEFAULT_DOCKER_IMGNAME_PREFIX: &str = "library";
const DEFAULT_TAG: &str = "latest";
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
pub fn parse<'a>(input_ref: &'a str) -> DockerReferenceResult {
    let captured_ref = ANCHORED_REFERENCE_RE.captures(input_ref);

    let (mut name, mut tag, digest): (String, String, &str);
    match captured_ref {
        Some(c) => {
            if c.len() != 4 {
                return Err(DockerReferenceError::InvalidFormatError);
            }

            name = String::from(c.get(1).map_or("", |m| m.as_str()));
            if name.len() == 0 {
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
                    if domain.len() == 0 {
                        domain = String::from(DEFAULT_DOCKER_DOMAIN);
                    }

                    path_name = String::from(cn.get(2).map_or("", |m| m.as_str()));
                    if let None = path_name.find("/") {
                        path_name.insert(0, '/');
                        path_name.insert_str(0, DEFAULT_DOCKER_IMGNAME_PREFIX);
                    }

                    if tag.len() == 0 {
                        tag = String::from(DEFAULT_TAG);
                    }

                    return Ok(DockerReference {
                        repo: DockerRepo {
                            domain: domain,
                            path: path_name,
                        },
                        tag: tag,
                        digest: Digest::from_str(digest),
                        input_ref: String::from(input_ref),
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_success_simple() {
        struct ParseTC<'a> {
            input_ref: &'a str,
            output_ref_result: DockerReferenceResult,
        }

        let test_cases = vec![
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
        ];

        for tc in test_cases {
            let result = parse(tc.input_ref);
            match result {
                Ok(r) => assert_eq!(r, tc.output_ref_result.ok().unwrap()),
                Err(e) => assert_eq!(e, tc.output_ref_result.err().unwrap()),
            }
        }
    }
}
