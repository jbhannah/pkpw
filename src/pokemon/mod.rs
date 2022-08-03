use std::array::IntoIter;

use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, Rng};

pub const POKEMON_COUNT: usize = 915;

lazy_static! {
    /// Array of all Pokémon names, in English and ASCII-normalized (e.g.
    /// Nidoran♀ -> Nidoran-F, Flabébé -> Flabebe) for easier keyboard entry.
    pub static ref POKEMON: [&'static str; POKEMON_COUNT] =
        include_str!("./pokemon.txt").trim().split('\n').collect::<Vec<&str>>().try_into().unwrap();
}

/// Container for a shuffled array of all Pokémon names.
pub struct Pokemon<'a> {
    inner: [&'a str; POKEMON_COUNT],
    iter: IntoIter<&'a str, POKEMON_COUNT>,
}

impl<'a> Pokemon<'a> {
    /// Initialize a newly-shuffled set of all Pokémon names.
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut pokemon = *POKEMON;
        pokemon.shuffle(rng);

        Self {
            inner: pokemon,
            iter: pokemon.into_iter(),
        }
    }

    /// Return the bare shuffled array of Pokémon names.
    pub fn into_inner(&self) -> [&'a str; POKEMON_COUNT] {
        self.inner
    }

    /// Take names from the shuffled array, until the length of the generated
    /// password with a separator of the given length equals or exceeds the
    /// given minimum length.
    pub fn length(&mut self, length: usize, separator_length: usize) -> Vec<&'a str> {
        let mut picked: Vec<&str> = vec![];
        let sep = vec![" "; separator_length].join("");

        while picked.join(&sep).len() < length {
            picked.push(self.iter.next().expect("no unique names left"));
        }

        picked
    }

    /// Take the given number of Pokémon names from the shuffled array.
    pub fn pick(&mut self, count: usize) -> Vec<&'a str> {
        self.iter.by_ref().take(count).collect()
    }
}

#[cfg(test)]
mod test {
    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    fn from_seed(state: u64) -> Pokemon<'static> {
        let mut rng = StdRng::seed_from_u64(state);
        Pokemon::new(&mut rng)
    }

    /// Ensure that Pokemon::new returns a container around a shuffled array of
    /// Pokémon names.
    #[test]
    fn test_new() {
        let pokemon = from_seed(915);

        assert_eq!(pokemon.inner.len(), POKEMON_COUNT);
        assert_ne!(pokemon.inner, *POKEMON);
    }

    /// Ensure that multiple calls to Pokemon::new return differently-shuffled
    /// arrays of names.
    #[test]
    fn test_new_new() {
        let pokemon_1 = from_seed(915);
        let pokemon_2 = from_seed(319);

        assert_ne!(pokemon_1.inner, pokemon_2.inner);
        assert_ne!(pokemon_1.inner, *POKEMON);
        assert_ne!(pokemon_2.inner, *POKEMON);
    }

    /// Ensure that into_inner() returns the wrapped array of shuffled Pokémon
    /// names.
    #[test]
    fn test_into_inner() {
        let pokemon = from_seed(915);
        let inner = pokemon.into_inner();

        assert_eq!(inner.len(), POKEMON_COUNT);
        assert_eq!(inner, pokemon.inner);
    }

    /// Ensure that length(40, 1) returns a vector of five strings, which create
    /// a password with a length greater than 4 when joined with a separator of
    /// length 1.
    #[test]
    fn test_length() {
        let mut pokemon = from_seed(915);
        let picked = pokemon.length(40, 1);

        assert_eq!(
            vec!["Smoliv", "Trapinch", "Swanna", "Aromatisse", "Charjabug"],
            picked
        );
        assert!(picked.join(" ").len() > 40);
    }

    /// Ensure that pick(4) returns a vector of four strings.
    #[test]
    fn test_pick() {
        let mut pokemon = from_seed(915);

        assert_eq!(
            vec!["Smoliv", "Trapinch", "Swanna", "Aromatisse"],
            pokemon.pick(4)
        );
    }

    /// Ensure that calling pick() multiple times returns a different set of
    /// names.
    #[test]
    fn test_pick_pick() {
        let mut pokemon = from_seed(915);

        assert_eq!(vec!["Smoliv", "Trapinch", "Swanna"], pokemon.pick(3));
        assert_eq!(vec!["Aromatisse", "Charjabug"], pokemon.pick(2));
    }

    /// Ensure that all Pokémon names are loaded.
    #[test]
    fn test_pokemon() {
        assert_eq!(POKEMON.len(), 915);
    }
}
