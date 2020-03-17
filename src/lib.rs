#[cfg(feature = "aho-corasick")]
pub fn reconstruct(string: &str, dictionary: &[&str]) -> Vec<String> {
    use aho_corasick::{AhoCorasickBuilder, MatchKind};

    let ac = AhoCorasickBuilder::new()
        .prefilter(false)
        .auto_configure(dictionary)
        .build(dictionary);

    ac.find_iter(string)
        .map(|m| &string[m.start()..m.end()])
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::reconstruct;

    #[test]
    fn problem_example1() {
        let dictionary = &["quick", "brown", "the", "fox"];
        let string = "thequickbrownfox";

        let expected = &["the", "quick", "brown", "fox"];
        let actual = reconstruct(string, dictionary);
        assert_eq!(&actual, expected)
    }

    #[test]
    fn problem_example2() {
        let dictionary = &["bed", "bath", "bedbath", "and", "beyond"];
        let string = "bedbathandbeyond";

        let expected1 = &["bed", "bath", "and", "beyond"];
        let expected2 = &["bedbath", "and", "beyond"];
        let actual = reconstruct(string, dictionary);
        assert!(&actual == expected1 || &actual == expected2);
    }
}
