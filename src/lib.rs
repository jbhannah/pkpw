#![doc = include_str!("../README.md")]

pub mod pokemon;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

use crate::pokemon::Pokemon;
pub use crate::pokemon::POKEMON;
use rand::{Rng, RngExt};

const DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIAL: &[char] = &[
    '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+', '=', '{', '}', '[',
    ']', '|', ':', ';', '<', ',', '>', '.', '?', '/',
];

/// Generate a password matching the given parameters of character length, word
/// count, word separator, and optional appended random digits.
pub fn generate<R: Rng>(
    len: Option<usize>,
    count: usize,
    separator: &str,
    append_numbers: Option<usize>,
    rng: &mut R,
) -> String {
    let mut pokemon = Pokemon::new(rng);

    let separator_length = match separator {
        "digit" | "special" | "random" => 1,
        sep => sep.len(),
    };

    let picked = match len {
        Some(len) => pokemon.length(len, separator_length),
        None => pokemon.pick(count),
    };

    let password = match separator {
        "digit" => join(picked, DIGITS, rng),
        "special" => join(picked, SPECIAL, rng),
        "random" => {
            let separators: Vec<char> = []
                .iter()
                .chain(DIGITS.iter())
                .chain(DIGITS.iter())
                .chain(DIGITS.iter())
                .chain(SPECIAL.iter())
                .copied()
                .collect();
            join(picked, &separators, rng)
        }
        sep => picked.join(sep),
    };

    let mut numbers = String::new();

    if let Some(num_digits) = append_numbers {
        for _ in 0..num_digits {
            let i = rng.random::<u32>() as usize % DIGITS.len();
            numbers.push(DIGITS[i]);
        }
    }

    format!("{}{}", password, numbers)
}

/// Join the collection of items with random selections from the set of possible
/// separators.
pub fn join<R: Rng + ?Sized>(picked: Vec<&str>, separators: &[char], rng: &mut R) -> String {
    picked
        .into_iter()
        .map(|name| name.to_owned())
        .reduce(|password, next| {
            let i = rng.random::<u32>() as usize % separators.len();
            format!("{}{}{}", password, separators[i], next)
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod test {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use crate::pokemon::POKEMON_COUNT;

    use super::*;

    fn rng_from_seed(state: usize) -> StdRng {
        StdRng::seed_from_u64(state as u64)
    }

    /// Ensure that generate() generates a password string.
    #[test]
    fn test_generate() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Wugtrio Vanilluxe Piplup Golett".to_string(),
            generate(None, 4, " ", None, &mut rng)
        );
    }

    /// Ensure that generate(Some(length), …) generates a password of a minimum
    /// length.
    #[test]
    fn test_generate_length() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Wugtrio Vanilluxe Piplup Golett Lapras Mr. Rime".to_string(),
            generate(Some(40), 4, " ", None, &mut rng)
        );
    }

    /// Ensure that generate(…, "-", …) generates a password with "-" as a
    /// separator between words.
    #[test]
    fn test_generate_separator() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Wugtrio-Vanilluxe-Piplup-Golett".to_string(),
            generate(None, 4, "-", None, &mut rng)
        );
    }

    /// Ensure that generate(…, "digit", …) generates a password with random
    /// digit separators.
    #[test]
    fn test_generate_digit() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Wugtrio1Vanilluxe3Piplup6Golett".to_string(),
            generate(None, 4, "digit", None, &mut rng)
        );
    }

    /// Ensure that generate(…, "special", …) generates a password with random
    /// special character separators.
    #[test]
    fn test_generate_special() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Wugtrio.Vanilluxe%Piplup/Golett".to_string(),
            generate(None, 4, "special", None, &mut rng)
        );
    }

    /// Ensure that generate(…, "random", …) generates a password with random
    /// separators.
    #[test]
    fn test_generate_random() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Wugtrio8Vanilluxe8Piplup`Golett".to_string(),
            generate(None, 4, "random", None, &mut rng)
        );
    }

    /// Ensure that join() joins the vector of strings with random elements from
    /// the slice of separators.
    #[test]
    fn test_join() {
        let mut rng = rng_from_seed(POKEMON_COUNT);
        let picked = vec!["Lilligant", "Tranquill", "Shelmet", "Mesprit"];

        assert_eq!(
            "Lilligant4Tranquill5Shelmet6Mesprit",
            join(picked, DIGITS, &mut rng)
        );
    }

    /// Ensure that join() returns an empty string when the list of items is empty.
    #[test]
    fn test_join_empty() {
        let mut rng = rng_from_seed(POKEMON_COUNT);
        let picked = vec![];

        assert_eq!("", join(picked, DIGITS, &mut rng));
    }

    /// Ensure that generate(…, …, …, 3, …) appends 3 random digits to the password.
    #[test]
    fn test_generate_append_numbers() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Wugtrio Vanilluxe Piplup Golett136".to_string(),
            generate(None, 4, " ", Some(3), &mut rng)
        );
    }

    /// Ensure that generate with append_numbers=0 behaves the same as before.
    #[test]
    fn test_generate_append_numbers_zero() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Wugtrio Vanilluxe Piplup Golett".to_string(),
            generate(None, 4, " ", Some(0), &mut rng)
        );
    }

    /// Ensure that append_numbers works with special separators.
    #[test]
    fn test_generate_append_numbers_with_special() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Wugtrio.Vanilluxe%Piplup/Golett311".to_string(),
            generate(None, 4, "special", Some(3), &mut rng)
        );
    }
}
