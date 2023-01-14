const CHAR_CONSONANTS: [char; 42] = [
    'B', 'C', 'D', 'F', 'G', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'S', 'T', 'V', 'X', 'Z', 'H', 'R',
    'W', 'Y', 'b', 'c', 'd', 'f', 'g', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 's', 't', 'v', 'x', 'z',
    'h', 'r', 'w', 'y',
];

const CHAR_VOWELS: [char; 10] = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

fn is_consonant(c: &char) -> bool {
    CHAR_CONSONANTS.contains(c)
}
fn is_vowel(c: &char) -> bool {
    CHAR_VOWELS.contains(c)
}

pub fn to_pig_latin(word: &String) -> String {
    let mut new_word = String::new();
    let mut is_first_char = true;
    let mut has_first_consonant = false;
    let mut first_consonant = String::new();
    let mut has_first_vowel = false;
    let mut chars = word.chars().peekable();

    loop {
        let next_element = chars.next();
        let is_last_char = match chars.peek() {
            Some(_) => false,
            None => true,
        };
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
                    new_word.push_str(&format!("-{}ay", first_consonant));
                }
            }
            None => break new_word,
        }
    }
}
