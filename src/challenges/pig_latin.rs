use std::collections::HashSet;

lazy_static! {
    static ref CHAR_CONSONANTS: HashSet<char> = HashSet::from([
        'B', 'C', 'D', 'F', 'G', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'S', 'T', 'V', 'X', 'Z', 'H',
        'R', 'W', 'Y', 'b', 'c', 'd', 'f', 'g', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 's', 't', 'v',
        'x', 'z', 'h', 'r', 'w', 'y',
    ]);
    static ref CHAR_VOWELS: HashSet<char> =
        HashSet::from(['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u']);
}

fn is_consonant(c: &char) -> bool {
    CHAR_CONSONANTS.contains(c)
}
fn is_vowel(c: &char) -> bool {
    CHAR_VOWELS.contains(c)
}

pub fn to_pig_latin(word: &str) -> String {
    let mut new_word = String::new();
    let mut is_first_char = true;
    let mut has_first_consonant = false;
    let mut first_consonant = String::new();
    let mut has_first_vowel = false;
    let mut chars = word.chars().peekable();

    loop {
        let next_element = chars.next();
        let is_last_char = chars.peek().is_none();
        match next_element {
            Some(c) => {
                if is_first_char {
                    is_first_char = false;
                    if is_consonant(&c) {
                        has_first_consonant = true;
                        first_consonant.push(c);
                        continue;
                    }
                    if is_vowel(&c) {
                        has_first_vowel = true;
                        new_word.push(c);
                        continue;
                    }
                    new_word.push(c);
                    continue;
                }
                new_word.push(c);
                if is_last_char && has_first_vowel {
                    new_word.push_str("-hay");
                    continue;
                }
                if is_last_char && has_first_consonant {
                    new_word.push_str(&format!("-{first_consonant}ay"));
                }
            }
            None => break new_word,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pig_latin_starting_with_consonant() {
        let word = "software";

        let pig_latin = to_pig_latin(word);

        assert_eq!(pig_latin, "oftware-say");
    }

    #[test]
    fn test_pig_latin_starting_with_vowel() {
        let word = "architecture";

        let pig_latin = to_pig_latin(word);

        assert_eq!(pig_latin, "architecture-hay");
    }

    #[test]
    fn test_non_ascii_character_starting_word_unchanged() {
        let word = "单词";

        let pig_latin = to_pig_latin(word);

        assert_eq!(pig_latin, word);
    }
}
