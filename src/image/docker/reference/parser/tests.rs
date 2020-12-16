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
        captures_len: Option<usize>,
        groups: Vec<&'a str>,
    }

    let test_cases = vec![
        NameTC {
            name: "",
            result: false,
            captures_len: None,
            groups: vec![],
        },
        NameTC {
            name: "short",
            result: true,
            captures_len: Some(3),
            groups: vec!["", "short"],
        },
        NameTC {
            name: "simple/short",
            result: true,
            captures_len: Some(3),
            groups: vec!["simple", "short"],
        },
    ];

    let anchored = anchor_re!(NAME_RE);
    for tc in test_cases {
        let result = anchored.captures(tc.name);
        match result {
            Some(n) => {
                assert_eq!(
                    n.len(),
                    tc.captures_len.unwrap(),
                    "regex: {}",
                    anchored.as_str()
                );
                assert_eq!(
                    n.get(1).map_or("", |m| m.as_str()),
                    tc.groups[0],
                    "expected: {}, found: {}",
                    tc.groups[0],
                    n.get(1).map_or("", |m| m.as_str()),
                );
                assert_eq!(
                    n.get(2).map_or("", |m| m.as_str()),
                    tc.groups[1],
                    "expected: {}, found: {}",
                    tc.groups[1],
                    n.get(2).map_or("z", |m| m.as_str()),
                );
            }
            None => assert_eq!(None, tc.captures_len),
        }
    }
}
