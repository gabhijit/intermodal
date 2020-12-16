use super::parser::*;
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
