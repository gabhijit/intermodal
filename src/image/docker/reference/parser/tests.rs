use super::*;
use regex::Regex;

#[test]
fn test_expression_re() {
    let composite = expression_re!(Regex::new("a").unwrap(), Regex::new("b").unwrap());
    assert_eq!(composite.to_string(), "ab");
}

#[test]
fn test_group_re() {
    let group = group_re!(Regex::new("a").unwrap(), Regex::new("b").unwrap());
    assert_eq!(group.to_string(), "(?:ab)");
}

#[test]
fn test_repeated_re() {
    let optional = repeated_re!(Regex::new("a").unwrap(), Regex::new("b").unwrap());
    assert_eq!(optional.to_string(), "(?:ab)+");
}

#[test]
fn test_optional_re() {
    let optional = optional_re!(Regex::new("a").unwrap(), Regex::new("b").unwrap());
    assert_eq!(optional.to_string(), "(?:ab)?");
}

#[test]
fn test_anchor_re() {
    let anchor = anchor_re!(Regex::new("a").unwrap(), Regex::new("b").unwrap());
    assert_eq!(anchor.to_string(), "^ab$");
}

#[test]
fn test_capture_re() {
    let capture = capture_re!(
        Regex::new("a").unwrap(),
        Regex::new("b").unwrap(),
        Regex::new("c").unwrap()
    );
    assert_eq!(capture.to_string(), "(abc)");
}

#[test]
fn test_literal_re() {
    let s = literal_re("$?");
    assert_eq!(s.to_string(), r"\$\?");
}

#[test]
fn test_domain_component() {
    let s = "-ab.com";
    let anchored = anchor_re!(DOMAIN_COMPONENT_RE);
    assert_eq!(
        anchored.is_match(s),
        false,
        "assertion failed {} {}",
        DOMAIN_COMPONENT_RE.to_string(),
        DOMAIN_COMPONENT_RE.find(s).expect("panicked").as_str()
    );
}

#[test]
fn test_domain_regexps() {
    struct DomainTC<'a> {
        case: &'a str,
        result: bool,
    }
    let test_cases = vec![
        DomainTC {
            case: "test.com",
            result: true,
        },
        DomainTC {
            case: "test.com:10304",
            result: true,
        },
        DomainTC {
            case: "localhost",
            result: true,
        },
        DomainTC {
            case: "localhostL:8080",
            result: true,
        },
        DomainTC {
            case: "a",
            result: true,
        },
        DomainTC {
            case: "a.b",
            result: true,
        },
        DomainTC {
            case: "ab.cd.com",
            result: true,
        },
        DomainTC {
            case: "a-b.com",
            result: true,
        },
        DomainTC {
            case: "-ab.com",
            result: false,
        },
        DomainTC {
            case: "ab-.com",
            result: false,
        },
        DomainTC {
            case: "ab.c-om",
            result: true,
        },
        DomainTC {
            case: "ab.-com",
            result: false,
        },
        DomainTC {
            case: "ab.com-",
            result: false,
        },
        DomainTC {
            case: "0101.com",
            result: true,
        },
        DomainTC {
            case: "0101a.com",
            result: true,
        },
        DomainTC {
            case: "g.abc.io",
            result: true,
        },
        DomainTC {
            case: "g.abc.io:443",
            result: true,
        },
        DomainTC {
            case: "xn--abc.com",
            result: true,
        },
        DomainTC {
            case: "Asdf.io",
            result: true,
        },
    ];

    let anchored = anchor_re!(DOMAIN_RE);
    for tc in test_cases {
        assert_eq!(
            anchored.is_match(tc.case),
            tc.result,
            "TC failed for domain: {}",
            tc.case
        );
    }
}

