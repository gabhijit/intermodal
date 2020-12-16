/// Docker Image reference handling
///
///
/// ```ignore
/// Grammar
///
/// 	reference                       := name [ ":" tag ] [ "@" digest ]
///	name                            := [domain '/'] path-component ['/' path-component]*
///	domain                          := domain-component ['.' domain-component]* [':' port-number]
///	domain-component                := /([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])/
///	port-number                     := /[0-9]+/
///	path-component                  := alpha-numeric [separator alpha-numeric]*
/// 	alpha-numeric                   := /[a-z0-9]+/
///	separator                       := /[_.]|__|[-]*/
///
///	tag                             := /[\w][\w.-]{0,127}/
///
///	digest                          := digest-algorithm ":" digest-hex
///	digest-algorithm                := digest-algorithm-component [ digest-algorithm-separator digest-algorithm-component ]*
///	digest-algorithm-separator      := /[+.-_]/
///	digest-algorithm-component      := /[A-Za-z][A-Za-z0-9]*/
///	digest-hex                      := /[0-9a-fA-F]{32,}/ ; At least 128 bit digest value
///
///	identifier                      := /[a-f0-9]{64}/
///	short-identifier                := /[a-f0-9]{6,64}/
/// ```
///
use lazy_static::lazy_static;
use regex::{escape, Regex};
use std::string::String;

// Regular expressions

macro_rules! expression_re {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_str = String::new();
            $(
                temp_str.push_str(&$x.to_string());
            )*
            Regex::new(&temp_str).unwrap()
        }
    }
}

macro_rules! group_re {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_str = String::from("(?:");
            $(
                temp_str.push_str(&expression_re!($x).to_string());
            )*
            temp_str.push_str(")");
            Regex::new(&temp_str).unwrap()
        }
    }
}

macro_rules! capture_re {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_str = String::from("(");
            $(
                temp_str.push_str(&expression_re!($x).to_string());
            )*
            temp_str.push_str(")");
            Regex::new(&temp_str).unwrap()
        }
    }
}

macro_rules! repeated_re {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_str = String::new();
            let mut expr_str = String::new();
            $(
                expr_str.push_str(&expression_re!($x).to_string());
            )*
            temp_str.push_str(&group_re!(Regex::new(&expr_str).unwrap()).to_string());
            temp_str.push_str("+");
            Regex::new(&temp_str).unwrap()
        }
    }
}

macro_rules! optional_re {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_str = String::new();
            let mut expr_str = String::new();
            $(
                expr_str.push_str(&expression_re!($x).to_string());
            )*
            temp_str.push_str(&group_re!(Regex::new(&expr_str).unwrap()).to_string());
            temp_str.push_str("?");
            Regex::new(&temp_str).unwrap()
        }
    }
}

macro_rules! anchor_re {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_str = String::from("^");
            $(
                temp_str.push_str(&expression_re!($x).to_string());
            )*
            temp_str.push_str("$");
            Regex::new(&temp_str).unwrap()
        }
    }
}

lazy_static! {
    static ref DOMAIN_COMPONENT_RE: Regex =
        Regex::new(r"(?:[[:alnum:]]|[[:alnum:]][[:alnum:]]*[[:alnum:]])").unwrap();
    static ref PORT_NO_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref LOWER_ALNUM_RE: Regex = Regex::new(r"[a-z0-9]+").unwrap();
    static ref SEPERATOR_RE: Regex = Regex::new(r"[_.]|__|[-]").unwrap();
    static ref TAG_RE: Regex = Regex::new(r"[\w][\w.-]{0,127}").unwrap();

    // Digest specific
    static ref DIGEST_ALGO_SEP_RE: Regex = Regex::new(r"[+.-_]").unwrap();
    static ref DIGEST_ALGO_COMP_RE: Regex = Regex::new(r"[[:alnum:]][[:alnum:]]*").unwrap();
    static ref DIGEST_HEX_RE: Regex = Regex::new(r"[[:xdigit:]]{32,}").unwrap();

    static ref ID_RE: Regex = Regex::new(r"[a-f0-9]{64}").unwrap();
    static ref SHORT_ID_RE: Regex = Regex::new(r"[a-f0-9]{6, 64}").unwrap();

    // Composed regular expressions for repository types.
    static ref DOMAIN_RE: Regex =
        expression_re!(
            DOMAIN_COMPONENT_RE,
            optional_re!(
                repeated_re!(
                    literal_re("."), DOMAIN_COMPONENT_RE
                )
            ),
            optional_re!(
                literal_re(":"),
                PORT_NO_RE
            )
        );

    static ref PATH_COMPONENT_RE: Regex =
        expression_re!(
            LOWER_ALNUM_RE,
            optional_re!(
                SEPERATOR_RE,
                LOWER_ALNUM_RE
            )
        );

    static ref NAME_RE: Regex =
        expression_re!(
            optional_re!(
                DOMAIN_RE,
                literal_re("/")
            ),
            PATH_COMPONENT_RE,
            optional_re!(
                repeated_re!(
                    literal_re("/"),
                    PATH_COMPONENT_RE
                )
            )
        );

    static ref DIGEST_ALGO_RE: Regex =
        expression_re!(
            DIGEST_ALGO_COMP_RE,
            optional_re!(repeated_re!( DIGEST_ALGO_SEP_RE, DIGEST_ALGO_COMP_RE))
        );

    static ref DIGEST_RE: Regex = expression_re!(DIGEST_ALGO_RE, literal_re(":"), DIGEST_HEX_RE);

    static ref REFERENCE_RE: Regex =
        expression_re!(
            NAME_RE,
            optional_re!(literal_re(":"), TAG_RE),
            optional_re!(literal_re("@"), DIGEST_RE)
        );
}

fn literal_re(l: &str) -> Regex {
    Regex::new(&String::from(escape(l))).unwrap()
}

#[cfg(test)]
mod tests {

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
}
