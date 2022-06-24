#![doc = include_str!("../README.md")]

use std::vec::IntoIter;

use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, Rng};

const DIGITS: &'static [&'static str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const SPECIAL: &'static [&'static str] = &[
    "~", "`", "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "-", "+", "=", "{", "}", "[",
    "]", "|", ":", ";", "<", ",", ">", ".", "?", "/",
];

lazy_static! {
    /// Vector of all Pokémon names, in English and ASCII-normalized (e.g.
    /// Nidoran♀ -> Nidoran-F, Flabébé -> Flabebe) for easier keyboard entry.
    pub static ref POKEMON: Vec<&'static str> =
        include_str!("../pokemon.txt").trim().split("\n").collect();
}

/// Generate a password matching the given parameters of character length, word
/// count, and word separator.
pub fn generate<R: Rng + Clone + ?Sized>(
    len: Option<usize>,
    count: usize,
    separator: &str,
    rng: &mut R,
) -> String {
    let mut rng_local = rng.clone();
    let mut pokemon = shuffle(&mut rng_local);

    let picked = match len {
        Some(len) => length(&mut pokemon, len),
        None => pick(&mut pokemon, count),
    };

    let mut rng_local = rng.clone();
    match separator {
        "digit" => join(picked, DIGITS, &mut rng_local),
        "special" => join(picked, SPECIAL, &mut rng_local),
        sep => picked.join(sep),
    }
}

/// Shuffle the list of Pokémon using the given RNG and return them as an
/// iterator.
pub fn shuffle<R: Rng + ?Sized>(rng: &mut R) -> IntoIter<&str> {
    let mut pokemon = POKEMON.clone();
    pokemon.shuffle(rng);
    pokemon.into_iter()
}

/// Pick a number of random Pokémon names from the given iterator.
pub fn pick<'a>(pokemon: &mut IntoIter<&'a str>, count: usize) -> Vec<&'a str> {
    pokemon.take(count).collect()
}

/// Take Pokémon names from the given iterator until the resulting password
/// would meet the required minimum length.
pub fn length<'a>(pokemon: &mut IntoIter<&'a str>, length: usize) -> Vec<&'a str> {
    let mut picked: Vec<&str> = vec![];

    while picked.join(" ").len() < length {
        picked.push(pokemon.next().expect("no unique names left"));
    }

    picked
}

/// Join the collection of items with random selections from the set of possible
/// separators.
pub fn join<R: Rng + ?Sized>(picked: Vec<&str>, separators: &[&str], rng: &mut R) -> String {
    picked
        .into_iter()
        .map(|name| name.to_owned())
        .reduce(|password, next| {
            let i = rng.gen::<usize>() % separators.len();
            format!("{}{}{}", password, separators[i], next)
        })
        .unwrap_or("".to_string())
}

#[cfg(test)]
mod test {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;

    fn rng_from_seed(state: u64) -> StdRng {
        StdRng::seed_from_u64(state)
    }

    /// Ensure that generate() generates a password string.
    #[test]
    fn test_generate() {
        let mut rng = rng_from_seed(913);

        assert_eq!(
            "Shroomish Venusaur Froakie Tyranitar".to_string(),
            generate(None, 4, " ", &mut rng)
        );
    }

    /// Ensure that generate(Some(length), …) generates a password of a minimum
    /// length.
    #[test]
    fn test_generate_length() {
        let mut rng = rng_from_seed(913);

        assert_eq!(
            "Shroomish Venusaur Froakie Tyranitar Wingull".to_string(),
            generate(Some(40), 4, " ", &mut rng)
        );
    }

    /// Ensure that generate(…, "-", …) generates a password with "-" as a
    /// separator between words.
    #[test]
    fn test_generate_separator() {
        let mut rng = rng_from_seed(913);

        assert_eq!(
            "Shroomish-Venusaur-Froakie-Tyranitar".to_string(),
            generate(None, 4, "-", &mut rng)
        );
    }

    /// Ensure that generate(…, "digit", …) generates a password with random
    /// digit separators.
    #[test]
    fn test_generate_digit() {
        let mut rng = rng_from_seed(913);

        assert_eq!(
            "Shroomish7Venusaur0Froakie2Tyranitar".to_string(),
            generate(None, 4, "digit", &mut rng)
        );
    }

    /// Ensure that generate(…, "special", …) generates a password with random
    /// special character separators.
    #[test]
    fn test_generate_special() {
        let mut rng = rng_from_seed(913);

        assert_eq!(
            "Shroomish#Venusaur`Froakie/Tyranitar".to_string(),
            generate(None, 4, "special", &mut rng)
        );
    }

    /// Ensure that join() joins the vector of strings with random elements from
    /// the slice of separators.
    #[test]
    fn test_join() {
        let mut rng = rng_from_seed(913);
        let mut rng_1 = rng.clone();

        let mut pokemon = shuffle(&mut rng);
        let picked = pick(&mut pokemon, 4);

        assert_eq!(
            "Shroomish7Venusaur0Froakie2Tyranitar",
            join(picked, DIGITS, &mut rng_1)
        );
    }

    /// Ensure that length(…, 40) returns a vector of five strings which create
    /// a password with a length greater than 40.
    #[test]
    fn test_length() {
        let mut rng = rng_from_seed(913);
        let mut pokemon = shuffle(&mut rng);

        assert_eq!(
            vec!["Shroomish", "Venusaur", "Froakie", "Tyranitar", "Wingull"],
            length(&mut pokemon, 40)
        );
    }

    /// Ensure that pick(…, 4) returns a vector of four strings.
    #[test]
    fn test_pick() {
        let mut rng = rng_from_seed(913);
        let mut pokemon = shuffle(&mut rng);

        assert_eq!(
            vec!["Shroomish", "Venusaur", "Froakie", "Tyranitar"],
            pick(&mut pokemon, 4)
        );
    }

    /// Ensure that pick() returns different results for different RNGs.
    #[test]
    fn test_pick_rand() {
        let mut rng_1 = rng_from_seed(913);
        let mut pokemon_1 = shuffle(&mut rng_1);

        let mut rng_2 = rng_from_seed(319);
        let mut pokemon_2 = shuffle(&mut rng_2);

        assert_ne!(pick(&mut pokemon_1, 4), pick(&mut pokemon_2, 4));
    }

    /// Ensure that all Pokémon names are loaded.
    #[test]
    fn test_pokemon() {
        assert_eq!(POKEMON.len(), 913);
    }
}