#[test]
fn test_name_regexps() {
    struct NameTC<'a> {
        name: &'a str,
        result: bool,
        groups: Vec<&'a str>,
    }

    let mut test_cases = vec![
        NameTC {
            name: "",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "short",
            result: true,
            groups: vec!["", "short"],
        },
        NameTC {
            name: "simple/short",
            result: true,
            groups: vec!["simple", "short"],
        },
        NameTC {
            name: "library/ubuntu",
            result: true,
            groups: vec!["library", "ubuntu"],
        },
        NameTC {
            name: "docker/hyphenos/app",
            result: true,
            groups: vec!["docker", "hyphenos/app"],
        },
        NameTC {
            name: "aa/aa/aa/aa/aa/aa/bb/bb",
            result: true,
            groups: vec!["aa", "aa/aa/aa/aa/aa/bb/bb"],
        },
        NameTC {
            name: "a/a/a/a/a",
            result: true,
            groups: vec!["a", "a/a/a/a"],
        },
        NameTC {
            name: "a/a/a/a/a/",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "a//a/a",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "foo.com",
            result: true,
            groups: vec!["", "foo.com"],
        },
        NameTC {
            name: "foo.com/",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "foo.com:8080/bar",
            result: true,
            groups: vec!["foo.com:8080", "bar"],
        },
        NameTC {
            name: "foo.com:http/bar",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "foo.com/",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "foo.com:8080/bar",
            result: true,
            groups: vec!["foo.com:8080", "bar"],
        },
        NameTC {
            name: "foo.com:8080/bar/baz",
            result: true,
            groups: vec!["foo.com:8080", "bar/baz"],
        },
        NameTC {
            name: "a^a",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "aa/aa$$^a/a",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "aaaa$$^a/aa",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "___/___",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "_docker/docker_",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "docker/docker_.",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "docker/docker_.new",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "d..ocker/docker",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "d..ocker",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "docker.io/docker.new",
            result: true,
            groups: vec!["docker.io", "docker.new"],
        },
        NameTC {
            name: "docker.io/some_seperator_with__double.dot-hyphen",
            result: true,
            groups: vec!["docker.io", "some_seperator_with__double.dot-hyphen"],
        },
        NameTC {
            name: "registry.io/foo/project--id.module--name.ver---sion--name",
            result: true,
            groups: vec![
                "registry.io",
                "foo/project--id.module--name.ver---sion--name",
            ],
        },
        NameTC {
            name: "d__ocker/docker",
            result: true,
            groups: vec!["", "d__ocker/docker"],
        },
        NameTC {
            name: "d__ocker:8080/docker",
            result: false,
            groups: vec![],
        },
        NameTC {
            name: "Docker.com:8080/docker",
            result: true,
            groups: vec!["Docker.com:8080", "docker"],
        },
        NameTC {
            name: "Docker/Docker.image",
            result: false,
            groups: vec![],
        },
    ];

    let mut long_slashes_path_string = "a/".repeat(127);
    long_slashes_path_string.push_str("a");

    let mut long_slashes_input_string = "a/".repeat(128);
    long_slashes_input_string.push_str("a");

    let long_slashes_tc = NameTC {
        name: &long_slashes_input_string,
        result: true,
        groups: vec!["a", &long_slashes_path_string],
    };
    test_cases.push(long_slashes_tc);

    let anchored = anchor_re!(CAPTURING_NAME_RE);
    for tc in test_cases {
        let captures = anchored.captures(tc.name);
        match captures {
            Some(c) => {
                assert_eq!(c.len(), 3, "regex: {}", anchored.as_str());
                assert_eq!(
                    c.get(1).map_or("", |m| m.as_str()),
                    tc.groups[0],
                    "expected: {}, found: {}",
                    tc.groups[0],
                    c.get(1).map_or("", |m| m.as_str()),
                );
                assert_eq!(
                    c.get(2).map_or("", |m| m.as_str()),
                    tc.groups[1],
                    "expected: {}, found: {}",
                    tc.groups[1],
                    c.get(2).map_or("", |m| m.as_str()),
                );
            }
            None => assert_eq!(
                tc.result,
                false,
                "failed for string: {}, regex: {}",
                tc.name,
                anchored.as_str()
            ),
        }
    }
}

