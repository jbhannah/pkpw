use rand::rng;
use wasm_bindgen::prelude::*;

use crate::generate;

const MAX_LEN: usize = 1024;
const MAX_COUNT: usize = 1024;
const MAX_APPEND_NUMBERS: usize = 256;

/// Generate a Pokémon password with the optional minimum length or word count,
/// the given optional list of separators, or "digit" or "symbol" to use
/// predefined lists of separators, and optional number of digits to append.
#[wasm_bindgen]
pub fn pkpw(len: Option<usize>, count: Option<usize>, separator: Option<String>, append_numbers: Option<usize>) -> String {
    let mut rng = rng();

    generate(
        len.map(|v| v.min(MAX_LEN)),
        count.map(|v| v.min(MAX_COUNT)).unwrap_or(4),
        &(separator.unwrap_or(" ".to_string())),
        append_numbers.map(|v| v.min(MAX_APPEND_NUMBERS)),
        &mut rng,
    )
}

#[cfg(test)]
mod test {
    use crate::{DIGITS, SPECIAL};
    use wasm_bindgen_test::*;

    use crate::wasm::pkpw;

    #[wasm_bindgen_test]
    fn returns_string() {
        assert!(!pkpw(None, None, None, None).is_empty());
    }

    #[wasm_bindgen_test]
    fn accepts_length() {
        let password = pkpw(Some(50), None, None, None);
        let len = password.len();

        assert!(len >= 50);
    }

    #[wasm_bindgen_test]
    fn accepts_count() {
        let password = pkpw(None, Some(6), None, None);
        let words = password.split(' ').count();

        assert!(words >= 6); // some pokémon names have spaces in them
    }

    #[wasm_bindgen_test]
    fn accepts_digit_split() {
        let password = pkpw(None, None, Some("digit".to_string()), None);
        let words = password.split(&DIGITS[..]).count();

        assert!(words >= 3); // Porygon2
    }

    #[wasm_bindgen_test]
    fn accepts_special_split() {
        let password = pkpw(None, None, Some("special".to_string()), None);
        let words = password.split(&SPECIAL[..]).count();

        assert!(words >= 3);
    }

    #[wasm_bindgen_test]
    fn accepts_random_split() {
        let password = pkpw(None, None, Some("random".to_string()), None);
        let mut separators = DIGITS.to_vec();
        separators.extend_from_slice(SPECIAL);
        separators.push(' ');
        let words = password.split(&separators[..]).count();

        assert!(words >= 3);
    }

    #[wasm_bindgen_test]
    fn accepts_chararcter_split() {
        let password = pkpw(None, None, Some("%".to_string()), None);
        let char_count = password.chars().filter(|c| c == &'%').count();

        assert_eq!(3, char_count);
    }

    #[wasm_bindgen_test]
    fn accepts_append_numbers() {
        let password = pkpw(None, None, None, Some(5));
        
        // Check that the last 5 characters are digits
        let chars: Vec<char> = password.chars().collect();
        let last_five = &chars[chars.len() - 5..];
        
        for &ch in last_five {
            assert!(ch.is_ascii_digit());
        }
    }

    #[wasm_bindgen_test]
    fn clamps_large_inputs() {
        use crate::wasm::{MAX_APPEND_NUMBERS, MAX_COUNT, MAX_LEN};

        // Check that a very large length is clamped
        let password = pkpw(Some(usize::MAX), None, None, None);
        // It should be at least MAX_LEN, but not excessively large
        // POKEMON_COUNT is 1028, and each name has some length.
        // If we requested usize::MAX, it would try to take all pokemon and then fail if it still hasn't reached it.
        // With clamping, it should stop much earlier.
        assert!(password.len() >= MAX_LEN);
        // It's hard to assert the exact upper bound because it depends on pokemon name lengths,
        // but it should definitely not be millions of characters.
        assert!(password.len() < MAX_LEN + 100);

        // Check that a very large count is clamped
        let password = pkpw(None, Some(usize::MAX), None, None);
        let words = password.split(' ').count();
        // It should be exactly MAX_COUNT because Pokemon::pick(count) takes `count` items.
        assert_eq!(words, MAX_COUNT);

        // Check that a very large append_numbers is clamped
        let password = pkpw(None, None, None, Some(usize::MAX));
        // The last MAX_APPEND_NUMBERS should be digits
        let chars: Vec<char> = password.chars().collect();
        let last_part = &chars[chars.len() - MAX_APPEND_NUMBERS..];
        for &ch in last_part {
            assert!(ch.is_ascii_digit());
        }
        // And the character before that should NOT be a digit (it should be a space or part of a pokemon name)
        let before_last_part = chars[chars.len() - MAX_APPEND_NUMBERS - 1];
        assert!(!before_last_part.is_ascii_digit() || before_last_part == '2'); // Porygon2 is an exception
    }
}
