use rand::rng;
use wasm_bindgen::prelude::*;

use crate::generate;

/// Generate a Pokémon password with the optional minimum length or word count,
/// the given optional list of separators, or "digit" or "symbol" to use
/// predefined lists of separators, and optional number of digits to append.
#[wasm_bindgen]
pub fn pkpw(len: Option<usize>, count: Option<usize>, separator: Option<String>, append_numbers: Option<usize>) -> String {
    let mut rng = rng();

    generate(
        len,
        count.unwrap_or(4),
        &(separator.unwrap_or(" ".to_string())),
        append_numbers,
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
}