#[test]
fn test_reference_regexpes() {
    struct RefTC<'a> {
        reference: &'a str,
        result: bool,
        groups: Vec<&'a str>,
    }

    let test_cases = vec![
        RefTC {
            reference: "fedora",
            result: true,
            groups: vec!["fedora", "", ""],
        },
        RefTC {
            reference: "registry.com:8080/myapp:tag",
            result: true,
            groups: vec!["registry.com:8080/myapp", "tag", ""],
        },
        RefTC {
            reference: "registry.com:8080/myapp@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912",
            result: true,
            groups: vec!["registry.com:8080/myapp", "", "sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912"],
        },
        RefTC {
            reference: "registry.com:8080/myapp:mytag2@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912",
            result: true,
            groups: vec!["registry.com:8080/myapp", "mytag2", "sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912"],
        },
        RefTC {
            reference: "registry.com:8080/myapp@sha256:badbadbadbad",
            result: false,
            groups: vec![],
        },
        RefTC {
            reference: "registry.com:8080/myapp:invalid~tag",
            result: false,
            groups: vec![],
        },
        RefTC {
            reference: "bad_hostname.com:8080/myapp:tag",
            result: false,
            groups: vec![],
        },
        RefTC {
            reference: "localhost:8080@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912",
            result: true,
            groups: vec!["localhost", "8080", "sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912"],
        },
        RefTC {
            reference: "localhost:8080/name@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912",
            result: true,
            groups: vec!["localhost:8080/name", "", "sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912"],
        },
        RefTC {
            reference: "localhost:http/name@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912",
            result: false,
            groups: vec![]
        },
        RefTC {
            reference: "localhost@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912",
            result: true,
            groups: vec!["localhost", "", "sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912"],
        },
        RefTC {
            reference: "registry.com:8080/myapp@bad",
            result: false,
            groups: vec![],
        },
        RefTC {
            reference: "registry.com:8080/myapp@2bad",
            result: false,
            groups: vec![],
        },
        RefTC {
            reference: "",
            result: false,
            groups: vec![],
        },
    ];

    let anchored = anchor_re!(REFERENCE_RE);
    for tc in test_cases {
        let captures = anchored.captures(tc.reference);
        match captures {
            Some(c) => {
                assert_eq!(c.len(), 4, "regex: {}", anchored.as_str());
                assert_eq!(
                    c.get(1).map_or("", |m| m.as_str()),
                    tc.groups[0],
                    "expected: {}, found: {}",
                    tc.groups[0],
                    c.get(1).map_or("", |m| m.as_str()),
                );
                assert_eq!(
                    c.get(2).map_or("", |m| m.as_str()),
                    tc.groups[1],
                    "expected: {}, found: {}",
                    tc.groups[1],
                    c.get(2).map_or("", |m| m.as_str()),
                );
                assert_eq!(
                    c.get(3).map_or("", |m| m.as_str()),
                    tc.groups[2],
                    "expected: {}, found: {}",
                    tc.groups[2],
                    c.get(3).map_or("", |m| m.as_str()),
                );
            }
            None => assert_eq!(
                tc.result,
                false,
                "failed for string: {}, regex: {}",
                tc.reference,
                anchored.as_str()
            ),
        }
    }
}

#[test]
fn test_identifier_regexps() {
    struct StructID<'a> {
        id: &'a str,
        result: bool,
    }

    let test_cases = vec![
        StructID {
            id: "da304e823d8ca2b9d863a3c897baeb852ba21ea9a9f1414736394ae7fcaf9821",
            result: true,
        },
        StructID {
            id: "7EC43B381E5AEFE6E04EFB0B3F0693FF2A4A50652D64AEC573905F2DB5889A1C",
            result: false,
        },
        StructID {
            id: "da304e823d8ca2b9d863a3c897baeb852ba21ea9a9f1414736394ae7fcaf",
            result: false,
        },
        StructID {
            id: "sha256:da304e823d8ca2b9d863a3c897baeb852ba21ea9a9f1414736394ae7fcaf9821",
            result: false,
        },
        StructID {
            id: "da304e823d8ca2b9d863a3c897baeb852ba21ea9a9f1414736394ae7fcaf98218482",
            result: false,
        },
    ];

    for tc in test_cases {
        let anchored = anchor_re!(ID_RE);
        assert_eq!(anchored.is_match(tc.id), tc.result);
    }

    let short_test_cases = vec![
        StructID {
            id: "abcde",
            result: false,
        },
        StructID {
            id: "abc2de",
            result: true,
        },
        StructID {
            id: "da304e823d8ca2b9d863a3c897baeb852ba21ea9a9f1414736394ae7fcaf",
            result: true,
        },
    ];

    for tc in short_test_cases {
        let anchored = anchor_re!(SHORT_ID_RE);
        assert_eq!(anchored.is_match(tc.id), tc.result);
    }
}
