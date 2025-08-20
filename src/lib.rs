#![doc = include_str!("../README.md")]

pub mod pokemon;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

use crate::pokemon::Pokemon;
pub use crate::pokemon::POKEMON;
use rand::Rng;

const DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIAL: &[char] = &[
    '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+', '=', '{', '}', '[',
    ']', '|', ':', ';', '<', ',', '>', '.', '?', '/',
];

/// Generate a password matching the given parameters of character length, word
/// count, word separator, and optional appended random digits.
pub fn generate<R: Rng + Clone + ?Sized>(
    len: Option<usize>,
    count: usize,
    separator: &str,
    append_numbers: Option<usize>,
    rng: &mut R,
) -> String {
    let mut rng_local = rng.clone();
    let mut pokemon = Pokemon::new(&mut rng_local);

    let separator_length = match separator {
        "digit" | "special" => 1,
        sep => sep.len(),
    };

    let picked = match len {
        Some(len) => pokemon.length(len, separator_length),
        None => pokemon.pick(count),
    };

    let mut rng_local = rng.clone();
    let password = match separator {
        "digit" => join(picked, DIGITS, &mut rng_local),
        "special" => join(picked, SPECIAL, &mut rng_local),
        sep => picked.join(sep),
    };

    if let Some(num_digits) = append_numbers {
        if num_digits > 0 {
            let mut rng_local = rng.clone();
            let mut numbers = String::new();
            for _ in 0..num_digits {
                let digit = DIGITS[rng_local.random::<u32>() as usize % DIGITS.len()];
                numbers.push(digit);
            }
            format!("{}{}", password, numbers)
        } else {
            password
        }
    } else {
        password
    }
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
        .unwrap_or_else(|| "".to_string())
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
            "Makuhita Milotic Shiftry Charmander".to_string(),
            generate(None, 4, " ", None, &mut rng)
        );
    }

    /// Ensure that generate(Some(length), …) generates a password of a minimum
    /// length.
    #[test]
    fn test_generate_length() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Makuhita Milotic Shiftry Charmander Swadloon".to_string(),
            generate(Some(40), 4, " ", None, &mut rng)
        );
    }

    /// Ensure that generate(…, "-", …) generates a password with "-" as a
    /// separator between words.
    #[test]
    fn test_generate_separator() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Makuhita-Milotic-Shiftry-Charmander".to_string(),
            generate(None, 4, "-", None, &mut rng)
        );
    }

    /// Ensure that generate(…, "digit", …) generates a password with random
    /// digit separators.
    #[test]
    fn test_generate_digit() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Makuhita0Milotic6Shiftry8Charmander".to_string(),
            generate(None, 4, "digit", None, &mut rng)
        );
    }

    /// Ensure that generate(…, "special", …) generates a password with random
    /// special character separators.
    #[test]
    fn test_generate_special() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Makuhita=Milotic;Shiftry]Charmander".to_string(),
            generate(None, 4, "special", None, &mut rng)
        );
    }

    /// Ensure that join() joins the vector of strings with random elements from
    /// the slice of separators.
    #[test]
    fn test_join() {
        let mut rng = rng_from_seed(POKEMON_COUNT);
        let picked = vec!["Lilligant", "Tranquill", "Shelmet", "Mesprit"];

        assert_eq!(
            "Lilligant0Tranquill6Shelmet8Mesprit",
            join(picked, DIGITS, &mut rng)
        );
    }

    /// Ensure that generate(…, …, …, 3, …) appends 3 random digits to the password.
    #[test]
    fn test_generate_append_numbers() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Makuhita Milotic Shiftry Charmander068".to_string(),
            generate(None, 4, " ", Some(3), &mut rng)
        );
    }

    /// Ensure that generate with append_numbers=0 behaves the same as before.
    #[test]
    fn test_generate_append_numbers_zero() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Makuhita Milotic Shiftry Charmander".to_string(),
            generate(None, 4, " ", Some(0), &mut rng)
        );
    }

    /// Ensure that append_numbers works with special separators.
    #[test]
    fn test_generate_append_numbers_with_special() {
        let mut rng = rng_from_seed(POKEMON_COUNT);

        assert_eq!(
            "Makuhita=Milotic;Shiftry]Charmander068".to_string(),
            generate(None, 4, "special", Some(3), &mut rng)
        );
    }
}
