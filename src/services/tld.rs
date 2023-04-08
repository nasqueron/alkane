//  -------------------------------------------------------------
//  Alkane :: Services :: TLD
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  Description:    Parse public suffix list to identify TLD
//  -------------------------------------------------------------

use lazy_static::lazy_static;

//  -------------------------------------------------------------
//  Public suffix list
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

lazy_static! {
    static ref PUBLIC_SUFFIX_LIST: &'static str = include_str!("../data/public_suffix_list.dat");
    static ref PUBLIC_SUFFIXES: Vec<&'static str> = {
        PUBLIC_SUFFIX_LIST
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .collect()
    };
}

//  -------------------------------------------------------------
//  Helper methods
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn get_tld<S>(fqdn: S) -> String
where
    S: AsRef<str>,
{
    let fqdn = fqdn.as_ref();

    let parts: Vec<&str> = fqdn.split(".").collect();
    let n = parts.len();

    // Seek a.b.c.d, b.c.d, c.d, d until we find a known suffix
    for i in 0..n {
        let candidate = &parts[i..].join(".");

        if PUBLIC_SUFFIXES.contains(&candidate.as_str()) {
            return String::from(candidate);
        }
    }

    // If the TLD isn't declared in the public suffix list,
    // heuristic suggests the last part is a private TLD.
    match parts.last() {
        None => String::new(),
        Some(part) => String::from(*part),
    }
}

pub fn extract_domain_parts<S>(fqdn: S) -> Option<(String, String, String)>
where
    S: AsRef<str>,
{
    let fqdn = fqdn.as_ref();

    let tld = get_tld(fqdn);

    let tld_parts_len = count_chars(&tld, '.') + 1;
    let parts: Vec<_> = fqdn.split(".").collect();

    let n = parts.len();
    if n >= tld_parts_len + 2 {
        // We've got a winner
        let bound = n - tld_parts_len - 1;
        Some((parts[0..bound].join("."), parts[bound].to_string(), tld))
    } else {
        None
    }
}

fn count_chars<S>(expression: S, pattern: char) -> usize
where
    S: AsRef<str>,
{
    let expression = expression.as_ref();

    expression.chars().filter(|&c| c == pattern).count()
}

//  -------------------------------------------------------------
//  Unit tests
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tld() {
        assert_eq!("eu.org", get_tld("foo.acme.eu.org"))
    }

    fn provide_extracted_domains(
    ) -> impl Iterator<Item = (&'static str, Option<(String, String, String)>)> {
        vec![
            // Regular TLD cases
            (
                "foo.acme.org",
                Some((
                    String::from("foo"),
                    String::from("acme"),
                    String::from("org"),
                )),
            ),
            ("acme.org", None),
            ("org", None),
            ("", None),
            // Composite TLD from the public suffix list
            (
                "foo.acme.co.uk",
                Some((
                    String::from("foo"),
                    String::from("acme"),
                    String::from("co.uk"),
                )),
            ),
            (
                "foo.acme.eu.org",
                Some((
                    String::from("foo"),
                    String::from("acme"),
                    String::from("eu.org"),
                )),
            ),
            // Longer subdomain
            (
                "bar.foo.acme.eu.org",
                Some((
                    String::from("bar.foo"),
                    String::from("acme"),
                    String::from("eu.org"),
                )),
            ),
        ]
        .into_iter()
    }

    #[test]
    fn test_extract_domain_parts() {
        for (fqdn, expected) in provide_extracted_domains() {
            assert_eq!(expected, extract_domain_parts(fqdn), "Test with {}", fqdn);
        }
    }
}
