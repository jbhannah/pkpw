use rand::rng;
use wasm_bindgen::prelude::*;

use crate::generate;

#[wasm_bindgen]
pub fn pkpw(len: Option<usize>, count: Option<usize>, separator: Option<String>) -> String {
    let mut rng = rng();

    generate(
        len,
        count.unwrap_or(4),
        &(separator.unwrap_or(" ".to_string())),
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
        assert!(!pkpw(None, None, None).is_empty());
    }

    #[wasm_bindgen_test]
    fn accepts_length() {
        let password = pkpw(Some(50), None, None);
        let len = password.len();

        assert!(len >= 50);
    }

    #[wasm_bindgen_test]
    fn accepts_count() {
        let password = pkpw(None, Some(6), None);
        let words = password.split(' ').count();

        assert!(words >= 6); // some pokémon names have spaces in them
    }

    #[wasm_bindgen_test]
    fn accepts_digit_split() {
        let password = pkpw(None, None, Some("digit".to_string()));
        let words = password.split(&DIGITS[..]).count();

        assert!(words >= 3); // Porygon2
    }

    #[wasm_bindgen_test]
    fn accepts_special_split() {
        let password = pkpw(None, None, Some("special".to_string()));
        let words = password.split(&SPECIAL[..]).count();

        assert!(words >= 3);
    }

    #[wasm_bindgen_test]
    fn accepts_chararcter_split() {
        let password = pkpw(None, None, Some("%".to_string()));
        let char_count = password.chars().filter(|c| c == &'%').count();

        assert_eq!(3, char_count);
    }
}
