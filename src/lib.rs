#[cfg(feature = "aho-corasick")]
pub fn reconstruct<'a>(string: &'a str, dictionary: &[&str]) -> Option<Vec<&'a str>> {
    use aho_corasick::AhoCorasickBuilder;

    let ac = AhoCorasickBuilder::new()
        .auto_configure(dictionary)
        .build(dictionary);
    let mut sentence_length = 0;
    let sentence = ac.find_iter(string).map(|m| &string[m.start()..m.end()]).inspect(|s| sentence_length += s.len()).collect();
    if sentence_length == string.len() {
        Some(sentence)
    } else {
        None
    }
}

#[cfg(not(feature = "aho-corasick"))]
pub fn reconstruct<'a>(string: &'a str, dictionary: &[&str]) -> Option<Vec<&'a str>> {
    let mut sentence = Vec::new();

    let mut start = 0;
    while start < string.len() {
        let m = find(&string[start..], dictionary)?;
        let end = start + m.len();
        sentence.push(&string[start..end]);
        start = end;
    }
    Some(sentence)
}

#[cfg(all(not(feature = "aho-corasick"), not(feature = "rayon")))]
fn find<'dict>(substring: &str, dictionary: &[&'dict str]) -> Option<&'dict str> {
    dictionary
        .iter()
        .copied()
        .find(|&word| substring.starts_with(word))
}

#[cfg(all(feature = "rayon", not(feature = "aho-corasick")))]
fn find<'dict>(substring: &str, dictionary: &[&'dict str]) -> Option<&'dict str> {
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
    dictionary
        .par_iter()
        .copied()
        .find_first(|&word| substring.starts_with(word))
}

#[cfg(test)]
mod tests {
    use crate::reconstruct;

    #[test]
    fn problem_example1() {
        let dictionary = &["quick", "brown", "the", "fox"];
        let string = "thequickbrownfox";

        let expected = &["the", "quick", "brown", "fox"];
        let actual = reconstruct(string, dictionary).expect("valid input should return Some");
        assert_eq!(actual, expected)
    }

    #[test]
    fn problem_example2() {
        let dictionary = &["bed", "bath", "bedbath", "and", "beyond"];
        let string = "bedbathandbeyond";

        let expected1 = &["bed", "bath", "and", "beyond"];
        let expected2 = &["bedbath", "and", "beyond"];
        let actual = reconstruct(string, dictionary).expect("valid input should return Some");
        assert!(actual == expected1 || actual == expected2);
    }

    #[test]
    fn non_example() {
        let dictionary = &["quick", "brown", "the", "fox"];
        let string = "thequickandbrownfox";

        let actual = reconstruct(string, dictionary);
        assert!(actual.is_none())
    }
}
