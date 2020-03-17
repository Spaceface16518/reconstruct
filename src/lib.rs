#[cfg(feature = "aho-corasick")]
pub fn reconstruct<'a>(string: &'a str, dictionary: &[&str]) -> Vec<&'a str> {
    use aho_corasick::AhoCorasickBuilder;

    let ac = AhoCorasickBuilder::new()
        .auto_configure(dictionary)
        .build(dictionary);

    ac.find_iter(string)
        .map(|m| &string[m.start()..m.end()])
        .collect()
}

#[cfg(not(feature = "aho-corasick"))]
pub fn reconstruct<'a>(string: &'a str, dictionary: &[&str]) -> Vec<&'a str> {
    let mut sentence = Vec::new();

    let mut start = 0;
    while start < string.len() {
        let m = find(&string[start..], dictionary);
        let end = start + m.len();
        sentence.push(&string[start..end]);
        start = end;
    }
    sentence
}

#[cfg(not(feature = "rayon"))]
fn find<'dict>(substring: &str, dictionary: &[&'dict str]) -> &'dict str {
    dictionary
        .iter()
        .find(|&&word| substring.starts_with(word))
        .expect("the string must be composed of words from the dictionary")
}

#[cfg(feature = "rayon")]
fn find<'dict>(substring: &str, dictionary: &[&'dict str]) -> &'dict str {
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
    dictionary
        .par_iter()
        .find_first(|&&word| substring.starts_with(word))
        .expect("the string must be composed of words from the dictionary")
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
